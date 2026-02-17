use leptos::prelude::*;
use oxide_i18n::oxide_i18n::i18n::use_i18n;
use leptos_i18n::{t, t_string};

#[component]
pub fn NotFound() -> impl IntoView{

    let i18n = use_i18n();

    view! {
        <div class="flex flex-col items-center justify-center py-40">
            <div class="text-center">
                <div class="space-y-6">
                    <div>
                        <h1 class="text-9xl font-bold bg-gradient-to-r from-blue-600 to-blue-400 bg-clip-text text-transparent">
                            404
                        </h1>
                    </div>

                    <div>
                        <h2 class="text-2xl font-semibold text-gray-800">
                            {move || t!(i18n, page_not_found)}
                        </h2>
                    </div>

                    <div class="max-w-md mx-auto">
                        <p class="text-gray-600">
                            {move || t!(i18n, not_found_error)}
                        </p>
                    </div>

                    <div class="pt-4">
                        <a href="/"
                           class="inline-block bg-blue-600 text-white py-3 px-8 rounded-lg hover:bg-blue-700 transition duration-200 font-medium">
                            {move || t!(i18n, go_back)}
                        </a>
                    </div>
                </div>

                <div class="mt-10 pt-6 border-t border-gray-200">
                    <p class="text-gray-500 text-sm">
                        {move || t!(i18n, contact_support)}
                        <a href="/support" class="text-blue-600 hover:text-blue-800 hover:underline">
                            " "{move || t!(i18n, customer_support)}
                        </a>
                    </p>
                </div>
            </div>
        </div>
    }
}