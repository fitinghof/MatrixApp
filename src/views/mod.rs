use leptos::prelude::*;
use leptos::task::spawn_local;

use matrix_sdk::{
    Client,
    ruma::{UserId, user_id},
};

use crate::ClientContext;

stylance::import_style!(style, "../styles/style.module.scss");

#[component]
pub fn side_bar() -> impl IntoView {
    view! {
        <div class=style::main_side_bar>
            a
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
