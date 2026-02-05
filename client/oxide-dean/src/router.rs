use leptos::prelude::{component, ClassAttribute, IntoView};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use oxide_i18n::oxide_i18n::i18n::I18nContextProvider;
use oxide_ui::component::header::ConditionalHeader;
use oxide_ui::page::login::Login;
use oxide_ui::page::not_found::NotFound;
use oxide_web_common::auth::AuthProvider;
use crate::component::header::Header;
use crate::page::faculty::Faculty;
use crate::page::home::Home;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <div>
            <Router>
                <AuthProvider>
                    <I18nContextProvider>
                        <main>
                            <ConditionalHeader><Header/></ConditionalHeader>
                            <div class="container mx-auto px-4">
                                <Routes fallback=NotFound>
                                    <Route path=path!("/login") view=Login />
                                    <Route path=path!("/faculty") view=Faculty />
                                    <Route path=path!("/") view=Home />
                                </Routes>
                            </div>
                        </main>
                    </I18nContextProvider>
                </AuthProvider>
            </Router>
        </div>
    }
}