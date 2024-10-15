use uuid::Uuid;

mod aggregate_id;
pub mod order;
pub mod value_object;
mod product;

pub fn generate_id() -> Uuid {
  Uuid::new_v4()
}