use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use crate::component::header::{ConditionalHeader};
use crate::page::dean_online::DeanOnline;
use crate::page::home::Home;
use crate::page::login::Login;
use crate::page::programs::Programs;
use crate::page::not_found::NotFound;
use crate::page::student::profile::Profile;

#[component]
pub fn AppRouter() -> impl IntoView{
    view! {
        <div id="root">
            <Router>
                <main>
                    <ConditionalHeader/>
                    <div class="container mx-auto px-4">
                        <Routes fallback=NotFound>
                            <Route path=path!("") view=Home />
                            <Route path=path!("/programs") view=Programs />
                            <Route path=path!("/login") view=Login />
                            <Route path=path!("/profile") view=Profile />
                            <Route path=path!("/dean-online") view=DeanOnline />
                        </Routes>
                    </div>
                </main>
            </Router>
        </div>
    }
}

// #[component]
// pub fn SimpleCounter(initial_value: i32) -> impl IntoView {
//     // create a reactive signal with the initial value
//     let (value, set_value) = signal(initial_value);
//
//     // create event handlers for our buttons
//     // note that `value` and `set_value` are `Copy`, so it's super easy to move them into closures
//     let clear = move |_| set_value.set(0);
//     let decrement = move |_| *set_value.write() -= 1;
//     let increment = move |_| *set_value.write() += 1;
//
//     view! {
//         <div>
//             <button on:click=clear>"Clear"</button>
//             <button on:click=decrement>"-1"</button>
//             <span>"Value: " {value} "!"</span>
//             <button on:click=increment>"+1"</button>
//         </div>
//     }
// }