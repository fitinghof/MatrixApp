mod views;
use leptos::prelude::*;
use leptos::task::spawn_local;
use matrix_sdk::{
    Client,
    config::SyncSettings,
    ruma::{events::room::message::SyncRoomMessageEvent, user_id},
};

stylance::import_style!(style, "styles/style.module.scss");

#[component]
fn App() -> impl IntoView {
    let user = user_id!("@sibbeeegold:matrix.org");

    let test = LocalResource::new(async move || {
        Client::builder()
            .server_name(user.server_name())
            .build()
            .await
    });

    let (some, set_some) = signal(0);

    view! {
        <div>
            {move || {
                match async_data.read() {
                    Some(client) => view! { <p>"Client ready!"</p> }.into_view(),
                    None => view! { <p>"Loading client..."</p> }.into_view(),
                }
            }}
        </div>
        <views::SideBar/>
    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
