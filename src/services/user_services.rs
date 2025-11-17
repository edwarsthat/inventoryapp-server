use crate::{models::errors::api_errors::ApiError, repositories::user_repository::UserRepository};

pub struct AuthService {
    user_repo: UserRepository,
}

impl AuthService {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn login(&self, username: &str, passwrod: &str) -> Result<String, ApiError> {
        let user = match self.user_repo.find_by_username(username).await {
            Err(e) => return Err(e.into()), // Conversión explícita
            Ok(None) => return Err(ApiError::Unauthorized("Credenciales inválidas".to_string())),
            Ok(Some(u)) => u,
        };

        // 2. Verificar si está activo
        if !user.activo {
            return Err(ApiError::Unauthorized("Usuario inactivo".to_string())); // Error 401
        }

        println!("Usuario autenticado: {:?}", user);

        Ok("token".to_string())
    }
}
