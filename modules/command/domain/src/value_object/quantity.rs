use std::fmt::{Display, Formatter};

/// 数量を表すValueObjectです
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Quantity {
  quantity: u16,
}

impl Display for Quantity {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Quantity-{}", self.quantity)
  }
}

impl From<u16> for Quantity {
  fn from(value: u16) -> Self {
    Self::new(value)
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
  pub fn new(quantity: u16) -> Self {
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
  pub fn add(self, quantity: u16) -> Self {
    Quantity::new(self.quantity + quantity)
  }

  /// `quantity`を返却します
  ///
  /// # Return
  ///
  /// * `u16`
  pub fn value(&self) -> u16 {
    self.quantity
  }
}

