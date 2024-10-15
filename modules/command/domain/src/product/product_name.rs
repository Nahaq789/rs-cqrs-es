use std::fmt::{Display, Formatter};
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProductName(String);

#[derive(Debug, Error)]
pub enum ProductNameError {
  #[error("product name is empty")]
  NameEmpty
}

impl ProductName {
  pub fn new(value: &str) -> Result<Self, ProductNameError> {
    if value.is_empty() { Err(ProductNameError::NameEmpty)? }
    Ok(Self(value.to_string()))
  }
}

impl FromStr for ProductName {
  type Err = ProductNameError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Self::new(s)
  }
}

impl Display for ProductName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}