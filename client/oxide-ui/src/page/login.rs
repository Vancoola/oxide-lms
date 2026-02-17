use leptos::ev::SubmitEvent;
use leptos::html;
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_i18n::{t, t_string};
use leptos_router::hooks::use_navigate;
use reqwasm::http::RequestCredentials;
use oxide_i18n::oxide_i18n::i18n::use_i18n;
use oxide_shared_types::auth::{AuthRequest};

#[component]
pub fn Login() -> impl IntoView {

    let email_ref: NodeRef<html::Input> = NodeRef::new();
    let password_ref: NodeRef<html::Input> = NodeRef::new();


    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let navigate = use_navigate();
        let request = AuthRequest {
            email: email_ref.get().unwrap().value(),
            password: password_ref.get().unwrap().value(),
        };
        spawn_local(async move {
            let _ = login_req(request).await;
            navigate("/", Default::default());
        });
    };

    let i18n = use_i18n();

    view! {
    <div class="flex flex-col items-center justify-center min-h-screen">
        <form on:submit=on_submit class="bg-white p-8 rounded-lg shadow-lg w-full max-w-md mb-8">

            <div class="text-center mb-8">
                <a href="/" class="text-3xl font-bold font-bold bg-gradient-to-r from-blue-700 to-blue-500 bg-clip-text text-transparent">
                    {move || t!(i18n, brand_title)}
                </a>
            </div>

            <div class="space-y-6">
                <div>
                    <label for="login" class="block text-sm font-medium text-gray-700 mb-2">
                        {move || t!(i18n, username)}
                    </label>
                    <input
                        node_ref=email_ref
                        type="text"
                        id="login"
                        placeholder=move || t_string!(i18n, login_placeholder)
                        class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-blue-500 transition"
                    />
                </div>

                <div>
                    <label for="password" class="block text-sm font-medium text-gray-700 mb-2">
                        {move || t!(i18n, password)}
                    </label>
                    <input
                        node_ref=password_ref
                        type="password"
                        id="password"
                        placeholder=move || t_string!(i18n, password_placeholder)
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
                        {move || t!(i18n, remember_me)}
                    </label>
                </div>

                <div>
                    <button class="w-full bg-blue-600 text-white py-2 px-4 rounded-lg hover:bg-blue-700 transition duration-200 font-medium">
                        {move || t!(i18n, login_btn)}
                    </button>
                </div>
            </div>

            <div class="text-center mt-6">
                <a href="#" class="text-sm text-blue-600 hover:text-blue-800">
                    {move || t!(i18n, forgot_password)}
                </a>
            </div>
        </form>
        <div class="fixed bottom-0 left-0 right-0 py-4 bg-white border-t border-gray-200">
        <div class="text-center text-gray-500 text-sm">
                <p class="mb-1">
                    <a href="/policy" class="text-gray-600 hover:text-gray-800 hover:underline">
                        {move || t!(i18n, user_agreement)}
                    </a>
                </p>
                <p>{move || t!(i18n, copyright)}</p>
            </div>
        </div>
    </div>
    }
}

async fn login_req(request: AuthRequest) -> Result<(), String> {
    let body_json = serde_json::to_string(&request)
        .map_err(|e| format!("Failed to serialize request: {}", e))?;
    let res = reqwasm::http::Request::post("http://localhost:3000/api/v1/auth/login")
        .header("Content-Type", "application/json")
        .body(body_json)
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .ok();
    if res == false {
        return Err(String::from("Login failed"));
    }
    Ok(())
}
