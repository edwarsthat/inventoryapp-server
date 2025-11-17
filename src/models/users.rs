use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub contrasena_hash: String,
    pub rol: String,
    pub telefono: Option<String>,
    pub fecha_creacion: NaiveDateTime,
    pub ultimo_acceso: Option<NaiveDateTime>,
    pub activo: bool,
    pub debe_cambiar_contrasena: bool,
    pub fecha_ultimo_cambio: Option<NaiveDateTime>,  // Agregado
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id_usuario: i32,
    pub nombre: String,
    pub apellido: String,
    pub correo: String,
    pub username: String,
    pub rol: String,
    pub telefono: Option<String>,
    pub activo: bool,
    pub debe_cambiar_contrasena: bool,
}

impl From<User> for UserInfo {
    fn from(user: User) -> Self {
        UserInfo {
            id_usuario: user.id_usuario,
            nombre: user.nombre,
            apellido: user.apellido,
            correo: user.correo,
            username: user.username,
            rol: user.rol,
            telefono: user.telefono,
            activo: user.activo,
            debe_cambiar_contrasena: user.debe_cambiar_contrasena,
            // fecha_ultimo_cambio no va en UserInfo
        }
    }
}