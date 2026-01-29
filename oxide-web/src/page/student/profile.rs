use leptos::prelude::*;

#[component]
pub fn Profile() -> impl IntoView{
    view! {
        <div class="min-h-screen py-8 px-4 sm:px-6 lg:px-8">
            <div class="max-w-7xl mx-auto mb-8">
                <h1 class="text-3xl font-bold text-gray-900">Профиль студента</h1>
                <p class="text-gray-600 mt-2">Личная информация и академические данные</p>
            </div>

            <div class="max-w-7xl mx-auto">
                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-2 space-y-8">
                        <div class="bg-white rounded-lg shadow-md p-6">
                            <h2 class="text-xl font-semibold text-gray-800 mb-6">Основная информация</h2>
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">ФИО</p>
                                    <p class="text-lg font-medium text-gray-900">Иванов Иван Иванович</p>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Группа</p>
                                    <p class="text-lg font-medium text-gray-900">ИСП-204</p>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Факультет</p>
                                    <p class="text-lg font-medium text-gray-900">Информационных технологий</p>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Специальность</p>
                                    <p class="text-lg font-medium text-gray-900">Программная инженерия</p>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Курс</p>
                                    <div class="flex items-center">
                                        <p class="text-lg font-medium text-gray-900 mr-2">4 курс</p>
                                        <span class="bg-blue-100 text-blue-800 text-xs font-medium px-2.5 py-0.5 rounded">из 4</span>
                                    </div>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Год поступления</p>
                                    <p class="text-lg font-medium text-gray-900">2022</p>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-lg shadow-md p-6">
                            <h2 class="text-xl font-semibold text-gray-800 mb-6">Академическая успеваемость</h2>

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                <div class="space-y-4">
                                    <div class="flex justify-between items-center">
                                        <div>
                                            <p class="text-sm text-gray-500">Средний балл (GPA)</p>
                                            <p class="text-3xl font-bold text-gray-900">4.2</p>
                                        </div>
                                        <div class="w-16 h-16 relative">
                                            <svg class="w-full h-full" viewBox="0 0 36 36">
                                                <path d="M18 2.0845
                                                    a 15.9155 15.9155 0 0 1 0 31.831
                                                    a 15.9155 15.9155 0 0 1 0 -31.831"
                                                    fill="none"
                                                    stroke="#E5E7EB"
                                                    stroke-width="3"/>
                                                <path d="M18 2.0845
                                                    a 15.9155 15.9155 0 0 1 0 31.831
                                                    a 15.9155 15.9155 0 0 1 0 -31.831"
                                                    fill="none"
                                                    stroke="#3B82F6"
                                                    stroke-width="3"
                                                    stroke-dasharray="84, 100"/>
                                            </svg>
                                            <span class="absolute inset-0 flex items-center justify-center text-sm font-semibold text-gray-900">84%</span>
                                        </div>
                                    </div>
                                    <p class="text-sm text-gray-600">Из максимальных 5.0</p>
                                </div>

                                <div class="space-y-4">
                                    <div>
                                        <p class="text-sm text-gray-500">Прогресс по программе</p>
                                        <p class="text-lg font-medium text-gray-900">Выполнено 210 из 240 кредитов</p>
                                    </div>
                                    <div class="space-y-2">
                                        <div class="flex justify-between text-sm">
                                            <span class="text-gray-600">Прогресс</span>
                                            <span class="font-medium text-gray-900">87.5%</span>
                                        </div>
                                        <div class="w-full bg-gray-200 rounded-full h-2.5">
                                            <div class="bg-green-500 h-2.5 rounded-full" style="width: 87.5%"></div>
                                        </div>
                                    </div>
                                    <p class="text-sm text-gray-600">Осталось 30 кредитов до выпуска</p>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-lg shadow-md p-6">
                            <div class="flex justify-between items-center mb-6">
                                <h2 class="text-xl font-semibold text-gray-800">Ближайшие дедлайны</h2>
                                <a href="#" class="text-sm text-blue-600 hover:text-blue-800 font-medium">Все дедлайны</a>
                            </div>

                            <div class="space-y-4">
                                <div class="flex items-center justify-between p-4 bg-blue-50 rounded-lg border border-blue-100">
                                    <div class="flex items-center space-x-4">
                                        <div class="flex-shrink-0 w-12 h-12 bg-blue-100 rounded-lg flex items-center justify-center">
                                            <span class="text-blue-800 font-semibold">15</span>
                                        </div>
                                        <div>
                                            <p class="font-medium text-gray-900">Курсовая работа</p>
                                            <p class="text-sm text-gray-600">Предмет: Базы данных</p>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm text-gray-500">До сдачи</p>
                                        <p class="font-medium text-red-600">3 дня</p>
                                    </div>
                                </div>

                                <div class="flex items-center justify-between p-4 bg-yellow-50 rounded-lg border border-yellow-100">
                                    <div class="flex items-center space-x-4">
                                        <div class="flex-shrink-0 w-12 h-12 bg-yellow-100 rounded-lg flex items-center justify-center">
                                            <span class="text-yellow-800 font-semibold">22</span>
                                        </div>
                                        <div>
                                            <p class="font-medium text-gray-900">Экзамен</p>
                                            <p class="text-sm text-gray-600">Предмет: Алгоритмы и структуры данных</p>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm text-gray-500">До сдачи</p>
                                        <p class="font-medium text-amber-600">10 дней</p>
                                    </div>
                                </div>

                                <div class="flex items-center justify-between p-4 bg-gray-50 rounded-lg border border-gray-100">
                                    <div class="flex items-center space-x-4">
                                        <div class="flex-shrink-0 w-12 h-12 bg-gray-100 rounded-lg flex items-center justify-center">
                                            <span class="text-gray-800 font-semibold">30</span>
                                        </div>
                                        <div>
                                            <p class="font-medium text-gray-900">Лабораторная работа</p>
                                            <p class="text-sm text-gray-600">Предмет: Веб-разработка</p>
                                        </div>
                                    </div>
                                    <div class="text-right">
                                        <p class="text-sm text-gray-500">До сдачи</p>
                                        <p class="font-medium text-gray-600">18 дней</p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="space-y-8">
                        <div class="bg-white rounded-lg shadow-md p-6">
                            <h2 class="text-xl font-semibold text-gray-800 mb-6">Академический статус</h2>

                            <div class="space-y-4">
                                <div class="flex items-center justify-between">
                                    <div class="space-y-1">
                                        <p class="text-sm text-gray-500">Статус обучения</p>
                                        <div class="flex items-center">
                                            <div class="w-3 h-3 bg-green-500 rounded-full mr-2"></div>
                                            <p class="text-lg font-medium text-gray-900">Учится</p>
                                        </div>
                                    </div>
                                    <span class="bg-green-100 text-green-800 text-sm font-medium px-3 py-1 rounded-full">Активный</span>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Форма обучения</p>
                                    <p class="text-lg font-medium text-gray-900">Очная</p>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Финансирование</p>
                                    <p class="text-lg font-medium text-gray-900">Бюджет</p>
                                </div>

                                <div class="space-y-1">
                                    <p class="text-sm text-gray-500">Следующая сессия</p>
                                    <p class="text-lg font-medium text-gray-900">Зимняя 2024</p>
                                </div>
                            </div>

                            <div class="mt-6 pt-6 border-t border-gray-200 space-y-3">
                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-600">Посещаемость</span>
                                    <span class="font-medium text-gray-900">95%</span>
                                </div>
                                <div class="w-full bg-gray-200 rounded-full h-2">
                                    <div class="bg-green-500 h-2 rounded-full" style="width: 95%"></div>
                                </div>

                                <div class="flex items-center justify-between">
                                    <span class="text-sm text-gray-600">Сдано экзаменов</span>
                                    <span class="font-medium text-gray-900">8 из 10</span>
                                </div>
                                <div class="w-full bg-gray-200 rounded-full h-2">
                                    <div class="bg-blue-500 h-2 rounded-full" style="width: 80%"></div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-lg shadow-md p-6">
                            <h2 class="text-xl font-semibold text-gray-800 mb-6">Быстрые действия</h2>

                            <div class="space-y-3">
                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center">
                                        <div class="w-8 h-8 bg-blue-100 rounded-lg flex items-center justify-center mr-3">
                                            <svg class="w-4 h-4 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Расписание занятий</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>

                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center">
                                        <div class="w-8 h-8 bg-green-100 rounded-lg flex items-center justify-center mr-3">
                                            <svg class="w-4 h-4 text-green-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-11a1 1 0 10-2 0v2H7a1 1 0 100 2h2v2a1 1 0 102 0v-2h2a1 1 0 100-2h-2V7z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Записаться на курс</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>

                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center">
                                        <div class="w-8 h-8 bg-purple-100 rounded-lg flex items-center justify-center mr-3">
                                            <svg class="w-4 h-4 text-purple-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Справки и документы</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>

                                <a href="#" class="flex items-center justify-between p-3 hover:bg-gray-50 rounded-lg transition">
                                    <div class="flex items-center">
                                        <div class="w-8 h-8 bg-red-100 rounded-lg flex items-center justify-center mr-3">
                                            <svg class="w-4 h-4 text-red-600" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clip-rule="evenodd"/>
                                            </svg>
                                        </div>
                                        <span class="font-medium text-gray-900">Выписка об оценках</span>
                                    </div>
                                    <svg class="w-5 h-5 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
                                        <path fill-rule="evenodd" d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z" clip-rule="evenodd"/>
                                    </svg>
                                </a>
                            </div>
                        </div>

                    </div>
                </div>
            </div>
        </div>
    }
}