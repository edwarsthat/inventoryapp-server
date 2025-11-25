use crate::models::{
    data::lita_items::ListaProductos,
    errors::server_error::{ServerError, ServerErrorKind},
};
use sqlx::PgPool;
use uuid::Uuid;

pub struct ListaProductosRepository {
    pool: PgPool,
}

impl ListaProductosRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(
        &self,
        barcode: &str,
        name: &str,
        id: Uuid,
    ) -> Result<ListaProductos, ServerError> {
        sqlx::query_as::<_, ListaProductos>(
            "INSERT INTO lista_productos (barcode, name, id) 
             VALUES ($1, $2, $3) 
             RETURNING id, barcode, name",
        )
        .bind(barcode)
        .bind(name)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            ServerError::new(
                500,
                &format!("Error creating product: {}", e),
                ServerErrorKind::DatabaseError,
                "lista_elementos_repository::create",
            )
        })
    }
}
