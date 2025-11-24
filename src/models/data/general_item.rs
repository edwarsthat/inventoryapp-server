use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    // OBLIGATORIOS: Datos mínimos para que el sistema funcione
    pub id: Uuid,
    pub barcode: String, // Sin esto, el escáner no sirve
    pub name: String,    // Necesitas saber qué es
    pub stock_quantity: i32, // Por defecto suele ser 0, pero debe ser un número

    // OPCIONALES: Datos que puedes llenar después
    
    /// Descripción detallada (puede estar vacía)
    pub description: Option<String>, 
    
    /// SKU interno (no todos los negocios usan SKU + Código de barras)
    pub sku: Option<String>, 

    /// Marca (a veces es un producto genérico)
    pub brand: Option<String>,

    /// Categoría (si aún no has clasificado el producto)
    pub category: Option<String>,

    /// Ubicación (si el producto está en recepción y aún no en estante)
    pub location_code: Option<String>,

    /// Fecha de vencimiento (Herramientas o ropa no tienen)
    pub expiration_date: Option<DateTime<Utc>>,

    // PRECIOS: A veces son opcionales si solo estás haciendo conteo
    // y finanzas lo llena después.
    pub cost_price: Option<Decimal>,
    pub selling_price: Option<Decimal>,
}