use axum::{middleware, routing::post, Router};

use crate::{
    controllers::data::lista_productos, middleware::auth::require_auth, server::app::AppState,
};

pub fn routes() -> Router<AppState> {
    // Rutas protegidas (requieren autenticación)
    let protected_routes = Router::new()
        .route(
            "/api/lista_productos",
            post(lista_productos::create_product),
        )
        .route_layer(middleware::from_fn(require_auth));

    // Combinar ambos routers
    protected_routes
}
