use leptos::prelude::*;

#[component]
pub fn Home() -> impl IntoView{
    view! {
        <div class="min-h-screen p-6">
            <div class="mb-8">
                <div class="flex justify-between items-center mb-6">
                    <h1 class="text-3xl font-bold text-gray-800">Факультет Информатики</h1>
                    <div class="flex gap-4">
                        <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition">
                            "📊 Экспорт отчетов"
                        </button>
                        <button class="px-4 py-2 border border-blue-600 text-blue-600 rounded-lg hover:bg-blue-50 transition">
                            "⚙️ Настройки"
                        </button>
                    </div>
                </div>

                <div class="bg-white rounded-xl shadow p-6 mb-6">
                    <h2 class="text-lg font-semibold text-gray-700 mb-4">Состояние факультета на сегодня</h2>
                    <div class="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
                        <div class="text-center p-3 bg-blue-50 rounded-lg">
                            <div class="text-2xl font-bold text-blue-700">1,247</div>
                            <div class="text-sm text-gray-600">"👥 Студенты"</div>
                        </div>
                        <div class="text-center p-3 bg-green-50 rounded-lg">
                            <div class="text-2xl font-bold text-green-700">89</div>
                            <div class="text-sm text-gray-600">"📚 Активных курсов"</div>
                        </div>
                        <div class="text-center p-3 bg-purple-50 rounded-lg">
                            <div class="text-2xl font-bold text-purple-700">~210</div>
                            <div class="text-sm text-gray-600">"🎓 Выпуск в этом семестре"</div>
                        </div>
                        <div class="text-center p-3 bg-yellow-50 rounded-lg">
                            <div class="text-2xl font-bold text-yellow-700">3.8</div>
                            <div class="text-sm text-gray-600">"📈 Средний GPA"</div>
                        </div>
                        <div class="text-center p-3 bg-red-50 rounded-lg">
                            <div class="text-2xl font-bold text-red-700">34</div>
                            <div class="text-sm text-gray-600">"⚠️ Академ. задолженности"</div>
                        </div>
                        <div class="text-center p-3 bg-orange-50 rounded-lg">
                            <div class="text-2xl font-bold text-orange-700">12</div>
                            <div class="text-sm text-gray-600">"📅 Ближайшие защиты"</div>
                        </div>
                    </div>
                </div>

                <div class="space-y-3">
                    <div class="bg-red-50 border-l-4 border-red-500 p-4 rounded">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <span class="text-red-500">"⚠️"</span>
                            </div>
                            <div class="ml-3">
                                <p class="text-sm text-red-700">
                                    <span class="font-medium">Завтра комиссия по отчислению</span>" — подготовьте документы по 5 студентам"
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="bg-yellow-50 border-l-4 border-yellow-500 p-4 rounded">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <span class="text-yellow-500">"📝"</span>
                            </div>
                            <div class="ml-3">
                                <p class="text-sm text-yellow-700">
                                    <span class="font-medium">Требуется утвердить учебный план на следующий год</span>" — срок до 15.12.2024"
                                </p>
                            </div>
                        </div>
                    </div>
                    <div class="bg-orange-50 border-l-4 border-orange-500 p-4 rounded">
                        <div class="flex items-center">
                            <div class="flex-shrink-0">
                                <span class="text-orange-500">"👨‍🏫"</span>
                            </div>
                            <div class="ml-3">
                                <p class="text-sm text-orange-700">
                                    <span class="font-medium">3 курса не имеют назначенных преподавателей</span>" — срочно назначьте ответственных"
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                <div class="lg:col-span-2 space-y-6">
                    <div class="bg-white rounded-xl shadow p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold text-gray-800">Структура факультета</h2>
                            <button class="text-blue-600 hover:text-blue-800 font-medium">"⚙️ Редактировать"</button>
                        </div>

                        <div class="font-mono text-sm bg-gray-50 p-4 rounded-lg border">
                            <div class="flex items-center justify-between mb-2">
                                <div class="font-bold">ФАКУЛЬТЕТ ИНФОРМАТИКИ</div>
                                <button class="text-xs text-blue-600">"⚙️"</button>
                            </div>
                            <div class="ml-4">
                                <div class="flex items-center justify-between mb-1">
                                    <div>"├── 🎓 Кафедра Программной инженерии"</div>
                                    <div class="text-xs text-gray-500">"👤 Зав. кафедрой: Петров И.И."</div>
                                </div>
                                <div class="ml-8">
                                    <div class="flex items-center justify-between mb-1">
                                        <div>"├── Группа ПИ-21-1 (24 студента)"</div>
                                        <div class="text-xs text-green-600">Успеваемость: 4.2</div>
                                    </div>
                                    <div class="flex items-center justify-between mb-1">
                                        <div>"├── Группа ПИ-21-2 (22 студента)"</div>
                                        <div class="text-xs text-yellow-600">Успеваемость: 3.9</div>
                                    </div>
                                    <div class="text-blue-600 cursor-pointer">
                                        "└── [+] Добавить группу"
                                    </div>
                                </div>
                                <div class="flex items-center justify-between mb-1">
                                    <div>"├── 🎓 Кафедра Искусственного интеллекта"</div>
                                    <button class="text-xs text-blue-600">Назначить зав. кафедрой</button>
                                </div>
                                <div class="text-blue-600 cursor-pointer">
                                    "└── [+] Добавить кафедру"
                                </div>
                            </div>
                        </div>

                        <div class="mt-4 flex gap-3">
                            <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 text-sm">
                                Назначить зав. кафедрой
                            </button>
                            <button class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 text-sm">
                                Объединить группы
                            </button>
                            <button class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 text-sm">
                                Создать новую специальность
                            </button>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow p-6">
                        <div class="flex justify-between items-center mb-6">
                            <h2 class="text-xl font-semibold text-gray-800">Люди и роли</h2>
                            <div class="flex gap-2">
                                <button class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 text-sm">
                                    [+] Пригласить преподавателя
                                </button>
                                <button class="px-4 py-2 bg-gray-100 text-gray-700 rounded-lg hover:bg-gray-200 text-sm">
                                    "📧 Массовая рассылка"
                                </button>
                            </div>
                        </div>

                        <div class="mb-6">
                            <h3 class="font-semibold text-gray-700 mb-3">Преподаватели</h3>
                            <div class="space-y-3">
                                <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg border">
                                    <div class="flex items-center gap-4">
                                        <div class="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
                                            "👤"
                                        </div>
                                        <div>
                                            <div class="font-medium">Иванов А.А. (доцент)</div>
                                            <div class="text-sm text-gray-500">Курсы: 4 | Стаж: 12 лет</div>
                                        </div>
                                    </div>
                                    <div class="flex gap-2">
                                        <button class="px-3 py-1 bg-blue-100 text-blue-700 rounded text-sm hover:bg-blue-200">
                                            Назначить курс
                                        </button>
                                        <button class="px-3 py-1 bg-gray-100 text-gray-700 rounded text-sm hover:bg-gray-200">
                                            Изменить нагрузку
                                        </button>
                                    </div>
                                </div>
                                <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg border">
                                    <div class="flex items-center gap-4">
                                        <div class="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
                                            "👤"
                                        </div>
                                        <div>
                                            <div class="font-medium">Петрова С.И. (ассистент)</div>
                                            <div class="text-sm text-gray-500">Курсы: 1 | Статус: <span class="text-yellow-600">В отпуске</span></div>
                                        </div>
                                    </div>
                                    <div class="flex gap-2">
                                        <button class="px-3 py-1 bg-orange-100 text-orange-700 rounded text-sm hover:bg-orange-200">
                                            Заменить
                                        </button>
                                        <button class="px-3 py-1 bg-green-100 text-green-700 rounded text-sm hover:bg-green-200">
                                            Отправить на повышение
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div>
                            <div class="flex justify-between items-center mb-3">
                                <h3 class="font-semibold text-gray-700">Студенты</h3>
                                <div class="flex gap-2">
                                    <select class="px-3 py-1 border rounded text-sm">
                                        <option>По курсу</option>
                                        <option>По успеваемости</option>
                                        <option>С академ. задолженностями</option>
                                    </select>
                                    <button class="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700">
                                        Массовые действия
                                    </button>
                                </div>
                            </div>
                            <div class="flex gap-2 mt-4">
                                <button class="px-4 py-2 bg-blue-50 text-blue-700 rounded-lg hover:bg-blue-100 text-sm">
                                    Перевести на следующий курс
                                </button>
                                <button class="px-4 py-2 bg-green-50 text-green-700 rounded-lg hover:bg-green-100 text-sm">
                                    Назначить стипендию
                                </button>
                                <button class="px-4 py-2 bg-purple-50 text-purple-700 rounded-lg hover:bg-purple-100 text-sm">
                                    Сформировать приказ
                                </button>
                            </div>
                        </div>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="bg-white rounded-xl shadow p-6">
                            <h2 class="text-xl font-semibold text-gray-800 mb-4">Учебные планы</h2>
                            <div class="space-y-4">
                                <div class="p-4 bg-blue-50 rounded-lg">
                                    <div class="font-semibold">Программа Программная инженерия</div>
                                    <div class="text-sm text-gray-600">бакалавриат, 4 года <span class="text-green-600">"📄 Утвержден 12.09.2023"</span></div>
                                    <div class="mt-2 text-sm">
                                        <div>1 курс: 10 дисциплин | 60 кредитов</div>
                                        <div>2 курс: 12 дисциплин | 62 кредита</div>
                                    </div>
                                    <div class="mt-3 flex gap-2">
                                        <button class="px-3 py-1 bg-blue-600 text-white rounded text-sm hover:bg-blue-700">
                                            Сравнить с гос. стандартом
                                        </button>
                                        <button class="px-3 py-1 border border-blue-600 text-blue-600 rounded text-sm hover:bg-blue-50">
                                            Создать версию на 2025-2026
                                        </button>
                                    </div>
                                </div>
                                <div class="text-center">
                                    <button class="text-blue-600 hover:text-blue-800 font-medium">
                                        "📋 Визуальный конструктор учебных планов →"
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow p-6">
                            <h2 class="text-xl font-semibold text-gray-800 mb-4">Аналитика и отчетность</h2>
                            <div class="space-y-4">
                                <div class="p-3 bg-gray-50 rounded">
                                    <div class="font-medium text-sm mb-2">Динамика успеваемости</div>
                                    <div class="h-2 bg-gray-200 rounded-full overflow-hidden">
                                        <div class="h-full bg-gradient-to-r from-green-400 to-blue-500 w-3/4"></div>
                                    </div>
                                    <div class="text-xs text-gray-500 mt-1">Средний GPA по кафедрам за 5 лет</div>
                                </div>
                                <div class="p-3 bg-red-50 rounded">
                                    <div class="font-medium text-sm mb-2">Нагрузка преподавателей</div>
                                    <div class="flex items-center gap-2">
                                        <div class="w-16 h-16 rounded-full border-4 border-red-500 border-t-transparent"></div>
                                        <div class="text-sm">
                                            <div>45% перегружены</div>
                                            <div class="text-red-600 font-medium">Рекомендация: Переназначить 2 курса</div>
                                        </div>
                                    </div>
                                </div>
                                <div class="p-3 bg-green-50 rounded">
                                    <div class="font-medium text-sm">Будущее факультета</div>
                                    <div class="text-xs mt-1">Через 2 года ожидается рост набора на 15%</div>
                                    <div class="text-xs">Требуется 3 новых преподавателя по машинному обучению</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div class="space-y-6">
                    <div class="bg-white rounded-xl shadow p-6">
                        <h2 class="text-xl font-semibold text-gray-800 mb-4">Лента событий</h2>
                        <div class="space-y-4">
                            <div class="border-l-4 border-blue-500 pl-4">
                                <div class="text-sm text-gray-500">14:30</div>
                                <div class="font-medium">Защита ВКР группы ПИ-21-1</div>
                                <div class="text-xs text-gray-500">Аудитория 304, 8 студентов</div>
                            </div>
                            <div class="border-l-4 border-red-500 pl-4">
                                <div class="text-sm text-gray-500">Сегодня</div>
                                <div class="font-medium">Последний день сдачи долгов по математике</div>
                                <div class="text-xs text-gray-500">34 студента имеют задолженности</div>
                            </div>
                            <div class="border-l-4 border-orange-500 pl-4">
                                <div class="text-sm text-gray-500">Завтра</div>
                                <div class="font-medium">Визит аккредитационной комиссии</div>
                                <div class="text-xs text-gray-500">Подготовьте отчеты и документы</div>
                            </div>
                            <div class="border-l-4 border-green-500 pl-4">
                                <div class="text-sm text-gray-500">Новый</div>
                                <div class="font-medium">Отзыв о курсе Базы данных</div>
                                <div class="text-xs text-gray-500">4.7/5.0 от 24 студентов</div>
                            </div>
                        </div>
                    </div>

                    <div class="bg-yellow-50 rounded-xl shadow p-6 border border-yellow-200">
                        <h2 class="text-xl font-semibold text-gray-800 mb-4">Напоминания</h2>
                        <ul class="space-y-3">
                            <li class="flex items-start">
                                <span class="text-yellow-500 mr-2">"•"</span>
                                <span>Не утверждены темы ВКР для 4 курса</span>
                            </li>
                            <li class="flex items-start">
                                <span class="text-yellow-500 mr-2">"•"</span>
                                <span>Требуется обновить положение о стипендиях</span>
                            </li>
                        </ul>
                        <button class="mt-4 w-full py-2 bg-yellow-100 text-yellow-800 rounded-lg hover:bg-yellow-200 font-medium">
                            Показать все (12)
                        </button>
                    </div>

                    <div class="bg-white rounded-xl shadow p-6">
                        <h2 class="text-xl font-semibold text-gray-800 mb-4">Документооборот</h2>
                        <div class="space-y-4">
                            <div class="text-center p-4 border-2 border-dashed border-gray-300 rounded-lg">
                                <div class="text-3xl mb-2">"📝"</div>
                                <div class="font-medium mb-2">Создать приказ</div>
                                <div class="text-sm text-gray-600 mb-4">Быстрая генерация документов</div>
                                <button class="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 w-full">
                                    Начать создание
                                </button>
                            </div>
                            <div class="grid grid-cols-2 gap-2">
                                <button class="p-3 bg-gray-50 rounded-lg hover:bg-gray-100 text-sm">
                                    О зачислении
                                </button>
                                <button class="p-3 bg-gray-50 rounded-lg hover:bg-gray-100 text-sm">
                                    О переводе
                                </button>
                                <button class="p-3 bg-gray-50 rounded-lg hover:bg-gray-100 text-sm">
                                    Об отчислении
                                </button>
                                <button class="p-3 bg-gray-50 rounded-lg hover:bg-gray-100 text-sm">
                                    О назначении
                                </button>
                            </div>
                        </div>
                    </div>

                    <div class="bg-white rounded-xl shadow p-6">
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-semibold text-gray-800">Расписание</h2>
                            <button class="text-blue-600 hover:text-blue-800 text-sm">"Весь день →"</button>
                        </div>
                        <div class="space-y-3">
                            <div class="p-3 bg-red-50 rounded">
                                <div class="font-medium text-sm text-red-700">"⚠️ Конфликты расписания"</div>
                                <div class="text-xs text-red-600">2 аудитории заняты одновременно</div>
                            </div>
                            <div class="text-sm">
                                <div class="flex justify-between py-2 border-b">
                                    <span>09:00-10:30</span>
                                    <span class="font-medium">Базы данных</span>
                                    <span class="text-gray-500">ауд. 304</span>
                                </div>
                                <div class="flex justify-between py-2 border-b">
                                    <span>10:45-12:15</span>
                                    <span class="font-medium">Алгоритмы</span>
                                    <span class="text-gray-500">ауд. 215</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}