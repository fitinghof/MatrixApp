use gloo_storage::Storage;
use leptos::logging::debug_log;
use leptos::task::spawn_local;
use leptos::{ev, prelude::*};

use leptos::html::{Style, div};
use matrix_sdk::ruma::room::RoomType;
use matrix_sdk::ruma::user_id;
use matrix_sdk::{Client, ruma::UserId};

use crate::Error;
use crate::Result;

stylance::import_style!(style, "../styles/style.module.scss");

#[component]
pub fn main_page() -> impl IntoView {
    let client_context = expect_context::<LocalResource<Option<Client>>>();
    let client = client_context
        .get()
        .flatten()
        .expect("Invalid Client in main_page");

    // let (room_state, set_state) = signal(vec![]);

    // provide_context(room_state);

    spawn_local({
        // let client = client.clone();
        // let state_clone = set_state.clone();

        async move {
            client
                .sync_once(matrix_sdk::config::SyncSettings::default())
                .await
                .unwrap();

            // state_clone.set(client.rooms());
            debug_log!("MainPage Async done");
        }
    });
    debug_log!("MainPage");

    div().child(SideBar())
}

#[component]
pub fn space_container() -> impl IntoView {
    view! {
        <SpaceSidebar/>
    }
}
#[component]
pub fn space_sidebar() -> impl IntoView {}

#[component]
pub fn side_bar() -> impl IntoView {
    debug_log!("Sidebar");
    let client_context = expect_context::<LocalResource<Option<Client>>>();

    let rooms = move || {
        let client = client_context
            .get()
            .flatten()
            .expect("Client should have ben initilized if this window is open");
        debug_log!("{}", client.rooms().len());

        client
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
                                        _ => view! { "-" }.into_any(),
                                    }
                                }
                            }
                        }
                    </li>
                }
            })
            .collect_view()
    };

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

    let loading = RwSignal::new(false);
    let client_context = expect_context::<LocalResource<Option<Client>>>();

    let onclick = {
        async move || {
            loading.set(true);

            // do your Matrix login
            let user_string = format!("@{}:matrix.org", user.get());
            let user_id = <&UserId>::try_from(user_string.as_str()).unwrap();

            let client_res = Client::builder()
                .indexeddb_store("IndexDBStore", None)
                .server_name(user_id.server_name())
                .build()
                .await;

            if let Ok(client) = client_res {
                let _ = client
                    .matrix_auth()
                    .login_username(user_id, &password.get())
                    .initial_device_display_name("SuperDuperAwesomeMatrixApp")
                    .send()
                    .await;

                debug_log!("{:?}", client.session());
                gloo_storage::LocalStorage::set(
                    "auth_session",
                    crate::AuthSession::from(client.session().expect("Where it go?")),
                )
                .unwrap();
                client_context.set(Some(Some(client)));
            }

            loading.set(false);
        }
    };

    view! {
        <div>
            {move || if loading.get() {
                "loading".into_any()
            } else {
                view! {
                    <input bind:value=user></input>
                    <input type="password" bind:value=password></input>
                    <button on:click=move |_| {spawn_local(async move {onclick().await;})}>Button</button>
                }.into_any()
            }}
        </div>
    }
}
