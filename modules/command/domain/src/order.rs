pub mod order_id;
mod order_error;
mod order_item;
mod order_item_id;

use crate::order::order_error::OrderError;
use crate::order::order_id::OrderId;
use crate::order::order_item::OrderItem;
use crate::value_object::price::Price;
use chrono;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone)]
pub struct Order {
  /// 注文ID
  id: OrderId,

  /// 注文日時
  ordered_at: DateTime<Utc>,

  /// 合計金額
  total_price: Price,

  /// 注文アイテム
  order_items: Vec<OrderItem>,
}

impl Order {
  /// コンストラクタです
  ///
  /// # Argument
  /// * `id`: OrderId
  /// * `created_at`: DateTime<Utc>
  /// * `total_price`: Price
  /// * `order_items`: Vec<OrderItem>
  ///
  /// # Return
  /// * `Order`
  fn new(
    id: OrderId,
    ordered_at: DateTime<Utc>,
    total_price: Price,
    order_items: Vec<OrderItem>,
  ) -> Self {
    Order {
      id,
      ordered_at,
      total_price,
      order_items,
    }
  }

  /// 外部から呼び出すコンストラクタです
  ///
  /// # Argument
  /// * `id`: OrderId
  /// * `created_at`: DateTime<Utc>
  /// * `total_price`: Price
  /// * `order_items`: Vec<OrderItem>
  ///
  /// # Return
  /// * `Result<Order, OrderError>`
  pub fn place_order(
    id: OrderId,
    ordered_at: DateTime<Utc>,
    order_items: Vec<OrderItem>,
  ) -> Result<Self, OrderError> {
    Ok(Order::new(
      id,
      ordered_at,
      Self::calc_total_price(&order_items)?,
      order_items,
    ))
  }

  /// 合計金額を計算します
  pub fn calc_total_price(items: &Vec<OrderItem>) -> Result<Price, OrderError> {
    let price: Decimal = items.iter()
      .try_fold(Decimal::ZERO, |acc, item| -> Result<Decimal, OrderError> {
        let unit_price = item.get_unit_price();
        let discount = item.get_discount();
        let quantity = Decimal::from(item.get_quantity());

        let item_total = unit_price * quantity;
        let discounted = item_total - (item_total * discount / Decimal::from(100));
        Ok(acc + discounted)
      })?;
    let result = Price::try_from(Decimal::from(price))?;
    Ok(result)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::order::order_item_id::OrderItemId;
  #[test]
  fn test_order_calc_total_price_success() {
    let data1 = OrderItem::place_order_item(
      OrderItemId::new(),
      1,
      "hogehoge",
      500,
      1,
      2,
    ).unwrap();
    let data2 = OrderItem::place_order_item(
      OrderItemId::new(),
      2,
      "fugafuga",
      100,
      1,
      10,
    ).unwrap();
    let vec: Vec<OrderItem> = vec![data1.clone(), data2.clone()];
    let result = Order::calc_total_price(&vec);

    let expected_value = {
      let data1_unit_price = data1.get_unit_price();
      let data1_quantity = Decimal::from(data1.get_quantity());
      let data1_discount = data1.get_discount();
      let data1_item_total = data1_unit_price * data1_quantity;
      let data1_discounted = data1_item_total - (data1_item_total * data1_discount / Decimal::from(100));

      let data2_unit_price = data2.get_unit_price();
      let data2_quantity = Decimal::from(data2.get_quantity());
      let data2_discount = data2.get_discount();
      let data2_item_total = data2_unit_price * data2_quantity;
      let data2_discounted = data2_item_total - (data2_item_total * data2_discount / Decimal::from(100));

      data1_discounted + data2_discounted
    };

    // assert
    assert_eq!(
      &expected_value,
      result.unwrap().value()
    )
  }

  #[test]
  fn test_order_place_order_success() {
    let order_id = OrderId::new();
    let ordered_at = Utc::now();
    let data1 = OrderItem::place_order_item(
      OrderItemId::new(),
      1,
      "hogehoge",
      500,
      1,
      2,
    ).unwrap();
    let data2 = OrderItem::place_order_item(
      OrderItemId::new(),
      2,
      "fugafuga",
      100,
      1,
      10,
    ).unwrap();
    let order_items: Vec<OrderItem> = vec![data1.clone(), data2.clone()];

    let result = Order::place_order(
      order_id.clone(),
      ordered_at.clone(),
      order_items,
    );

    // assert
    assert!(result.is_ok());
    assert_eq!(result.unwrap().id, order_id);
  }

  #[test]
  fn test_order_place_order_failed() {
    let order_id = OrderId::new();
    let ordered_at = Utc::now();
    let order_items: Vec<OrderItem> = vec![];

    let result = Order::place_order(
      order_id, ordered_at, order_items,
    );

    assert!(result.is_err())
  }
}