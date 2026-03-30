mod views;
use leptos::prelude::*;
use matrix_sdk::ruma::user_id;

stylance::import_style!(style, "styles/style.module.scss");

#[derive(Clone, Debug)]
#[repr(transparent)]
pub struct ClientContext(matrix_sdk::Client);

#[component]
fn App() -> impl IntoView {
    let client: RwSignal<Option<ClientContext>, LocalStorage> = RwSignal::new_local(None);

    provide_context(client);

    view! {
        <Show
         when=move || client.get().is_none()
         >
            <views::Login/>
         </Show>
        <Show
         when=move || client.get().is_some()
         >
            <views::SideBar/>
         </Show>

    }
}

fn main() {
    leptos::mount::mount_to_body(App)
}
