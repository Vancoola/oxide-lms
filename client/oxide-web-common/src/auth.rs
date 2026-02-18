use leptos::prelude::*;
use leptos::view;
use leptos::web_sys::RequestCredentials;
use leptos_router::hooks::{use_location, use_navigate};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
pub struct AuthContext {
    pub user: RwSignal<Option<User>>,
    pub resource: LocalResource<Result<User, String>>
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
    let user_resource = LocalResource::new(get_info);
    let location = use_location();
    let navigate = use_navigate();
    let user_signal = RwSignal::new(None);

    provide_context(AuthContext { user: user_signal, resource: user_resource });

    Effect::new(move |_| {
        match user_resource.get() {
            Some(Ok(user)) => {
                user_signal.set(Some(user));
                if location.pathname.get() == "/login" {
                    navigate("/", Default::default());
                }
            }
            Some(Err(_)) => {
                user_signal.set(None);
                if location.pathname.get() != "/login" {
                    navigate("/login", Default::default());
                }
            }
            None => {}
        }
    });
    view! {
        {children()}
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