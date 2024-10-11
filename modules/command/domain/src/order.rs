pub mod order_id;
mod order_error;
mod order_item;

use crate::order::order_error::OrderError;
use crate::order::order_id::OrderId;
use chrono;
use chrono::{DateTime, Utc};

///
#[derive(Debug, Clone)]
pub struct Order {
  /// 注文ID
  id: OrderId,

  /// 商品ID
  item_id: String,

  /// 注文日時
  ordered_at: DateTime<Utc>,
}

impl Order {
  /// コンストラクタです
  ///
  /// # Argument
  /// * `id`: OrderId
  /// * `item_id`: String
  /// * `created_at`: DateTime<Utc>
  ///
  /// # Return
  /// * `Order`
  fn new(
    id: OrderId,
    item_id: String,
    ordered_at: DateTime<Utc>,
  ) -> Self {
    Order {
      id,
      item_id,
      ordered_at,
    }
  }

  /// 外部から呼び出すコンストラクタです
  ///
  /// # Argument
  /// * `id`: OrderId
  /// * `item_id`: String
  /// * `created_at`: DateTime<Utc>
  ///
  /// # Return
  /// * `Result<Order, OrderError>`
  pub fn place_order(
    id: OrderId,
    item_id: String,
    ordered_at: DateTime<Utc>,
  ) -> Result<Self, OrderError> {
    Ok(Order::new(
      id, item_id, ordered_at,
    ))
  }
}