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

use gloo_storage;

use matrix_sdk::Client;

use serde::Deserialize;
use serde::Serialize;

stylance::import_style!(style, "styles/style.module.scss");

// #[derive(Clone, Debug)]
// #[repr(transparent)]
// pub struct Client;

#[derive(Serialize, Deserialize, Debug)]
pub struct OAuthSessionWrapper {
    pub client_id: matrix_sdk::authentication::oauth::ClientId,
    pub user_session: matrix_sdk::authentication::oauth::UserSession,
}

impl From<matrix_sdk::authentication::oauth::OAuthSession> for OAuthSessionWrapper {
    fn from(value: matrix_sdk::authentication::oauth::OAuthSession) -> Self {
        Self {
            client_id: value.client_id,
            user_session: value.user,
        }
    }
}

impl Into<matrix_sdk::authentication::oauth::OAuthSession> for OAuthSessionWrapper {
    fn into(self) -> matrix_sdk::authentication::oauth::OAuthSession {
        matrix_sdk::authentication::oauth::OAuthSession {
            client_id: self.client_id,
            user: self.user_session,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AuthSession {
    MatrixSession(matrix_sdk::authentication::matrix::MatrixSession),
    OAuthSession(OAuthSessionWrapper),
}

impl Into<matrix_sdk::AuthSession> for AuthSession {
    fn into(self) -> matrix_sdk::AuthSession {
        match self {
            Self::MatrixSession(v) => matrix_sdk::AuthSession::Matrix(v),
            Self::OAuthSession(v) => matrix_sdk::AuthSession::OAuth(Box::new(v.into())),
        }
    }
}

impl From<matrix_sdk::AuthSession> for AuthSession {
    fn from(value: matrix_sdk::AuthSession) -> Self {
        match value {
            matrix_sdk::AuthSession::Matrix(v) => AuthSession::MatrixSession(v),
            matrix_sdk::AuthSession::OAuth(v) => {
                AuthSession::OAuthSession(OAuthSessionWrapper::from(*v))
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

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

    spawn_local(async move {
        client_context.await;
    });

    view! {
        <Show
         when=move ||
            client_context.get().is_some_and(|client| client.is_some_and(|c| c.is_active()))

            fallback=|| view! {<views::Login/>}
         >
            <views::MainPage/>
         </Show>

    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App)
}
