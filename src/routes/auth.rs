use axum::{routing::post, Router};
use crate::{controllers::auth, server::app::AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/api/auth/login", post(auth::login))
}