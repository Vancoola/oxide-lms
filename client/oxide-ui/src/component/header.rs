use leptos::children::ChildrenFn;
use leptos::{component, view, IntoView};
use leptos::prelude::{Get, Show};
use leptos_router::hooks::use_location;

#[component]
pub fn ConditionalHeader(children: ChildrenFn) -> impl IntoView {
    let location = use_location();

    view! {
        <Show when=move || location.pathname.get() != "/login">
            {children()}
        </Show>
    }
}