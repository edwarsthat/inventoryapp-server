use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateProductRequest {
    pub barcode: String,
    pub name: String,
}
