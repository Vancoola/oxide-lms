use leptos::prelude::mount_to_body;
use leptos::view;
use crate::router::AppRouter;

mod router;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! {<AppRouter/>})
}
