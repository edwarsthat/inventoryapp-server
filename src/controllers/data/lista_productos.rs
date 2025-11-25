use axum::{extract::State, Json};

use crate::{
    models::{
        errors::api_errors::ApiError, request::lista_productos::CreateProductRequest,
        responses::common_responses::ApiResponse,
    },
    repositories::data::lista_productos::ListaProductosRepository,
    server::app::AppState,
    services::data::lista_productos_service::ListaProductosService,
};

pub async fn create_product(
    State(state): State<AppState>,
    Json(request): Json<CreateProductRequest>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    if request.barcode.is_empty() || request.name.is_empty() {
        return Err(ApiError::BadRequest(
            "Barcode y nombre son obligatorios".to_string(),
        ));
    }

    let product_repo = ListaProductosRepository::new(state.db.clone());
    let product_service = ListaProductosService::new(product_repo);

    match product_service
        .create(&request.barcode, &request.name)
        .await
    {
        Ok(_) => Ok(Json(ApiResponse {
            status: "success".to_string(),
            message: "Producto creado exitosamente".to_string(),
            data: Some("".to_string()),
        })),
        Err(e) => return Err(e.into()),
    }
}
