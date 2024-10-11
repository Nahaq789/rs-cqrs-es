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
         product_name: String,
         unit_price: Price,
         discount: Price,
         quantity: Quantity) -> Self {
    Self {
      order_item_id,
      product_id,
      product_name,
      unit_price,
      discount,
      quantity,
    }
  }

  pub fn place_order_item(
    order_item_id: i32,
    product_id: i32,
    product_name: String,
    unit_price: i32,
    discount: i32,
    quantity: u16,
  ) -> Result<Self, OrderError> {
    if !Self::check_quantity(quantity) {
      Err(OrderError::InvalidQuantityError(quantity))?
    };

    Ok(OrderItem::new(
      order_item_id,
      product_id,
      product_name,
      Price::try_from(unit_price)?,
      Price::try_from(discount)?,
      Quantity::new(quantity),
    ))
  }

  /// 数量のチェック
  /// 数量は1以上でなければならない
  ///
  /// # Argument
  /// * `quantity`: u16
  fn check_quantity(quantity: u16) -> bool {
    quantity > 0
  }

  // fn is_discount_valid_for_order_total(unit_price: i32, quantity: u16, discount: i32) -> bool {
  //   if (unit_price * )
  // }
}