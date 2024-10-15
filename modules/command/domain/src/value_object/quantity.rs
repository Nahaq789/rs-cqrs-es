use std::fmt::{Display, Formatter};
use thiserror::Error;

/// 数量を表すValueObjectです
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Quantity {
  quantity: i32,
}

#[derive(Error, Debug, Clone)]
pub struct QuantityError;

impl Display for QuantityError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Quantity failed to validate")
  }
}

impl Display for Quantity {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Quantity-{}", self.quantity)
  }
}

impl TryFrom<i32> for Quantity {
  type Error = QuantityError;

  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value <= 0 {
      Err(QuantityError)?
    };
    Ok(Quantity::new(value))
  }
}

impl Quantity {
  /// コンストラクタです
  ///
  /// # Arguments
  ///
  /// * `quantity` 数量
  ///
  /// # Return
  ///
  /// * `Quantity`
  fn new(quantity: i32) -> Self {
    Self { quantity }
  }

  /// 数量をプラスします
  ///
  /// # Arguments
  ///
  /// * `quantity` 数量
  ///
  /// # Return
  ///
  /// * `Quantity`
  pub fn add(&self, value: i32) -> Result<Self, QuantityError> {
    if value <= 0 {
      Err(QuantityError)?
    }
    Ok(Quantity::new(self.quantity + value))
  }

  /// `quantity`を返却します
  ///
  /// # Return
  ///
  /// * `Decimal`
  pub fn value(&self) -> i32 {
    self.quantity
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  #[rstest]
  #[case(1, 10)]
  #[case(2, 78)]
  fn test_quantity_add_success(#[case] base: i32, #[case] value: i32) {
    let quantity = Quantity::try_from(base);
    let result = Quantity::add(&quantity.clone().unwrap(), value);

    // assert
    assert!(result.is_ok());
    assert_eq!(base + value, result.unwrap().quantity)
  }

  #[rstest]
  #[case(0)]
  #[case(-1)]
  fn test_quantity_add_failed(#[case] value: i32) {
    let quantity = Quantity::try_from(1);
    let result = Quantity::add(&quantity.unwrap(), value);

    // assert
    assert!(result.is_err())
  }

  #[rstest]
  #[case(10)]
  #[case(1)]
  fn test_quantity_value_success(#[case] value: i32) {
    let result = Quantity::try_from(value).unwrap().value();

    // assert
    assert_eq!(value, result)
  }
}
