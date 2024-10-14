use crate::order::order_error::OrderError;
use crate::value_object::price::Price;
use crate::value_object::quantity::Quantity;

#[derive(Debug, Clone)]
pub struct OrderItem {
  order_item_id: i32,
  product_id: i32,
  product_name: String,
  unit_price: Price,
  discount: Price,
  quantity: Quantity,
}

impl OrderItem {
  fn new(order_item_id: i32,
         product_id: i32,
         product_name: &str,
         unit_price: Price,
         discount: Price,
         quantity: Quantity) -> Self {
    Self {
      order_item_id,
      product_id,
      product_name: product_name.to_string(),
      unit_price,
      discount,
      quantity,
    }
  }

  pub fn place_order_item(
    order_item_id: i32,
    product_id: i32,
    product_name: &str,
    unit_price: i32,
    discount: i32,
    quantity: i32,
  ) -> Result<Self, OrderError> {
    Ok(OrderItem::new(
      order_item_id,
      product_id,
      product_name,
      Price::try_from(unit_price)?,
      Price::try_from(discount)?,
      Quantity::try_from(quantity)?
    ))
  }

  /// 数量のチェック
  /// 数量は1以上でなければならない
  ///
  /// # Argument
  /// * `quantity`: u16
  fn check_quantity(quantity: i32) -> bool {
    quantity > 0
  }

  pub fn get_unit_price(&self) -> i32 { self.unit_price.value() }

  pub fn get_quantity(&self) -> i32 { self.quantity.value() }

  pub fn get_discount(&self) -> i32 { self.discount.value() }
}