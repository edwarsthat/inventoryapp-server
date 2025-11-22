use serde::{Serialize};



#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub message: String,
    pub debe_cambiar_contrasena: bool,
    pub user_info: UserResponseLogin,
}

#[derive(Debug, Serialize)]
pub struct UserResponseLogin {
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub username: String,
    pub debe_cambiar_contrasena: bool,
    pub id_usuario: i32,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub message: String,
    pub username: String,
}