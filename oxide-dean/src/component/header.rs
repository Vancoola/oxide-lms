use leptos::prelude::{component, view, IntoView, ElementChild, ClassAttribute, OnAttribute, Get, signal, Update, use_context};
use leptos_i18n::t;
use oxide_i18n::oxide_i18n::i18n::use_i18n;
use oxide_web_common::auth::AuthContext;

#[component]
pub fn Header() -> impl IntoView {

    let i18n = use_i18n();

    let (is_hidden_menu, set_hidden_menu) = signal(true);

    let auth = use_context::<AuthContext>().expect("UserContext должен быть представлен AuthProvider");

    view! {
        <header class="bg-white shadow-sm border-b border-gray-300">
            <div class="container mx-auto px-4 py-4">
                <div class="flex items-center justify-between">

                    <div class="flex items-center">
                        <div class="text-2xl font-bold bg-gradient-to-r from-blue-700 to-blue-500 bg-clip-text text-transparent">
                            {t!(i18n, brand_title)}
                        </div>
                    </div>

                    <div class="hidden md:flex items-center space-x-6">
                        <a href="/" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            {t!(i18n, home)}
                        </a>
                        <a href="/schedule" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            {t!(i18n, schedule)}
                        </a>
                        <a href="/staff" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            {t!(i18n, staff)}
                        </a>
                        <a href="/faculty" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            {t!(i18n, settings)}
                        </a>
                        <a href="/students" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
                            {t!(i18n, students)}
                        </a>
                    </div>

                    <div class="flex items-center space-x-4">
                        <div class="flex items-center space-x-3">
                            <div class="relative">
                                <div class="w-10 h-10 bg-gradient-to-r from-blue-600 to-blue-400 rounded-full flex items-center justify-center text-white font-semibold text-sm">
                                    {move || auth.user.get().first_name.chars().next()}{move || auth.user.get().last_name.chars().next()}
                                </div>
                                <div class="absolute bottom-0 right-0 w-3 h-3 bg-green-500 rounded-full border-2 border-white"></div>
                            </div>

                            <div class="hidden md:block">
                                <div class="text-sm font-medium text-gray-900">
                                    {move || auth.user.get().first_name}" "{move || auth.user.get().last_name}
                                </div>
                                <div class="text-xs text-gray-500">
                                    {t!(i18n, group)}": "{move || auth.user.get().group_code}
                                </div>
                            </div>

                            <div class="relative">
                                <button on:click=move |_| set_hidden_menu.update(|h| *h = !*h) class="text-gray-700 hover:text-gray-900">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                    </svg>
                                </button>

                                <div class:hidden=is_hidden_menu class="absolute right-0 mt-2 w-48 bg-white rounded-md shadow-lg py-1 border border-gray-200">
                                    <a href="/profile" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100">
                                        Профиль
                                    </a>
                                    <a href="/settings" class="block px-4 py-2 text-sm text-gray-700 hover:bg-gray-100">
                                        Настройки
                                    </a>
                                    <div class="border-t border-gray-200 my-1"></div>
                                    <button class="w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-gray-100">
                                        Выйти
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="md:hidden flex items-center">
                        <button id="mobile-menu-button" class="text-gray-700">
                            <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
                            </svg>
                        </button>
                    </div>
                </div>

                <div id="mobile-menu" class="md:hidden hidden mt-4 pb-2">
                    <div class="flex flex-col space-y-3">
                        <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
                            Главная
                        </a>
                        <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
                            Факультеты
                        </a>
                        <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
                            Расписание
                        </a>
                        <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
                            Новости
                        </a>
                        <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
                            Контакты
                        </a>
                    </div>
                </div>
            </div>
    </header>
    }
}

// #[component]
// fn Header() -> impl IntoView {
//
//     let navigate = use_navigate();
//
//     view! {
//         <header class="bg-white shadow-sm border-b border-gray-300">
//             <div class="container mx-auto px-4 py-4">
//                 <div class="flex items-center justify-between">
//
//                     <div class="flex items-center">
//                         <div class="text-2xl font-bold bg-gradient-to-r from-blue-700 to-blue-500 bg-clip-text text-transparent">
//                             Университет
//                         </div>
//                     </div>
//
//                     <div class="hidden md:flex items-center space-x-6">
//                         <a href="/" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
//                             Главная
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
//                             Факультеты
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
//                             Расписание
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
//                             Новости
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium">
//                             Контакты
//                         </a>
//                     </div>
//
//                     <div class="flex items-center space-x-4">
//                         <button on:click=move |_| navigate("/login", Default::default()) class="bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors font-medium">
//                             Войти
//                         </button>
//                     </div>
//
//                     <div class="md:hidden flex items-center">
//                         <button id="mobile-menu-button" class="text-gray-700">
//                             <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
//                                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"/>
//                             </svg>
//                         </button>
//                     </div>
//                 </div>
//
//                 <div id="mobile-menu" class="md:hidden hidden mt-4 pb-2">
//                     <div class="flex flex-col space-y-3">
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
//                             Главная
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
//                             Факультеты
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
//                             Расписание
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
//                             Новости
//                         </a>
//                         <a href="#" class="text-gray-700 hover:text-blue-600 transition-colors font-medium py-1">
//                             Контакты
//                         </a>
//                     </div>
//                 </div>
//             </div>
//     </header>
//     }
// }