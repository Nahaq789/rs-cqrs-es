/// 集約ID用のトレイトです
///
/// 各集約IDはAggregateIdを実装しなければなりません
///
/// - type_name: 集約の型を返します
/// - value: 値を返します
pub trait AggregateId {
  fn type_name(&self) -> String;
  fn value(&self) -> String;
}