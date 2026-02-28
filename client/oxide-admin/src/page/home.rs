use leptos::prelude::*;
use crate::component::account_list::AccountList;
use crate::component::header::Header;
use crate::component::quick_action::QuickAction;
use crate::component::sidebar::Sidebar;
use crate::component::stats::Stats;

#[component()]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="text-gray-800 antialiased h-screen overflow-hidden flex">
            <Sidebar/>
            <div class="flex-1 flex flex-col h-full overflow-hidden bg-gray-bg relative">
                <Header/>
                <div class="flex-1 overflow-y-auto px-8 pb-8 custom-scrollbar">
                    <Stats/>
                    <QuickAction/>
                    <AccountList/>
                </div>
            </div>
        </div>
    }
}