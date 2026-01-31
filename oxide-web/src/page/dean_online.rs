use leptos::prelude::*;

#[component]
pub fn DeanOnline() -> impl IntoView{
    view! {
        <div class="min-h-screen">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">

                <div class="mb-8">
                    <h1 class="text-3xl font-bold text-gray-900">Онлайн-деканат</h1>
                    <p class="text-gray-600 mt-2">Управление документами, заявками и академическими процедурами</p>
                </div>

                <div class="grid grid-cols-1 lg:grid-cols-3 gap-8">
                    <div class="lg:col-span-2 space-y-8">

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <div class="flex items-center justify-between mb-6">
                                <h2 class="text-xl font-bold text-gray-900">Мои текущие заявки</h2>
                                <button class="px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition">
                                    Новая заявка
                                </button>
                            </div>

                            <div class="grid grid-cols-3 gap-4 mb-6">
                                <div class="bg-blue-50 p-4 rounded-lg text-center">
                                    <div class="text-2xl font-bold text-blue-600">3</div>
                                    <div class="text-sm text-blue-800">На рассмотрении</div>
                                </div>
                                <div class="bg-green-50 p-4 rounded-lg text-center">
                                    <div class="text-2xl font-bold text-green-600">12</div>
                                    <div class="text-sm text-green-800">Исполнено</div>
                                </div>
                                <div class="bg-red-50 p-4 rounded-lg text-center">
                                    <div class="text-2xl font-bold text-red-600">1</div>
                                    <div class="text-sm text-red-800">Отклонено</div>
                                </div>
                            </div>

                            <div class="overflow-x-auto">
                                <table class="w-full text-sm text-left text-gray-600">
                                    <thead class="text-xs text-gray-700 uppercase bg-gray-50">
                                        <tr>
                                            <th class="px-4 py-3">Тип заявки</th>
                                            <th class="px-4 py-3">Дата подачи</th>
                                            <th class="px-4 py-3">Статус</th>
                                            <th class="px-4 py-3">Действия</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        <tr class="bg-white border-b hover:bg-gray-50">
                                            <td class="px-4 py-3 font-medium text-gray-900">Справка об обучении</td>
                                            <td class="px-4 py-3">15.03.2026</td>
                                            <td class="px-4 py-3">
                                                <span class="px-3 py-1 bg-blue-100 text-blue-800 text-xs font-medium rounded-full">В обработке</span>
                                            </td>
                                            <td class="px-4 py-3">
                                                <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">Подробнее</button>
                                            </td>
                                        </tr>
                                        <tr class="bg-white border-b hover:bg-gray-50">
                                            <td class="px-4 py-3 font-medium text-gray-900">Академический отпуск</td>
                                            <td class="px-4 py-3">10.03.2026</td>
                                            <td class="px-4 py-3">
                                                <span class="px-3 py-1 bg-yellow-100 text-yellow-800 text-xs font-medium rounded-full">На согласовании</span>
                                            </td>
                                            <td class="px-4 py-3">
                                                <button class="text-blue-600 hover:text-blue-800 text-sm font-medium">Подробнее</button>
                                            </td>
                                        </tr>
                                    </tbody>
                                </table>
                            </div>

                            <div class="mt-8 pt-6 border-t border-gray-200">
                                <h3 class="font-semibold text-gray-900 mb-4">Быстрые заявки</h3>
                                <div class="grid grid-cols-2 md:grid-cols-4 gap-3">
                                    <button class="p-3 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition text-sm font-medium">
                                        Заказать справку
                                    </button>
                                    <button class="p-3 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition text-sm font-medium">
                                        Академ. отпуск
                                    </button>
                                    <button class="p-3 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition text-sm font-medium">
                                        Восстановление
                                    </button>
                                    <button class="p-3 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition text-sm font-medium">
                                        Изменить тему ВКР
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Справки и документы</h2>

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div class="space-y-4">
                                    <div class="p-4 border border-gray-200 rounded-lg hover:border-blue-300 transition">
                                        <div class="flex justify-between items-start">
                                            <div>
                                                <h3 class="font-bold text-gray-900">Справка об обучении</h3>
                                                <p class="text-sm text-gray-600 mt-1">Стандартная форма для предоставления по месту требования</p>
                                            </div>
                                            <span class="text-sm font-medium text-green-600">Бесплатно</span>
                                        </div>
                                        <div class="flex items-center text-sm text-gray-500 mt-3">
                                            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/>
                                            </svg>
                                            Срок изготовления: 2 рабочих дня
                                        </div>
                                        <button class="mt-3 px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition">
                                            Заказать
                                        </button>
                                    </div>

                                    <div class="p-4 border border-gray-200 rounded-lg hover:border-blue-300 transition">
                                        <div class="flex justify-between items-start">
                                            <div>
                                                <h3 class="font-bold text-gray-900">Справка для военкомата</h3>
                                                <p class="text-sm text-gray-600 mt-1">По форме установленного образца</p>
                                            </div>
                                            <span class="text-sm font-medium text-green-600">Бесплатно</span>
                                        </div>
                                        <div class="flex items-center text-sm text-gray-500 mt-3">
                                            <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/>
                                            </svg>
                                            Срок изготовления: 3 рабочих дня
                                        </div>
                                        <button class="mt-3 px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition">
                                            Заказать
                                        </button>
                                    </div>
                                </div>

                                <div class="p-4 bg-gray-50 rounded-lg">
                                    <h3 class="font-bold text-gray-900 mb-4">Настройки документа</h3>

                                    <div class="space-y-4">
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-2">Формат документа</label>
                                            <select class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500">
                                                <option>"PDF (стандартный)"</option>
                                                <option>PDF для визы</option>
                                                <option>"DOCX (редактируемый)"</option>
                                            </select>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-2">Язык документа</label>
                                            <div class="flex gap-3">
                                                <label class="inline-flex items-center">
                                                    <input type="radio" name="language" checked class="h-4 w-4 text-blue-600"/>
                                                    <span class="ml-2 text-gray-700">Русский</span>
                                                </label>
                                                <label class="inline-flex items-center">
                                                    <input type="radio" name="language" class="h-4 w-4 text-blue-600"/>
                                                    <span class="ml-2 text-gray-700">Английский</span>
                                                </label>
                                            </div>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-2">Дополнительно</label>
                                            <div class="space-y-2">
                                                <label class="flex items-center">
                                                    <input type="checkbox" class="h-4 w-4 text-blue-600 rounded"/>
                                                    <span class="ml-2 text-sm text-gray-700">"Добавить ЭЦП (электронную подпись)"</span>
                                                </label>
                                                <label class="flex items-center">
                                                    <input type="checkbox" class="h-4 w-4 text-blue-600 rounded"/>
                                                    <span class="ml-2 text-sm text-gray-700">Отправить копию на email</span>
                                                </label>
                                            </div>
                                        </div>

                                        <div class="pt-4 border-t border-gray-200">
                                            <div class="flex justify-between items-center text-sm">
                                                <span class="text-gray-600">Итоговая стоимость:</span>
                                                <span class="font-bold text-gray-900">"0 ₽"</span>
                                            </div>
                                            <button class="w-full mt-3 px-4 py-3 bg-green-600 text-white font-medium rounded-lg hover:bg-green-700 transition">
                                                Оформить и оплатить
                                            </button>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Академические процедуры</h2>

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                <div class="p-4 border border-gray-200 rounded-lg">
                                    <h3 class="font-bold text-gray-900 mb-3">Академический отпуск</h3>
                                    <p class="text-sm text-gray-600 mb-4">Подача заявления на отпуск по состоянию здоровья, семейным обстоятельствам или иным причинам</p>

                                    <div class="space-y-3">
                                        <select class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm">
                                            <option>Выберите тип отпуска</option>
                                            <option>По состоянию здоровья</option>
                                            <option>По уходу за ребенком</option>
                                            <option>По семейным обстоятельствам</option>
                                            <option>По иным уважительным причинам</option>
                                        </select>

                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-1">Прикрепить документы</label>
                                            <div class="border-2 border-dashed border-gray-300 rounded-lg p-4 text-center">
                                                <svg class="mx-auto h-12 w-12 text-gray-400" stroke="currentColor" fill="none" viewBox="0 0 48 48">
                                                    <path d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
                                                </svg>
                                                <p class="text-sm text-gray-600">Перетащите файлы или нажмите для загрузки</p>
                                            </div>
                                        </div>

                                        <button class="w-full px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition">
                                            Подать заявление
                                        </button>
                                    </div>
                                </div>

                                <div class="p-4 border border-gray-200 rounded-lg">
                                    <h3 class="font-bold text-gray-900 mb-3">Тема ВКР</h3>
                                    <p class="text-sm text-gray-600 mb-4">Утверждение или изменение темы выпускной квалификационной работы</p>

                                    <div class="space-y-3">
                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-1">Текущая тема</label>
                                            <input type="text" value="Разработка веб-приложения для университета" readonly class="w-full px-3 py-2 bg-gray-50 border border-gray-300 rounded-lg text-sm"/>
                                        </div>

                                        <div>
                                            <label class="block text-sm font-medium text-gray-700 mb-1">Предложить новую тему</label>
                                            <textarea rows="3" class="w-full px-3 py-2 border border-gray-300 rounded-lg text-sm" placeholder="Введите предложение..."></textarea>
                                        </div>

                                        <div class="text-sm">
                                            <div class="flex items-center justify-between">
                                                <span class="text-gray-600">Статус утверждения:</span>
                                                <span class="px-2 py-1 bg-green-100 text-green-800 text-xs font-medium rounded-full">Утверждено</span>
                                            </div>
                                            <div class="mt-2 text-gray-600">
                                                Научный руководитель: <span class="font-medium">Иванов А.П.</span>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Финансы и стипендии</h2>

                            <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                                <div>
                                    <h3 class="font-bold text-gray-900 mb-4">Финансовый статус</h3>
                                    <div class="bg-green-50 border border-green-200 rounded-lg p-4">
                                        <div class="text-center">
                                            <div class="text-2xl font-bold text-green-600 mb-2">"+15 240 ₽"</div>
                                            <div class="text-sm text-green-700">Переплата за текущий семестр</div>
                                        </div>
                                    </div>

                                    <div class="mt-4 space-y-3">
                                        <div class="flex justify-between text-sm">
                                            <span class="text-gray-600">Стоимость семестра:</span>
                                            <span class="font-medium">"85 000 ₽"</span>
                                        </div>
                                        <div class="flex justify-between text-sm">
                                            <span class="text-gray-600">Оплачено:</span>
                                            <span class="font-medium text-green-600">"100 240 ₽"</span>
                                        </div>
                                        <div class="flex justify-between text-sm">
                                            <span class="text-gray-600">Доп. услуги:</span>
                                            <span class="font-medium">"2 500 ₽"</span>
                                        </div>
                                    </div>

                                    <button class="w-full mt-4 px-4 py-3 bg-blue-600 text-white font-medium rounded-lg hover:bg-blue-700 transition">
                                        Оплатить онлайн
                                    </button>
                                </div>

                                <div>
                                    <h3 class="font-bold text-gray-900 mb-4">Стипендия</h3>
                                    <div class="bg-blue-50 border border-blue-200 rounded-lg p-4 mb-4">
                                        <div class="flex items-center justify-between">
                                            <div>
                                                <div class="text-lg font-bold text-blue-900">"5 200 ₽"</div>
                                                <div class="text-sm text-blue-700">Академическая стипендия</div>
                                            </div>
                                            <span class="px-3 py-1 bg-green-100 text-green-800 text-sm font-medium rounded-full">Активна</span>
                                        </div>
                                    </div>

                                    <div class="space-y-3">
                                        <div>
                                            <div class="flex justify-between text-sm">
                                                <span class="text-gray-600">Следующая выплата:</span>
                                                <span class="font-medium">25.03.2026</span>
                                            </div>
                                            <div class="flex justify-between text-sm">
                                                <span class="text-gray-600">Ваш рейтинг в группе:</span>
                                                <span class="font-medium">3 из 25</span>
                                            </div>
                                        </div>

                                        <div class="pt-3 border-t border-gray-200">
                                            <div class="text-sm text-gray-600 mb-2">Для повышенной стипендии нужно:</div>
                                            <div class="flex items-center justify-between text-sm">
                                                <span>Текущий средний балл:</span>
                                                <span class="font-bold text-blue-600">4.8</span>
                                            </div>
                                            <div class="flex items-center justify-between text-sm">
                                                <span>Пороговое значение:</span>
                                                <span class="font-medium">4.5</span>
                                            </div>
                                            <div class="mt-2">
                                                <div class="w-full bg-gray-200 rounded-full h-2">
                                                    <div class="bg-green-500 h-2 rounded-full" style="width: 95%"></div>
                                                </div>
                                            </div>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="space-y-8">

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Расписание и аттестации</h2>

                            <div class="space-y-4">
                                <div class="p-4 bg-red-50 border border-red-200 rounded-lg">
                                    <div class="flex items-center justify-between mb-2">
                                        <h3 class="font-bold text-gray-900">Экзамен</h3>
                                        <span class="text-sm font-medium text-red-600">Через 3 дня</span>
                                    </div>
                                    <p class="text-sm text-gray-700">Базы данных</p>
                                    <div class="flex items-center text-sm text-gray-600 mt-2">
                                        <svg class="w-4 h-4 mr-1" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/>
                                        </svg>
                                        "22 марта, 10:00 • Ауд. 304"
                                    </div>
                                </div>

                                <div class="p-4 border border-gray-200 rounded-lg">
                                    <h3 class="font-bold text-gray-900 mb-3">Электронная зачетка</h3>
                                    <div class="space-y-2 text-sm">
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">Текущий семестр:</span>
                                            <span class="font-medium">4.2 GPA</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">Зачетов:</span>
                                            <span class="font-medium text-green-600">5/5</span>
                                        </div>
                                        <div class="flex justify-between">
                                            <span class="text-gray-600">Экзаменов:</span>
                                            <span class="font-medium">2/4</span>
                                        </div>
                                    </div>
                                    <button class="w-full mt-3 px-4 py-2 border border-blue-600 text-blue-600 text-sm font-medium rounded-lg hover:bg-blue-50 transition">
                                        Открыть зачетку
                                    </button>
                                </div>

                                <div class="p-4 border border-gray-200 rounded-lg">
                                    <h3 class="font-bold text-gray-900 mb-3">Запись на пересдачу</h3>
                                    <select class="w-full mb-3 px-3 py-2 border border-gray-300 rounded-lg text-sm">
                                        <option>Выберите предмет</option>
                                        <option>Физика</option>
                                        <option>Математика</option>
                                        <option>Философия</option>
                                    </select>
                                    <button class="w-full px-4 py-2 bg-blue-600 text-white text-sm font-medium rounded-lg hover:bg-blue-700 transition">
                                        Выбрать слот
                                    </button>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Контакты деканата</h2>

                            <div class="space-y-4">
                                <div class="p-3 bg-blue-50 rounded-lg">
                                    <div class="flex items-center gap-3">
                                        <div class="flex-shrink-0">
                                            <div class="w-10 h-10 bg-blue-100 rounded-full flex items-center justify-center">
                                                <svg class="w-5 h-5 text-blue-600" fill="currentColor" viewBox="0 0 20 20">
                                                    <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"/>
                                                </svg>
                                            </div>
                                        </div>
                                        <div>
                                            <h3 class="font-medium text-gray-900">Ваш куратор</h3>
                                            <p class="text-sm text-gray-600">Петрова Светлана Михайловна</p>
                                            <div class="flex items-center gap-2 mt-1">
                                                <a href="#" class="text-sm text-blue-600 hover:text-blue-800">Написать сообщение</a>
                                                <span class="text-gray-400">"•"</span>
                                                <a href="#" class="text-sm text-blue-600 hover:text-blue-800">Запись на консультацию</a>
                                            </div>
                                        </div>
                                    </div>
                                </div>

                                <div class="space-y-3">
                                    <div class="flex items-center text-sm">
                                        <svg class="w-4 h-4 text-gray-400 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                            <path d="M2 3a1 1 0 011-1h2.153a1 1 0 01.986.836l.74 4.435a1 1 0 01-.54 1.06l-1.548.773a11.037 11.037 0 006.105 6.105l.774-1.548a1 1 0 011.059-.54l4.435.74a1 1 0 01.836.986V17a1 1 0 01-1 1h-2C7.82 18 2 12.18 2 5V3z"/>
                                        </svg>
                                        <span class="text-gray-700">+7 (495) 123-45-67</span>
                                    </div>
                                    <div class="flex items-center text-sm">
                                        <svg class="w-4 h-4 text-gray-400 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                            <path d="M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z"/>
                                            <path d="M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z"/>
                                        </svg>
                                        <span class="text-gray-700">dekanat@university.ru</span>
                                    </div>
                                    <div class="flex items-center text-sm">
                                        <svg class="w-4 h-4 text-gray-400 mr-2" fill="currentColor" viewBox="0 0 20 20">
                                            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clip-rule="evenodd"/>
                                        </svg>
                                        <span class="text-gray-700">Пн-Пт: 9:00-18:00, Сб: 10:00-15:00</span>
                                    </div>
                                </div>

                                <div class="pt-4 border-t border-gray-200">
                                    <h4 class="font-medium text-gray-900 mb-2">Часто задаваемые вопросы</h4>
                                    <div class="space-y-2 text-sm">
                                        <a href="#" class="block text-blue-600 hover:text-blue-800 hover:underline">
                                            Как получить справку об обучении?
                                        </a>
                                        <a href="#" class="block text-blue-600 hover:text-blue-800 hover:underline">
                                            Порядок оформления академотпуска
                                        </a>
                                        <a href="#" class="block text-blue-600 hover:text-blue-800 hover:underline">
                                            Оплата обучения онлайн
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                            <h2 class="text-xl font-bold text-gray-900 mb-6">Личные данные</h2>

                            <div class="space-y-4">
                                <div class="p-3 bg-gray-50 rounded-lg">
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <h3 class="font-medium text-gray-900">Паспортные данные</h3>
                                            <p class="text-sm text-gray-600">Актуальность подтверждена</p>
                                        </div>
                                        <span class="px-2 py-1 bg-green-100 text-green-800 text-xs font-medium rounded-full">"✓"</span>
                                    </div>
                                </div>

                                <div class="p-3 bg-yellow-50 border border-yellow-200 rounded-lg">
                                    <div class="flex items-center justify-between">
                                        <div>
                                            <h3 class="font-medium text-gray-900">Медицинская справка</h3>
                                            <p class="text-sm text-yellow-700">Истекает через 14 дней</p>
                                        </div>
                                        <button class="px-3 py-1 bg-yellow-100 text-yellow-800 text-sm font-medium rounded-lg hover:bg-yellow-200 transition">
                                            Обновить
                                        </button>
                                    </div>
                                </div>

                                <div class="space-y-3">
                                    <h4 class="font-medium text-gray-900">Хранилище документов</h4>
                                    <div class="grid grid-cols-2 gap-2">
                                        <div class="text-center p-2 border border-gray-200 rounded">
                                            <div class="text-lg font-bold text-blue-600">5</div>
                                            <div class="text-xs text-gray-600">Загружено</div>
                                        </div>
                                        <div class="text-center p-2 border border-gray-200 rounded">
                                            <div class="text-lg font-bold text-green-600">3.2 МБ</div>
                                            <div class="text-xs text-gray-600">Свободно</div>
                                        </div>
                                    </div>
                                    <button class="w-full px-4 py-2 border border-gray-300 text-gray-700 text-sm font-medium rounded-lg hover:bg-gray-50 transition">
                                        Управление документами
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <div id="payment-modal" class="hidden fixed inset-0 bg-gray-600 bg-opacity-50 overflow-y-auto h-full w-full z-50">
                    <div class="relative top-20 mx-auto p-5 border w-96 shadow-lg rounded-md bg-white">
                        <div class="mt-3">
                            <h3 class="text-lg font-bold text-gray-900 mb-4">Оплата услуг</h3>

                            <div class="space-y-4">
                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">Сумма к оплате</label>
                                    <input type="text" value="2 500 ₽" readonly class="w-full px-3 py-2 border border-gray-300 rounded-lg text-lg font-bold text-center"/>
                                </div>

                                <div>
                                    <label class="block text-sm font-medium text-gray-700 mb-1">Способ оплаты</label>
                                    <select class="w-full px-3 py-2 border border-gray-300 rounded-lg">
                                        <option>Банковская карта</option>
                                        <option>СБП (Система быстрых платежей)</option>
                                        <option>Электронный кошелек</option>
                                    </select>
                                </div>

                                <div class="grid grid-cols-2 gap-4">
                                    <button class="px-4 py-2 bg-gray-200 text-gray-800 rounded-lg hover:bg-gray-300 transition">
                                        Отмена
                                    </button>
                                    <button class="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition">
                                        Оплатить
                                    </button>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}