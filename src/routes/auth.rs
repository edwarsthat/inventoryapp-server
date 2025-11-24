use crate::{controllers::auth, middleware::auth::require_auth, server::app::AppState};
use axum::{middleware, routing::post, Router};

pub fn routes() -> Router<AppState> {
    // Rutas públicas (sin autenticación)
    let public_routes = Router::new()
        .route("/api/auth/login", post(auth::login))
        .route("/api/auth/register", post(auth::register));

    // Rutas protegidas (requieren autenticación)
    let protected_routes = Router::new()
        .route("/api/auth/change-password", post(auth::change_password))
        .route_layer(middleware::from_fn(require_auth));

    // Combinar ambos routers
    public_routes.merge(protected_routes)
}
