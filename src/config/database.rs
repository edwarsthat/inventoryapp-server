use sqlx::postgres::PgPoolOptions;
use crate::models::errors::server_error::{ServerError, ServerErrorKind};

pub async fn create_pool(database_url: &str) -> Result<sqlx::PgPool, ServerError> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|err| {
            ServerError::new(
                500,
                &format!("Error al conectar con la base de datos: {}", err),
                ServerErrorKind::ConnectionError,
                "database::create_pool"
            )
        })
}