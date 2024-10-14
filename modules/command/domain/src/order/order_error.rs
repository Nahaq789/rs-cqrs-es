use crate::value_object::price::PriceError;
use thiserror;
use thiserror::Error;
use crate::value_object::quantity::QuantityError;

/// 注文のエラーです
#[derive(Debug, Error)]
pub enum OrderError {
    #[error("Quantity must be greater than or equal to 0 {0:?}")]
    InvalidQuantityError(#[from] QuantityError),

    #[error("Price must be at least 1 {0:?}")]
    InvalidPriceError(#[from] PriceError),
}
