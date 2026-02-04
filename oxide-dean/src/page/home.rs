use leptos::prelude::*;
use leptos_i18n::t;
use oxide_i18n::oxide_i18n::i18n::use_i18n;

#[component]
pub fn Home() -> impl IntoView{

    let i18n = use_i18n();

    view! {
        <div class="min-h-screen p-6">
            <div class="mb-8">
                <div class="flex items-center justify-between mb-4">
                    <div>
                        <h1 class="text-3xl font-bold text-gray-800">Факультет информационных технологий</h1>
                        <p class="text-gray-600 mt-1">{t!(i18n, dean)}: Иванов Иван Иванович</p>
                    </div>
                    <div class="flex items-center space-x-4">
                        <span class="px-4 py-2 bg-blue-100 text-blue-800 rounded-full text-sm font-medium">
                            12" "{t!(i18n, departments)}
                        </span>
                        <span class="px-4 py-2 bg-green-100 text-green-800 rounded-full text-sm font-medium">
                            250" "{t!(i18n, teachers)}
                        </span>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2">
                    <div class="bg-white rounded-xl shadow-sm border border-gray-200">
                        <div class="p-6 border-b border-gray-200">
                            <div class="flex items-center justify-between">
                                <h2 class="text-xl font-semibold text-gray-800">{t!(i18n, departments_faculty)}</h2>
                                <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition flex items-center space-x-2">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                                    </svg>
                                    <span>{t!(i18n, add_department)}</span>
                                </button>
                            </div>
                        </div>

                        <div class="p-6">
                            <div class="space-y-4">
                                <div class="border border-gray-200 rounded-lg overflow-hidden">
                                    <div class="bg-blue-50 p-4 flex items-center justify-between">
                                        <div class="flex items-center space-x-4">
                                            <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                                                <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"/>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3 class="font-semibold text-gray-800">Кафедра программной инженерии</h3>
                                                <p class="text-sm text-gray-600">Зав. кафедрой: Петров Петр Петрович</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center space-x-2">
                                            <button class="p-2 text-gray-600 hover:text-blue-600 hover:bg-blue-50 rounded-lg transition">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                                                </svg>
                                            </button>
                                            <button class="p-2 text-gray-600 hover:text-red-600 hover:bg-red-50 rounded-lg transition">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                                                </svg>
                                            </button>
                                            <button class="p-2 text-gray-600 hover:text-gray-800 hover:bg-gray-100 rounded-lg transition">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"/>
                                                </svg>
                                            </button>
                                        </div>
                                    </div>

                                    <div class="bg-gray-50 p-4">
                                        <div class="ml-10 space-y-3">
                                            <div class="flex items-center justify-between py-2 px-4 bg-white rounded-lg">
                                                <div class="flex items-center space-x-3">
                                                    <div class="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center">
                                                        <span class="text-sm font-medium text-blue-800">СИ</span>
                                                    </div>
                                                    <div>
                                                        <h4 class="font-medium text-gray-800">Сидоров Иван Иванович</h4>
                                                        <p class="text-sm text-gray-600">Профессор, доктор наук</p>
                                                    </div>
                                                </div>
                                                <span class="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">Заведующий</span>
                                            </div>

                                            <div class="flex items-center justify-between py-2 px-4 bg-white rounded-lg">
                                                <div class="flex items-center space-x-3">
                                                    <div class="w-8 h-8 rounded-full bg-purple-100 flex items-center justify-center">
                                                        <span class="text-sm font-medium text-purple-800">КА</span>
                                                    </div>
                                                    <div>
                                                        <h4 class="font-medium text-gray-800">Кузнецов Алексей Сергеевич</h4>
                                                        <p class="text-sm text-gray-600">Доцент, кандидат наук</p>
                                                    </div>
                                                </div>
                                                <span class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">Доцент</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="border border-gray-200 rounded-lg">
                                    <div class="p-4 flex items-center justify-between">
                                        <div class="flex items-center space-x-4">
                                            <div class="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                                                <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4"/>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3 class="font-semibold text-gray-800">Кафедра искусственного интеллекта</h3>
                                                <p class="text-sm text-gray-600">Зав. кафедрой: Смирнова Анна Владимировна</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center space-x-2">
                                        </div>
                                    </div>
                                </div>

                                <div class="border border-gray-200 rounded-lg">
                                    <div class="p-4 flex items-center justify-between">
                                        <div class="flex items-center space-x-4">
                                            <div class="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
                                                <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
                                                </svg>
                                            </div>
                                            <div>
                                                <h3 class="font-semibold text-gray-800">Кафедра информационной безопасности</h3>
                                                <p class="text-sm text-gray-600">Зав. кафедрой: Васильев Дмитрий Олегович</p>
                                            </div>
                                        </div>
                                        <div class="flex items-center space-x-2">
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="space-y-6">
                    <div class="bg-white rounded-xl shadow-sm border border-gray-200">
                        <div class="p-6 border-b border-gray-200">
                            <h2 class="text-xl font-semibold text-gray-800">{t!(i18n, quick_actions)}</h2>
                        </div>
                        <div class="p-6">
                            <div class="grid grid-cols-2 gap-4">
                                <a href="#" class="p-4 bg-blue-50 hover:bg-blue-100 rounded-lg transition group">
                                    <div class="flex flex-col items-center text-center space-y-2">
                                        <div class="w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center group-hover:bg-blue-200 transition">
                                            <svg class="w-6 h-6 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-800">{t!(i18n, schedule)}</span>
                                        <p class="text-sm text-gray-600">{t!(i18n, setting_schedule)}</p>
                                    </div>
                                </a>

                                <a href="#" class="p-4 bg-green-50 hover:bg-green-100 rounded-lg transition group">
                                    <div class="flex flex-col items-center text-center space-y-2">
                                        <div class="w-12 h-12 bg-green-100 rounded-lg flex items-center justify-center group-hover:bg-green-200 transition">
                                            <svg class="w-6 h-6 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 110-4m0 4v2m0-6V4"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-800">{t!(i18n, settings)}</span>
                                        <p class="text-sm text-gray-600">{t!(i18n, faculty_settings)}</p>
                                    </div>
                                </a>

                                <a href="#" class="p-4 bg-purple-50 hover:bg-purple-100 rounded-lg transition group">
                                    <div class="flex flex-col items-center text-center space-y-2">
                                        <div class="w-12 h-12 bg-purple-100 rounded-lg flex items-center justify-center group-hover:bg-purple-200 transition">
                                            <svg class="w-6 h-6 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5 1.195a9 9 0 10-18 0 9 9 0 0018 0z"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-800">{t!(i18n, staff)}</span>
                                        <p class="text-sm text-gray-600">{t!(i18n, hr_management)}</p>
                                    </div>
                                </a>

                                <a href="#" class="p-4 bg-orange-50 hover:bg-orange-100 rounded-lg transition group">
                                    <div class="flex flex-col items-center text-center space-y-2">
                                        <div class="w-12 h-12 bg-orange-100 rounded-lg flex items-center justify-center group-hover:bg-orange-200 transition">
                                            <svg class="w-6 h-6 text-orange-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-800">{t!(i18n, reports)}</span>
                                        <p class="text-sm text-gray-600">{t!(i18n, analytics_and_reports)}</p>
                                    </div>
                                </a>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow-sm border border-gray-200">
                        <div class="p-6 border-b border-gray-200">
                            <h2 class="text-xl font-semibold text-gray-800">{t!(i18n, faculty_statistics)}</h2>
                        </div>
                        <div class="p-6 space-y-4">
                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <div class="w-10 h-10 bg-blue-100 rounded-lg flex items-center justify-center">
                                        <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-sm text-gray-600">{t!(i18n, staff)}</p>
                                        <p class="text-lg font-semibold text-gray-800">250</p>
                                    </div>
                                </div>
                                <span class="px-3 py-1 bg-green-100 text-green-800 rounded-full text-sm">+12 за год</span>
                            </div>

                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <div class="w-10 h-10 bg-green-100 rounded-lg flex items-center justify-center">
                                        <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14l9-5-9-5-9 5 9 5z"/>
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14l9-5-9-5-9 5 9 5z" opacity=".5"/>
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 14v6l9-5M12 20l-9-5"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-sm text-gray-600">{t!(i18n, students)}</p>
                                        <p class="text-lg font-semibold text-gray-800">"2,850"</p>
                                    </div>
                                </div>
                                <span class="px-3 py-1 bg-red-100 text-red-800 rounded-full text-sm">-3% за семестр</span>
                            </div>

                            <div class="flex items-center justify-between">
                                <div class="flex items-center space-x-3">
                                    <div class="w-10 h-10 bg-purple-100 rounded-lg flex items-center justify-center">
                                        <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="text-sm text-gray-600">{t!(i18n, disciplines)}</p>
                                        <p class="text-lg font-semibold text-gray-800">84</p>
                                    </div>
                                </div>
                                <span class="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm">+5 новых</span>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow-sm border border-gray-200">
                        <div class="p-6 border-b border-gray-200">
                            <h2 class="text-xl font-semibold text-gray-800">{t!(i18n, recent_changes)}</h2>
                        </div>
                        <div class="p-6 space-y-4">
                            <div class="flex items-start space-x-3">
                                <div class="w-8 h-8 rounded-full bg-blue-100 flex items-center justify-center flex-shrink-0">
                                    <svg class="w-4 h-4 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
                                    </svg>
                                </div>
                                <div>
                                    <p class="text-sm text-gray-800">Добавлена новая кафедра "Киберфизических систем"</p>
                                    <p class="text-xs text-gray-500 mt-1">2 часа назад</p>
                                </div>
                            </div>

                            <div class="flex items-start space-x-3">
                                <div class="w-8 h-8 rounded-full bg-green-100 flex items-center justify-center flex-shrink-0">
                                    <svg class="w-4 h-4 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
                                    </svg>
                                </div>
                                <div>
                                    <p class="text-sm text-gray-800">Обновлено расписание весеннего семестра</p>
                                    <p class="text-xs text-gray-500 mt-1">Вчера, 14:30</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}