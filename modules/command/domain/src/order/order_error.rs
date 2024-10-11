use crate::value_object::price::PriceError;
use thiserror;
use thiserror::Error;

/// 注文のエラーです
#[derive(Debug, Error)]
pub enum OrderError {
  #[error("Quantity must be greater than or equal to 0 {0:?}")]
  InvalidQuantityError(u16),

  #[error("Price must be at least 1 {0:?}")]
  InvalidPriceError(#[from] PriceError),
}