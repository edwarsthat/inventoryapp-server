use crate::models::{
    errors::server_error::{ServerError, ServerErrorKind},
    users::User,
};
use sqlx::PgPool;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        nombre: &str,
        apellido: &str,
        correo: &str,
        username: &str,
        contrasena_hash: &str,
        rol: &str,
        telefono: Option<&str>,
    ) -> Result<User, ServerError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO usuarios (nombre, apellido, correo, username, contrasena_hash, rol, telefono, debe_cambiar_contrasena) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8) 
             RETURNING id_usuario, nombre, apellido, correo, username, contrasena_hash, 
                       rol, telefono, fecha_creacion, ultimo_acceso, activo, debe_cambiar_contrasena, fecha_ultimo_cambio"
        )
        .bind(nombre)
        .bind(apellido)
        .bind(correo)
        .bind(username)
        .bind(contrasena_hash)
        .bind(rol)
        .bind(telefono)
        .bind(false) // debe_cambiar_contrasena por defecto en false
        .fetch_one(&self.pool)
        .await
        .map_err(|e| ServerError::new(
            500,
            &format!("Error creando usuario: {}", e),
            ServerErrorKind::DatabaseError,
            "user_repository::create"
        ))
    }

    pub async fn find_by_username(&self, username: &str) -> Result<Option<User>, ServerError> {
        let user = sqlx::query_as::<_, User>("SELECT * FROM usuarios WHERE username = $1")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| {
                ServerError::new(
                    500,
                    &format!("Error al consultar el usuario por username: {}", err),
                    ServerErrorKind::DatabaseError,
                    "user_repository::find_by_username",
                )
            })?;
        Ok(user)
    }

    /// Verificar si existe un username
    pub async fn exists_by_username(&self, username: &str) -> Result<bool, ServerError> {
        let result: (bool,) =
            sqlx::query_as("SELECT EXISTS(SELECT 1 FROM usuarios WHERE username = $1)")
                .bind(username)
                .fetch_one(&self.pool)
                .await
                .map_err(|e| {
                    ServerError::new(
                        500,
                        &format!("Error verificando username: {}", e),
                        ServerErrorKind::DatabaseError,
                        "user_repository::exists_by_username",
                    )
                })?;

        Ok(result.0)
    }
}
