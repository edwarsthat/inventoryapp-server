use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};

use crate::services::jwt_service::verify_jwt;

pub async fn require_auth(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers().get("authorization");
    
    if auth_header.is_none() {
        println!("❌ Token faltante: No se encontró el header 'Authorization'");
        return Err(StatusCode::UNAUTHORIZED);
    }

    let auth_header = auth_header
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "));

    match auth_header {
        Some(token) => {
            println!("✅ Token encontrado: {}", &token[..token.len().min(20)]); // Muestra solo los primeros 20 caracteres
            verify_jwt(token).map_err(|e| {
                println!("❌ Token inválido: {:?}", e);
                StatusCode::UNAUTHORIZED
            })?;
            println!("✅ Token válido, acceso permitido");
            Ok(next.run(req).await)
        }
        None => {
            println!("❌ Formato de token inválido: No tiene el prefijo 'Bearer '");
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}