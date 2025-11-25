use crate::{
    models::errors::api_errors::ApiError,
    repositories::data::lista_productos::ListaProductosRepository,
};

pub struct ListaProductosService {
    lista_elementos_repository: ListaProductosRepository,
}

impl ListaProductosService {
    pub fn new(lista_elementos_repository: ListaProductosRepository) -> Self {
        Self {
            lista_elementos_repository,
        }
    }

    pub async fn create(&self, barcode: &str, name: &str) -> Result<(), ApiError> {
        let id = uuid::Uuid::new_v4();
        match self
            .lista_elementos_repository
            .create(barcode, name, id)
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(ApiError::InternalError(format!(
                "Error al crear lista de productos: {}",
                e
            ))),
        }
    }
}
