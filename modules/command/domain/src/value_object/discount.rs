use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};
use thiserror::Error;

/// 割引のクラスです
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Discount {
  discount: Decimal,
}

/// 割引エラーのクラスです
#[derive(Debug, Clone, Error)]
pub struct DiscountError;

impl Display for DiscountError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Discount failed to validate")
  }
}

impl Display for Discount {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.discount)
  }
}

impl TryFrom<i32> for Discount {
  type Error = DiscountError;

  /// 実質的なコンストラクタです
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value < 0 && 101 < value {
      Err(DiscountError)?
    };
    Ok(Self::new(value))
  }
}

impl Discount {
  /// プライベートコンストラクタです
  fn new(discount: i32) -> Self {
    Self { discount: Decimal::from(discount) }
  }

  /// Getter
  pub fn value(&self) -> Decimal { self.discount }
}