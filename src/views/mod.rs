use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos::{logging::debug_log, tachys::view};

use matrix_sdk::ruma::room::RoomType;
use matrix_sdk::{
    Client,
    media::MediaThumbnailSettings,
    ruma::{UserId, events::room::name, user_id},
};

use crate::ClientContext;

stylance::import_style!(style, "../styles/style.module.scss");

#[component]
pub fn side_bar() -> impl IntoView {
    let client_context = expect_context::<RwSignal<Option<ClientContext>, LocalStorage>>();
    let client = client_context.get().unwrap().0;
    debug_log!("{}", client.rooms().len());

    let rooms = client
        .rooms()
        .into_iter()
        .filter(|r| r.create_content().and_then(|c| c.room_type) == Some(RoomType::Space))
        .map(|r| {
            let avatar = r.avatar_url();
            let name = r
                .cached_display_name()
                .unwrap_or(matrix_sdk::RoomDisplayName::Empty);

            view! {
                <li class=style::side_bar_list>
                    {
                        match avatar {
                            Some(url) => view! {
                                <img src=url.to_string() alt="a"/>
                            }.into_any(),

                            None => {
                                match name {
                                    matrix_sdk::RoomDisplayName::Named(name) => {
                                        view! { {name} }.into_any()
                                    }
                                    _ => view! { "" }.into_any(),
                                }
                            }
                        }
                    }
                </li>
            }
        })
        .collect_view();

    view! {
        <div class=style::main_side_bar>
            <ul>
                {rooms}
            </ul>
        </div>

    }
}
#[component]
pub fn login() -> impl IntoView {
    let user = RwSignal::new("".to_owned());
    let password = RwSignal::new("".to_owned());

    let client_context = expect_context::<RwSignal<Option<ClientContext>, LocalStorage>>();

    let onclick = async move || {
        let user_string = format!("@{}:{}", user.get(), "matrix.org");

        let user = <&UserId>::try_from(user_string.as_str()).unwrap();

        let client_res = Client::builder()
            .server_name(user.server_name())
            .build()
            .await;

        if let Ok(client) = client_res {
            client
                .matrix_auth()
                .login_username(user, &password.get())
                .send()
                .await
                .unwrap();

            client
                .sync_once(matrix_sdk::config::SyncSettings::default())
                .await
                .unwrap();

            client_context.set(Some(ClientContext(client)));
        }
    };

    view! {
        <div>
            <input bind:value=user></input>
            <input type="password" bind:value=password></input>
            <button on:click=move |_| {
                    spawn_local(async move {onclick().await;
                });
            }>Button</button>
        </div>
    }
}
