use leptos::prelude::mount_to_body;
use leptos::view;
use crate::router::AppRouter;

mod router;
mod page;
mod component;

fn main() {
    mount_to_body(move || view! { <AppRouter/> });
}
