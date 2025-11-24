use crate::{
    models::{
        errors::api_errors::ApiError,
        request::auth::{ChangePasswordRequest, LoginRequest, RegisterRequest},
        responses::auth::{LoginResponse, RegisterResponse, UserResponseLogin},
    },
    repositories::user_repository::UserRepository,
    server::app::AppState,
    services::user_services::AuthService,
};
use axum::{extract::State, Json};

pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, ApiError> {
    // 1. Validar que los campos no estén vacíos
    if request.nombre.trim().is_empty()
        || request.apellido.trim().is_empty()
        || request.correo.trim().is_empty()
        || request.username.trim().is_empty()
        || request.password.trim().is_empty()
        || request.rol.trim().is_empty()
        || request.token.as_deref().unwrap_or("").trim().is_empty()
    {
        return Err(ApiError::BadRequest(
            "Todos los campos son requeridos".to_string(),
        ));
    }

    // 2. Validar formato de correo (básico)
    if !request.correo.contains('@') {
        return Err(ApiError::BadRequest(
            "Correo electrónico inválido".to_string(),
        ));
    }

    // 3. Validar longitud de contraseña
    if request.password.len() < 6 {
        return Err(ApiError::BadRequest(
            "La contraseña debe tener al menos 6 caracteres".to_string(),
        ));
    }

    // 4. Hashear la contraseña
    let password_hash = bcrypt::hash(&request.password, bcrypt::DEFAULT_COST)
        .map_err(|_| ApiError::InternalError("Error al hashear contraseña".to_string()))?;

    // 5. Crear el repositorio y servicio
    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo);

    // 6. Registrar usuario
    auth_service
        .register(
            &request.nombre,
            &request.apellido,
            &request.correo,
            &request.username,
            &password_hash,
            &request.rol,
            request.telefono.as_deref(),
            request.token.as_deref().unwrap_or(""),
        )
        .await?;

    // 7. Retornar respuesta exitosa
    Ok(Json(RegisterResponse {
        message: "Usuario registrado exitosamente".to_string(),
        username: request.username,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo);

    let (token, user) = auth_service
        .login(&request.username, &request.password)
        .await?;

    Ok(Json(LoginResponse {
        token,
        message: "Login exitoso".to_string(),
        debe_cambiar_contrasena: user.debe_cambiar_contrasena,
        user_info: UserResponseLogin {
            nombre: user.nombre,
            apellido: user.apellido,
            correo: user.correo,
            username: user.username,
            debe_cambiar_contrasena: user.debe_cambiar_contrasena,
            id_usuario: user.id_usuario,
        },
    }))
}

pub async fn change_password(
    State(state): State<AppState>,
    Json(request): Json<ChangePasswordRequest>,
) -> Result<Json<crate::models::responses::common_responses::ApiResponse<String>>, ApiError> {
    println!("Attempting to change password for user");

    let user_repo = UserRepository::new(state.db.clone());
    let auth_service = AuthService::new(user_repo);

    let password_hash = bcrypt::hash(&request.new_password, bcrypt::DEFAULT_COST)
        .map_err(|_| ApiError::InternalError("Error al hashear contraseña".to_string()))?;

    match auth_service
        .change_password(&request.username, &password_hash)
        .await
    {
        Ok(_) => {
            println!("Password changed successfully");
        }
        Err(e) => {
            println!("Error changing password: {:?}", e);
            return Err(e);
        }
    }

    Ok(Json(
        crate::models::responses::common_responses::ApiResponse::success(
            "Contraseña cambiada exitosamente",
            None,
        ),
    ))
}
