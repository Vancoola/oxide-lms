use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">

                <div class="mb-8">
                    <div class="flex flex-col md:flex-row md:items-center justify-between gap-4">
                        <div>
                            <h1 class="text-3xl font-bold text-gray-900">Мое обучение</h1>
                            <div class="mt-2">
                                <p class="text-xl font-semibold text-gray-700">
                                    "Доброе утро, Иван! 👋"
                                </p>
                                <p class="text-gray-600 mt-1">
                                    <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-green-100 text-green-800">
                                        <span class="w-2 h-2 bg-green-500 rounded-full mr-2"></span>
                                        Сегодня у вас 2 занятия
                                    </span>
                                    <span class="ml-3 inline-flex items-center px-3 py-1 rounded-full text-sm font-medium bg-yellow-100 text-yellow-800">
                                        <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/>
                                        </svg>
                                        Следующий дедлайн через 3 дня
                                    </span>
                                </p>
                            </div>
                        </div>
                        <div class="text-sm text-gray-500">
                            Активная сессия: Весенняя 2024
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-2 space-y-8">

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <div class="flex flex-col md:flex-row md:items-center justify-between gap-4 mb-6">
                                <h2 class="text-xl font-bold text-gray-900">Быстрый доступ</h2>

                                <div class="relative w-full md:w-64">
                                    <input
                                        type="text"
                                        placeholder="Поиск по курсам и материалам..."
                                        class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                                    />
                                    <svg class="absolute left-3 top-2.5 w-5 h-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"/>
                                    </svg>
                                </div>
                            </div>

                            <div class="grid grid-cols-2 sm:grid-cols-4 gap-4">
                                <a href="#" class="flex flex-col items-center justify-center p-4 bg-blue-50 rounded-lg hover:bg-blue-100 transition border border-blue-100">
                                    <div class="w-12 h-12 bg-blue-100 rounded-full flex items-center justify-center mb-3">
                                        <svg class="w-6 h-6 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-gray-900 text-center">Продолжить обучение</span>
                                </a>

                                <a href="#" class="flex flex-col items-center justify-center p-4 bg-green-50 rounded-lg hover:bg-green-100 transition border border-green-100">
                                    <div class="w-12 h-12 bg-green-100 rounded-full flex items-center justify-center mb-3">
                                        <svg class="w-6 h-6 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-gray-900 text-center">Сдать работу</span>
                                </a>

                                <a href="#" class="flex flex-col items-center justify-center p-4 bg-purple-50 rounded-lg hover:bg-purple-100 transition border border-purple-100">
                                    <div class="w-12 h-12 bg-purple-100 rounded-full flex items-center justify-center mb-3">
                                        <svg class="w-6 h-6 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-gray-900 text-center">Задать вопрос</span>
                                </a>

                                <a href="#" class="flex flex-col items-center justify-center p-4 bg-amber-50 rounded-lg hover:bg-amber-100 transition border border-amber-100">
                                    <div class="w-12 h-12 bg-amber-100 rounded-full flex items-center justify-center mb-3">
                                        <svg class="w-6 h-6 text-amber-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 3a1 1 0 00-1 1v5H4a1 1 0 100 2h5v5a1 1 0 102 0v-5h5a1 1 0 100-2h-5V4a1 1 0 00-1-1z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <span class="font-medium text-gray-900 text-center">Записаться на курс</span>
                                </a>
                            </div>
                        </div>

                        <div>
                            <div class="flex items-center justify-between mb-6">
                                <h2 class="text-2xl font-bold text-gray-900">Текущие курсы</h2>
                                <span class="text-sm font-medium text-gray-600">4 курса активны</span>
                            </div>

                            <div class="space-y-6">
                                <div class="bg-white rounded-xl shadow-sm border-2 border-red-200 p-6">
                                    <div class="flex flex-col md:flex-row md:items-start gap-6">
                                        <div class="flex-shrink-0 w-16 h-16 bg-blue-100 rounded-xl flex items-center justify-center">
                                            <span class="text-2xl font-bold text-blue-600">DB</span>
                                        </div>

                                        <div class="flex-1">
                                            <div class="flex flex-col md:flex-row md:items-start md:justify-between gap-4">
                                                <div>
                                                    <h3 class="text-xl font-bold text-gray-900">Базы данных</h3>
                                                    <div class="flex items-center gap-2 mt-1">
                                                        <span class="text-gray-600">Преподаватель:</span>
                                                        <span class="font-medium text-gray-900">Иванов А.П.</span>
                                                        <span class="ml-4 inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-red-100 text-red-800">
                                                            <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z" clip-rule="evenodd"/>
                                                            </svg>
                                                            1 просроченное задание
                                                        </span>
                                                    </div>
                                                </div>

                                                <div class="flex items-center gap-3">
                                                    <div class="text-right">
                                                        <div class="flex items-center gap-2">
                                                            <span class="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-blue-100 text-blue-800">
                                                                <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                                    <path fill-rule="evenodd" d="M18 13V5a2 2 0 00-2-2H4a2 2 0 00-2 2v8a2 2 0 002 2h3l3 3 3-3h3a2 2 0 002-2zM5 7a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1zm1 3a1 1 0 100 2h3a1 1 0 100-2H6z" clip-rule="evenodd"/>
                                                                </svg>
                                                                3 новых
                                                            </span>
                                                            <span class="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-green-100 text-green-800">
                                                                2 непрочитанных
                                                            </span>
                                                        </div>
                                                        <p class="text-sm text-gray-600 mt-1">Ждете оценку за проект</p>
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="mt-6">
                                                <div class="flex justify-between text-sm text-gray-600 mb-2">
                                                    <span>Прогресс курса: 68%</span>
                                                    <span>Завтра тест по модулю 3</span>
                                                </div>
                                                <div class="w-full h-3 bg-gray-200 rounded-full overflow-hidden">
                                                    <div class="flex h-full">
                                                        <div class="bg-blue-500" style="width: 40%"></div>
                                                        <div class="bg-green-500" style="width: 20%"></div>
                                                        <div class="bg-purple-500" style="width: 8%"></div>
                                                    </div>
                                                </div>
                                                <div class="flex justify-between text-xs text-gray-500 mt-2">
                                                    <div class="flex items-center">
                                                        <div class="w-3 h-3 bg-blue-500 rounded mr-1"></div>
                                                        <span>Лекции: 16/20</span>
                                                    </div>
                                                    <div class="flex items-center">
                                                        <div class="w-3 h-3 bg-green-500 rounded mr-1"></div>
                                                        <span>Тесты: 4/5</span>
                                                    </div>
                                                    <div class="flex items-center">
                                                        <div class="w-3 h-3 bg-purple-500 rounded mr-1"></div>
                                                        <span>Практики: 2/3</span>
                                                    </div>
                                                </div>
                                            </div>

                                            <div class="flex flex-wrap gap-3 mt-6 pt-6 border-t border-gray-200">
                                                <a href="#" class="px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition">
                                                    Перейти к лекциям
                                                </a>
                                                <a href="#" class="px-4 py-2 bg-white text-gray-700 text-sm font-medium rounded-lg border border-gray-300 hover:bg-gray-50 transition">
                                                    Открыть задания
                                                </a>
                                                <a href="#" class="px-4 py-2 bg-white text-gray-700 text-sm font-medium rounded-lg border border-gray-300 hover:bg-gray-50 transition">
                                                    Чат курса
                                                </a>
                                                <a href="#" class="px-4 py-2 bg-white text-gray-700 text-sm font-medium rounded-lg border border-gray-300 hover:bg-gray-50 transition">
                                                    Запись к преподавателю
                                                </a>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                                    <div class="flex flex-col md:flex-row md:items-start gap-6">
                                        <div class="flex-shrink-0 w-16 h-16 bg-green-100 rounded-xl flex items-center justify-center">
                                            <span class="text-2xl font-bold text-green-600">WD</span>
                                        </div>

                                        <div class="flex-1">
                                            <div class="flex flex-col md:flex-row md:items-start md:justify-between gap-4">
                                                <div>
                                                    <h3 class="text-xl font-bold text-gray-900">Веб-разработка</h3>
                                                    <div class="flex items-center gap-2 mt-1">
                                                        <span class="text-gray-600">Преподаватель:</span>
                                                        <span class="font-medium text-gray-900">Петрова С.М.</span>
                                                    </div>
                                                </div>

                                                <div class="text-right">
                                                    <div class="inline-flex items-center px-2 py-1 rounded text-xs font-medium bg-blue-100 text-blue-800">
                                                        1 новое задание
                                                    </div>
                                                    <p class="text-sm text-gray-600 mt-1">Новая лекция добавлена</p>
                                                </div>
                                            </div>

                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <div class="flex items-center justify-between mb-6">
                                <h2 class="text-xl font-bold text-gray-900">Недавняя активность</h2>
                                <a href="#" class="text-sm font-medium text-blue-600 hover:text-blue-800">"Вся активность "</a>
                            </div>

                            <div class="space-y-4">
                                <div class="flex items-start gap-4 p-4 bg-blue-50 rounded-lg">
                                    <div class="flex-shrink-0 w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
                                        <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M16.707 5.293a1 1 0 010 1.414l-8 8a1 1 0 01-1.414 0l-4-4a1 1 0 011.414-1.414L8 12.586l7.293-7.293a1 1 0 011.414 0z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="font-medium text-gray-900">Вы сдали задание "Эссе по философии"</p>
                                        <p class="text-sm text-gray-600 mt-1">"Курс: Философия • 2 часа назад"</p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-4 p-4 bg-green-50 rounded-lg">
                                    <div class="flex-shrink-0 w-10 h-10 bg-green-100 rounded-full flex items-center justify-center">
                                        <svg class="w-5 h-5 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M18 13V5a2 2 0 00-2-2H4a2 2 0 00-2 2v8a2 2 0 002 2h3l3 3 3-3h3a2 2 0 002-2zM5 7a1 1 0 011-1h8a1 1 0 110 2H6a1 1 0 01-1-1zm1 3a1 1 0 100 2h3a1 1 0 100-2H6z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="font-medium text-gray-900">Преподаватель Иванов оставил комментарий к вашей работе</p>
                                        <p class="text-sm text-gray-600 mt-1">"Курс: Базы данных • Вчера, 15:30"</p>
                                    </div>
                                </div>

                                <div class="flex items-start gap-4 p-4 bg-purple-50 rounded-lg">
                                    <div class="flex-shrink-0 w-10 h-10 bg-purple-100 rounded-full flex items-center justify-center">
                                        <svg class="w-5 h-5 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M6.267 3.455a3.066 3.066 0 001.745-.723 3.066 3.066 0 013.976 0 3.066 3.066 0 001.745.723 3.066 3.066 0 012.812 2.812c.051.643.304 1.254.723 1.745a3.066 3.066 0 010 3.976 3.066 3.066 0 00-.723 1.745 3.066 3.066 0 01-2.812 2.812 3.066 3.066 0 00-1.745.723 3.066 3.066 0 01-3.976 0 3.066 3.066 0 00-1.745-.723 3.066 3.066 0 01-2.812-2.812 3.066 3.066 0 00-.723-1.745 3.066 3.066 0 010-3.976 3.066 3.066 0 00.723-1.745 3.066 3.066 0 012.812-2.812zm7.44 5.252a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
                                        </svg>
                                    </div>
                                    <div>
                                        <p class="font-medium text-gray-900">Вы получили 95% за тест "Термодинамика"</p>
                                        <p class="text-sm text-gray-600 mt-1">"Курс: Физика • 3 дня назад"</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="space-y-8">

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <div class="flex items-center justify-between mb-6">
                                <h2 class="text-xl font-bold text-gray-900">Что требует внимания?</h2>
                                <span class="text-xs font-medium text-gray-500 bg-gray-100 px-2 py-1 rounded">5 пунктов</span>
                            </div>

                            <div class="space-y-4">
                                <div class="p-4 bg-red-50 border border-red-200 rounded-lg">
                                    <div class="flex items-start justify-between">
                                        <div class="flex items-start gap-3">
                                            <input type="checkbox" class="mt-1 h-4 w-4 text-red-600 rounded focus:ring-red-500"/>
                                            <div>
                                                <p class="font-medium text-gray-900">Эссе по философии</p>
                                                <p class="text-sm text-gray-600 mt-1">"Просрочено на 2 дня • Курс: Философия"</p>
                                            </div>
                                        </div>
                                        <span class="text-xs font-medium text-red-600 bg-red-100 px-2 py-1 rounded">Срочно</span>
                                    </div>
                                    <div class="flex gap-3 mt-3 pt-3 border-t border-red-200">
                                        <button class="text-sm text-blue-600 hover:text-blue-800 font-medium">Открыть задание</button>
                                        <button class="text-sm text-gray-600 hover:text-gray-800 font-medium">Отложить</button>
                                    </div>
                                </div>

                                <div class="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
                                    <div class="flex items-start justify-between">
                                        <div class="flex items-start gap-3">
                                            <input type="checkbox" class="mt-1 h-4 w-4 text-yellow-600 rounded focus:ring-yellow-500"/>
                                            <div>
                                                <p class="font-medium text-gray-900">Тест по модулю 3</p>
                                                <p class="text-sm text-gray-600 mt-1">"Дедлайн: завтра • Курс: Базы данных"</p>
                                            </div>
                                        </div>
                                        <span class="text-xs font-medium text-yellow-600 bg-yellow-100 px-2 py-1 rounded">Завтра</span>
                                    </div>
                                </div>

                                <div class="p-4 bg-gray-50 rounded-lg">
                                    <div class="flex items-start gap-3">
                                        <input type="checkbox" class="mt-1 h-4 w-4 text-gray-600 rounded focus:ring-gray-500"/>
                                        <div>
                                            <p class="font-medium text-gray-900">Просмотреть лекцию 5.2</p>
                                            <p class="text-sm text-gray-600 mt-1">"К занятию в пятницу • Курс: Веб-разработка"</p>
                                        </div>
                                    </div>
                                </div>

                                <button class="w-full py-3 border-2 border-dashed border-gray-300 rounded-lg text-gray-600 hover:text-gray-900 hover:border-gray-400 transition flex items-center justify-center gap-2">
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"/>
                                    </svg>
                                    Добавить свой пункт
                                </button>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Учебная траектория</h2>

                            <div class="space-y-4">
                                <div>
                                    <h3 class="font-semibold text-gray-700 mb-2">1 курс</h3>
                                    <div class="flex flex-wrap gap-2">
                                        <span class="px-3 py-1 bg-green-100 text-green-800 text-sm font-medium rounded-full">"✓ Математика"</span>
                                        <span class="px-3 py-1 bg-green-100 text-green-800 text-sm font-medium rounded-full">"✓ Физика"</span>
                                        <span class="px-3 py-1 bg-green-100 text-green-800 text-sm font-medium rounded-full">"✓ Программирование"</span>
                                    </div>
                                </div>

                                <div>
                                    <h3 class="font-semibold text-gray-700 mb-2">2 курс</h3>
                                    <div class="flex flex-wrap gap-2">
                                        <span class="px-3 py-1 bg-green-100 text-green-800 text-sm font-medium rounded-full">"✓ Алгоритмы"</span>
                                        <span class="px-3 py-1 bg-blue-100 text-blue-800 text-sm font-medium rounded-full">Базы данных</span>
                                        <span class="px-3 py-1 bg-blue-100 text-blue-800 text-sm font-medium rounded-full">ООП</span>
                                    </div>
                                </div>

                                <div>
                                    <h3 class="font-semibold text-gray-700 mb-2">3 курс</h3>
                                    <div class="flex flex-wrap gap-2">
                                        <span class="px-3 py-1 bg-gray-100 text-gray-600 text-sm font-medium rounded-full">Веб-разработка</span>
                                        <span class="px-3 py-1 bg-gray-100 text-gray-600 text-sm font-medium rounded-full">Мобильная разработка</span>
                                    </div>
                                </div>

                                <div class="pt-4 mt-4 border-t border-gray-200">
                                    <div class="flex items-center gap-4 text-sm text-gray-600">
                                        <div class="flex items-center gap-1">
                                            <div class="w-3 h-3 bg-green-500 rounded"></div>
                                            <span>Завершено</span>
                                        </div>
                                        <div class="flex items-center gap-1">
                                            <div class="w-3 h-3 bg-blue-500 rounded"></div>
                                            <span>В процессе</span>
                                        </div>
                                        <div class="flex items-center gap-1">
                                            <div class="w-3 h-3 bg-gray-300 rounded"></div>
                                            <span>Запланировано</span>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Ресурсы</h2>

                            <div class="space-y-3">
                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center gap-3">
                                        <div class="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center">
                                            <svg class="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Расписание</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>

                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center gap-3">
                                        <div class="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center">
                                            <svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Электронная библиотека</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>

                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center gap-3">
                                        <div class="w-8 h-8 bg-purple-100 rounded-lg flex items-center justify-center">
                                            <svg class="w-4 h-4 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Служба поддержки</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="mt-12">
                    <details class="bg-white rounded-xl shadow-sm border border-gray-200">
                        <summary class="px-6 py-4 cursor-pointer list-none">
                            <div class="flex items-center justify-between">
                                <h2 class="text-xl font-bold text-gray-900">Завершенные курсы</h2>
                                <svg class="w-5 h-5 text-gray-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
                                </svg>
                            </div>
                        </summary>

                        <div class="px-6 pb-6 pt-4 border-t border-gray-200">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div class="p-4 border border-gray-200 rounded-lg">
                                    <div class="flex items-center justify-between mb-3">
                                        <h3 class="font-bold text-gray-900">Математический анализ</h3>
                                        <span class="px-3 py-1 bg-green-100 text-green-800 text-sm font-bold rounded-full">A</span>
                                    </div>
                                    <p class="text-gray-600 text-sm mb-4">"Итоговая оценка: 95% • Осень 2023"</p>
                                    <div class="flex gap-3">
                                        <a href="#" class="text-sm text-blue-600 hover:text-blue-800 font-medium">Материалы</a>
                                        <a href="#" class="text-sm text-blue-600 hover:text-blue-800 font-medium">Скачать сертификат</a>
                                    </div>
                                </div>

                            </div>
                        </div>
                    </details>
                </div>
            </div>
        </div>
    }
}