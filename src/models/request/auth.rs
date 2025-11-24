use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub username: String,
    pub password: String,
    pub rol: String,
    pub telefono: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    pub username: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
}
