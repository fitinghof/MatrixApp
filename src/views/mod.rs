use leptos::prelude::*;

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
    let (stage, set_stage) = signal(0);

    view! {
        <div>
            <input></input>
        </div>
    }
}
