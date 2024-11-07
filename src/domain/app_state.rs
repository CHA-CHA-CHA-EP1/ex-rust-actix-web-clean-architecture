use std::sync::Arc;

use super::services::auth::AuthService;

pub struct AppState {
    pub auth_service: Arc<dyn AuthService>
}
