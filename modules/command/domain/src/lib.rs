use uuid::Uuid;

mod aggregate_id;
pub mod order;
pub mod value_object;

pub fn generate_id() -> Uuid {
  Uuid::new_v4()
}