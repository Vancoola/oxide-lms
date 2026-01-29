use leptos::mount::mount_to_body;
use leptos::view;
use crate::router::OxideRouter;

mod router;
mod component;
mod page;

fn main() {
    mount_to_body(|| view! {<OxideRouter />})
}
