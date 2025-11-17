use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::{
    models::errors::api_errors::ApiError,
    repositories::user_repository::UserRepository,
    server::app::AppState,
    services::user_services::AuthService,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    
    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo);
    let token = auth_service.login(&request.username, &request.password).await?;
    
    Ok(Json(LoginResponse {
        token,
        message: "Login exitoso".to_string(),
    }))
}