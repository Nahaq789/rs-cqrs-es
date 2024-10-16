use chrono::{DateTime, Utc};
use std::fmt::{Debug, Display};
use uuid::Uuid;

pub mod order;
pub mod value_object;
mod product;

/// 集約ID用のトレイトです
///
/// 各集約IDはAggregateIdを実装しなければなりません
///
/// - type_name: 集約の型を返します
/// - value: 値を返します
pub trait AggregateId {
  fn type_name(&self) -> String;
  fn value(&self) -> String;
  fn generate_id() -> Uuid;
}

/// 集約用のトレイトです
///
/// 各集約はAggregateを実装しなければなりません
pub trait Aggregate: Debug + Clone + Send + Sync + 'static {
  type ID: AggregateId;

  /// IDを返します
  fn id(&self) -> &Self::ID;

  /// シーケンス番号を返します
  fn seq_nr(&self) -> Uuid;

  /// シーケンス番号をセットします
  fn set_seq_nr(&mut self, swq: Uuid);

  /// バージョンを返します
  fn version(&self) -> usize;

  /// バージョンをセットします
  fn set_version(&mut self, version: usize);

  /// タイムスタンプを返します
  fn timestamp(&self) -> DateTime<Utc>;
}

pub trait Event: Debug + Clone + Send + Sync + 'static {
  type ID: Display;
  type AggregateID: AggregateId;
  fn id(&self) -> &Self::ID;
  fn aggregate_id(&self) -> &Self::AggregateID;
  fn occurred_at(&self) -> DateTime<Utc>;
  fn event_name(&self) -> String;
}