mod error;
mod store;
mod types;
mod views;

pub use error::Error;
pub use error::Result;

use gloo_storage::Storage;
use leptos::logging::debug_log;
use leptos::prelude::*;
use leptos::task::spawn_local;
use views::{Login, MainPage};

use gloo_storage;

use matrix_sdk::Client;

use types::session::AuthSession;

stylance::import_style!(style, "styles/style.module.scss");

// #[derive(Clone, Debug)]
// #[repr(transparent)]
// pub struct Client;

#[component]
fn App() -> impl IntoView {
    // gloo_storage::LocalStorage::get("session");

    let client_context = LocalResource::new({
        async move || {
            let client_res = Client::builder()
                .indexeddb_store("IndexDBStore", None)
                .homeserver_url("https://matrix.org")
                .build()
                .await;

            debug_log!("Session restart client: {:#?}", client_res);
            if let Ok(client) = client_res {
                let session = gloo_storage::LocalStorage::get::<AuthSession>("auth_session").ok();
                debug_log!("Client Session: {:#?}", session);
                if let Some(session) = session {
                    // gloo_storage::LocalStorage::set("session", session.access_token());
                    let res = client.restore_session(session).await;
                    debug_log!("Client restore: {:#?}", res);
                }
                Some(client)
            } else {
                None
            }
        }
    });

    provide_context(client_context);

    view! {
        { move || match client_context.get().is_some() {
            false => view! {"Loading"}.into_any(),
            true => {
                let context = expect_context::<LocalResource<Option<Client>>>();
                match context.get().flatten() {
                    Some(_) => view! { <MainPage/> }.into_any(),
                    None => view! {<Login/> }.into_any(),
                }
            }
        }

        }

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
