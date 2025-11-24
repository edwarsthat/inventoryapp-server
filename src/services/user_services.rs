use crate::{
    config::env,
    models::{errors::api_errors::ApiError, users::User},
    repositories::user_repository::UserRepository,
    services::jwt_service::create_jwt,
};

pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn register(
        &self,
        nombre: &str,
        apellido: &str,
        correo: &str,
        username: &str,
        contrasena_hash: &str,
        rol: &str,
        telefono: Option<&str>,
        token_create_user: &str,
    ) -> Result<(), ApiError> {
        let config = env::load_config()
            .map_err(|e| ApiError::InternalError(format!("Error cargando configuración: {}", e)))?;

        if token_create_user != config.token_create_user {
            return Err(ApiError::Unauthorized(
                "Token de creación de usuario inválido".to_string(),
            ));
        }
        match self
            .user_repo
            .create(
                nombre,
                apellido,
                correo,
                username,
                contrasena_hash,
                rol,
                telefono,
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError::InternalError(format!(
                "Error al registrar usuario: {}",
                e
            ))),
        }
    }

    pub async fn login(&self, username: &str, passwrod: &str) -> Result<(String, User), ApiError> {
        let user = match self.user_repo.find_by_username(username).await {
            Err(e) => return Err(e.into()), // Conversión explícita
            Ok(None) => return Err(ApiError::Unauthorized("Credenciales inválidas".to_string())),
            Ok(Some(u)) => u,
        };

        // 2. Verificar si está activo
        if !user.activo {
            return Err(ApiError::Unauthorized("Usuario inactivo".to_string())); // Error 401
        }

        // 3. Verificar contraseña
        if !bcrypt::verify(passwrod, &user.contrasena_hash).unwrap_or(false) {
            return Err(ApiError::Unauthorized("Credenciales inválidas".to_string()));
        }

        // Aquí se generaría un token JWT
        let jwt_token = match create_jwt(&user.id_usuario, &user.rol) {
            Ok(token) => token,
            Err(e) => {
                return Err(ApiError::InternalError(format!(
                    "Error generando token: {}",
                    e
                )))
            }
        };
        println!("Usuario autenticado: {:?}", user);

        Ok((jwt_token, user))
    }

    pub async fn change_password(
        &self,
        username: &str,
        new_password_hash: &str,
    ) -> Result<(), ApiError> {

        match self
            .user_repo
            .update_password(username, new_password_hash, false)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError::InternalError(format!(
                "Error al cambiar la contraseña: {}",
                e
            ))),
        }
    }
}
