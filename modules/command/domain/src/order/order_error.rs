use crate::product::product_name::ProductNameError;
use crate::value_object::discount::DiscountError;
use crate::value_object::price::PriceError;
use crate::value_object::quantity::QuantityError;
use thiserror;
use thiserror::Error;

/// 注文のエラーです
#[derive(Debug, Error)]
pub enum OrderError {
  #[error("Quantity must be greater than or equal to 0 {0:?}")]
  InvalidQuantityError(#[from] QuantityError),

  #[error("Price must be at least 1 {0:?}")]
  InvalidPriceError(#[from] PriceError),

  #[error("Discount must be at least 0 {0:?}")]
  InvalidDiscountError(#[from] DiscountError),

  #[error("Invalid Product Name: {0}")]
  InvalidProductName(#[from] ProductNameError),
}
