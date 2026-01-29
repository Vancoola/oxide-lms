use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

#[component]
pub fn Login() -> impl IntoView {
    view! {
    <div class="flex flex-col items-center justify-center min-h-screen">
        <div class="bg-white p-8 rounded-lg shadow-lg w-full max-w-md mb-8">

            <div class="text-center mb-8">
                <a href="/" class="text-3xl font-bold font-bold bg-gradient-to-r from-blue-700 to-blue-500 bg-clip-text text-transparent">
                    Университет
                </a>
            </div>

            <div class="space-y-6">
                <div>
                    <label for="login" class="block text-sm font-medium text-gray-700 mb-2">
                        Логин
                    </label>
                    <input
                        type="text"
                        id="login"
                        placeholder="Введите ваш логин"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition"
                    />
                </div>

                <div>
                    <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
                        Пароль
                    </label>
                    <input
                        type="password"
                        id="password"
                        placeholder="Введите ваш пароль"
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition"
                    />
                </div>

                <div class="flex items-center">
                    <input
                        type="checkbox"
                        id="remember"
                        class="h-4 w-4 text-blue-600 rounded focus:ring-blue-500"
                    />
                    <label for="remember" class="ml-2 text-sm text-gray-700">
                        Запомнить меня
                    </label>
                </div>

                <div>
                    <button class="w-full bg-blue-600 text-white py-2 px-4 rounded-lg hover:bg-blue-700 transition duration-200 font-medium">
                        Войти
                    </button>
                </div>
            </div>

            <div class="text-center mt-6">
                <a href="#" class="text-sm text-blue-600 hover:text-blue-800">
                    Забыли пароль?
                </a>
            </div>
        </div>
        <div class="fixed bottom-0 left-0 right-0 py-4 bg-white border-t border-gray-200">
        <div class="text-center text-gray-500 text-sm">
                <p class="mb-1">
                    <a href="/policy" class="text-gray-600 hover:text-gray-800 hover:underline">
                        Пользовательское соглашение
                    </a>
                </p>
                <p>"© 2026 AxionTech. Все права защищены."</p>
            </div>
        </div>
    </div>
    }
}