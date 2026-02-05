use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use oxide_i18n::oxide_i18n::i18n::I18nContextProvider;
use oxide_ui::component::header::ConditionalHeader;
use oxide_ui::page::login::Login;
use oxide_ui::page::not_found::NotFound;
use oxide_web_common::auth::AuthProvider;
use crate::component::header::{Header};
use crate::page::dean_online::DeanOnline;
use crate::page::home::Home;
use crate::page::programs::Programs;
use crate::page::student::profile::Profile;

#[component]
pub fn AppRouter() -> impl IntoView {
    view! {
        <div id="root">
            <Router>
                <AuthProvider>
                    <I18nContextProvider>
                        <main>
                            <ConditionalHeader><Header/></ConditionalHeader>
                            <div class="container mx-auto px-4">
                                <Routes fallback=NotFound>
                                    <Route path=path!("/") view=Home />
                                    <Route path=path!("/programs") view=Programs />
                                    <Route path=path!("/login") view=Login />
                                    <Route path=path!("/profile") view=Profile />
                                    <Route path=path!("/dean-online") view=DeanOnline />
                                </Routes>
                            </div>
                        </main>
                    </I18nContextProvider>
                </AuthProvider>
            </Router>
        </div>
    }
}

