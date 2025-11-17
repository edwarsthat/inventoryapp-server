use crate::models::errors::api_errors::ApiError;
use axum::Router;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub fn create_router(db_pool: PgPool) -> Router {
    let state = AppState { db: db_pool };

    Router::new()
        // Rutas sin estado
        // .merge(crate::routes::healt::routes())
        .merge(crate::routes::auth::routes())
        .fallback(handler_404)
        .with_state(state)
}

async fn handler_404() -> ApiError {
    ApiError::NotFound("Ruta no encontrada".to_string())
}
