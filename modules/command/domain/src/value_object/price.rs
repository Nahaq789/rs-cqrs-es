use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// 金額のクラスです
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Price {
  price: Decimal,
}

/// 金額エラーのクラスです
#[derive(Debug, Clone, Error)]
pub struct PriceError;

impl Display for PriceError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Price failed to validate")
  }
}

impl Display for Price {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.price)
  }
}

impl TryFrom<Decimal> for Price {
  type Error = PriceError;

  /// 実質的なコンストラクタです
  fn try_from(value: Decimal) -> Result<Self, Self::Error> {
    if value <= Decimal::from(0) {
      Err(PriceError)?
    };
    Ok(Self::new(value))
  }
}

impl Price {
  /// プライベートコンストラクタです
  fn new(price: Decimal) -> Self {
    Self { price }
  }

  /// 引数で受け取った値で足し算
  pub fn add(self, value: Decimal, quantity: i32) -> Result<Self, PriceError> {
    if value <= Decimal::from(0) {
      Err(PriceError)?
    }
    Ok(Self::new(self.price + (value * Decimal::from(quantity))))
  }

  /// Getter
  pub fn value(&self) -> Decimal { self.price }
}

#[cfg(test)]
mod tests {
  use super::*;
  use rstest::rstest;

  #[rstest]
  #[case(1)]
  #[case(100)]
  fn test_price_new_success(#[case] value: i32) {
    let result = Price::new(Decimal::from(value));
    assert_eq!(Decimal::from(value), result.price)
  }

  #[test]
  fn test_price_try_from_success() {
    let value = Decimal::from(1);
    let result = Price::try_from(value);

    // assert
    assert!(result.is_ok());
    assert_eq!(value, result.unwrap().price)
  }

  #[test]
  fn test_price_try_from_failed() {
    let value = Decimal::ZERO;
    let result = Price::try_from(value);

    //assert
    assert!(result.is_err())
  }

  #[rstest]
  #[case(100, 2)]
  #[case(300, 5)]
  fn test_price_add_success(#[case] value: i32, #[case] quantity: i32) {
    let price = Price::try_from(Decimal::from(value));
    let result = Price::add(
      price.unwrap(),
      Decimal::from(value),
      quantity,
    );

    //assert
    assert!(result.is_ok())
  }

  #[test]
  fn test_price_add_failed() {
    let price = Price::try_from(Decimal::from(1));
    let result = Price::add(
      price.unwrap(),
      Decimal::from(0),
      1,
    );

    //assert
    assert!(result.is_err())
  }

  #[rstest]
  #[case(100)]
  #[case(50)]
  fn test_price_value_success(#[case] value: i32) {
    let result = Price::try_from(Decimal::from(value));

    // assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap().value(), Decimal::from(value))
  }
}