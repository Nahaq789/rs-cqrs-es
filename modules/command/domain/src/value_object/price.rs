use std::fmt::{Display, Formatter};

/// 金額のクラスです
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Price {
  price: i32,
}

/// 金額エラーおのクラスです
#[derive(Debug, Clone)]
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

impl TryFrom<i32> for Price {
  type Error = PriceError;

  /// 実質的なコンストラクタです
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    if value < 0 {
      Err(PriceError)?
    };
    Ok(Self::new(value))
  }
}

impl Price {
  /// プライベートコンストラクタです
  fn new(price: i32) -> Self { Self { price } }

  /// 引数で受け取った値で足し算
  pub fn add(self, value: i32) -> Result<Self, PriceError> {
    if value < 0 {
      Err(PriceError)?
    }
    Ok(Self::new(self.price + value))
  }
}