use crate::order::order_error::OrderError;
use crate::order::order_item_id::OrderItemId;
use crate::product::product_name::ProductName;
use crate::value_object::discount::Discount;
use crate::value_object::price::Price;
use crate::value_object::quantity::Quantity;
use rust_decimal::Decimal;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct OrderItem {
  order_item_id: OrderItemId,
  product_id: i32,
  product_name: ProductName,
  unit_price: Price,
  discount: Discount,
  quantity: Quantity,
}

impl OrderItem {
  fn new(order_item_id: OrderItemId,
         product_id: i32,
         product_name: ProductName,
         unit_price: Price,
         discount: Discount,
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
    order_item_id: OrderItemId,
    product_id: i32,
    product_name: &str,
    unit_price: i32,
    discount: i32,
    quantity: i32,
  ) -> Result<Self, OrderError> {
    Ok(OrderItem::new(
      order_item_id,
      product_id,
      ProductName::from_str(product_name)?,
      Price::try_from(Decimal::from(unit_price))?,
      Discount::try_from(discount)?,
      Quantity::try_from(quantity)?,
    ))
  }

  /// 価格のゲッター
  /// 参照を返します。
  ///
  /// # return
  /// * `unit_price`: i32
  pub fn get_unit_price(&self) -> &Decimal { &self.unit_price.value() }

  /// 数量のゲッター
  /// 参照を返します。
  ///
  /// # return
  /// * `quantity`: i32
  pub fn get_quantity(&self) -> i32 { self.quantity.value() }

  /// 割引のゲッター
  /// 参照を返します。
  ///
  /// # return
  /// * `discount`: i32
  pub fn get_discount(&self) -> &Decimal { &self.discount.value() }
}