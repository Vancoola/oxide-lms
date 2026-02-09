use std::sync::Arc;
use crate::user::plugin::guards::PreRegistrationGuard;
use crate::user::plugin::hooks::PostRegistrationHook;
use crate::user::plugin::middleware::UserMiddleware;

pub mod hooks;
pub mod guards;
pub mod middleware;

pub struct UserExtensionRegistry {
    pub guards: Vec<Arc<dyn PreRegistrationGuard>>,
    pub middlewares: Vec<Arc<dyn UserMiddleware>>,
    pub hooks: Vec<Arc<dyn PostRegistrationHook>>,
}

impl UserExtensionRegistry {
    pub fn new() -> Self {
        Self {
            guards: Vec::new(),
            middlewares: Vec::new(),
            hooks: Vec::new(),
        }
    }
}