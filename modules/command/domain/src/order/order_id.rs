use crate::aggregate_id::AggregateId;
use crate::generate_id;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use uuid::Uuid;

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct OrderId {
  value: Uuid,
}

const ORDER_PREFIX: &str = "ORDER";

impl OrderId {
  pub fn new() -> Self {
    let value = generate_id();
    Self { value }
  }
}

impl AggregateId for OrderId {
  fn type_name(&self) -> String {
    ORDER_PREFIX.to_string()
  }
  fn value(&self) -> String {
    self.value.to_string()
  }
}

impl Display for OrderId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{}", self.type_name(), self.value())
  }
}

impl From<Uuid> for OrderId {
  fn from(value: Uuid) -> Self {
    Self { value }
  }
}