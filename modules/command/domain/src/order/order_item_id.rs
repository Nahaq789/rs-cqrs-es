use crate::aggregate_id::AggregateId;
use crate::generate_id;
use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct OrderItemId {
  value: Uuid,
}

const ORDER_ITEM_PREFIX: &str = "ORDER_ITEM";

impl OrderItemId {
  pub fn new() -> Self {
    let value = generate_id();
    Self { value }
  }
}

impl AggregateId for OrderItemId {
  fn type_name(&self) -> String { ORDER_ITEM_PREFIX.to_string() }
  fn value(&self) -> String { self.value.to_string() }
}

impl Display for OrderItemId {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}-{}", self.type_name(), self.value)
  }
}

impl From<Uuid> for OrderItemId {
  fn from(value: Uuid) -> Self {
    Self { value }
  }
}