use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ListaProductos {
    pub id: Uuid,
    pub barcode: String,
    pub name: String,
}
