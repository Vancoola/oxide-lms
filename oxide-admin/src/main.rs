use leptos::prelude::mount_to_body;
use leptos::view;
use crate::router::AppRouter;

mod router;

fn main() {
    mount_to_body(|| view! {<AppRouter/>})
}
