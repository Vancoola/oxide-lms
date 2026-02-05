use leptos::prelude::*;
use leptos::view;
use leptos::web_sys::RequestCredentials;
use leptos_router::hooks::{use_location, use_navigate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub user: RwSignal<User>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub group_code: String,
}

#[component]
pub fn AuthProvider(children: ChildrenFn) -> impl IntoView {
    let user_resource = LocalResource::new(move || get_info());
    let location = use_location();
    let navigate = use_navigate();

    view! {
        {move || {
            match user_resource.get() {
                None => view! { <p>"Загрузка..."</p> }.into_any(),
                Some(Err(_)) => {
                    if location.pathname.get() != "/login" {
                        navigate("/login", Default::default());
                    }
                    children().into_any()
                },
                Some(Ok(user)) => {
                    provide_context(AuthContext { user: RwSignal::new(user) });
                    children().into_any()
                },
            }
        }}
    }
}

async fn get_info() -> Result<User, String> {
    let res = reqwasm::http::Request::get("http://localhost:3000/api/v1/users/me")
        .credentials(RequestCredentials::Include)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<User>()
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}