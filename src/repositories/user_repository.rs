use sqlx::PgPool;
use crate::models::{errors::server_error::{ServerError, ServerErrorKind}, users::User};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {

    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, ServerError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM usuarios WHERE username = $1"
        )
        .bind(username)
        .fetch_optional(&self.pool)
        .await
        .map_err(|err| {
            ServerError::new(
                500,
                &format!("Error al consultar el usuario por username: {}", err),
                ServerErrorKind::DatabaseError,
                "user_repository::find_by_username"
            )
        })?;
        Ok(user)
    }

        /// Verificar si existe un username
    pub async fn exists_by_username(&self, username: &str) -> Result<bool, ServerError> {
        let result: (bool,) = sqlx::query_as(
            "SELECT EXISTS(SELECT 1 FROM usuarios WHERE username = $1)"
        )
        .bind(username)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ServerError::new(
            500,
            &format!("Error verificando username: {}", e),
            ServerErrorKind::DatabaseError,
            "user_repository::exists_by_username"
        ))?;
        
        Ok(result.0)
    }
}