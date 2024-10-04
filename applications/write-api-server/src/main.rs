use axum::routing::get;
use axum::{Json, Router};
use command_domain::order::order_id::OrderId;
use config::Config;
use serde::Deserialize;
use serde_json::{json, Value};
use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tower_http::trace::TraceLayer;
use tracing::log::info;
use tracing::Level;

/// 各設定の集約的な構造体です
///
/// api: ApiSettings
#[derive(Deserialize, Debug)]
struct AppSettings {
  api: ApiSettings,
}

/// API起動時の設定用の構造体です
///
/// host: ホスト
///
/// port: ポート番号
#[derive(Deserialize, Debug)]
struct ApiSettings {
  host: String,
  port: u16,
}

/// 書き込み用サーバーの起動用関数です
///
/// 開発環境
/// 0.0.0.0:18080で起動します
///
/// 本番環境
/// 0.0.0.0:8080で起動します
///
/// # return
/// ```
/// anyhow::Result<()>
/// ```
#[tokio::main]
async fn main() -> anyhow::Result<()> {
  // ログ出力の設定
  tracing_subscriber::fmt()
    .with_max_level(Level::TRACE)
    .with_ansi(false)
    .with_target(false)
    .init();

  // 設定ファイルの読み込み
  let app_settings = load_app_config()?;

  // 仮でルーティング設定
  // Module側を作成したら置き換え
  let app = Router::new()
    .route("/", get(root))
    .layer(TraceLayer::new_for_http());

  // 起動用のアドレス
  let socket_addr = SocketAddr::new(
    IpAddr::from_str(&app_settings.api.host)?,
    app_settings.api.port,
  );
  info!("Binding to address: {}", socket_addr);

  let listener = tokio::net::TcpListener::bind(socket_addr)
    .await
    .expect("Failed to bind to address");

  info!("Write server started!!");
  info!("{}", std::env::var("HOGE")?);
  axum::serve(listener, app)
    .await?;

  Ok(())
}


/// write-api-server.tomlから設定ファイルを読み込みます
///
/// AppSettingsのapiとawsに値をセットします
///
/// ## return
/// ```
/// AppSettings
/// ```
#[cfg(debug_assertions)]
fn load_app_config() -> anyhow::Result<AppSettings> {
  let settings = Config::builder()
    .add_source(config::File::with_name("../../config/write-api-server"))
    .add_source(config::Environment::with_prefix("APP"))
    .build()?;
  let config = settings.try_deserialize::<AppSettings>()?;
  Ok(config)
}

/// 環境変数から設定を読み込みます
///
/// AppSettingsのapiとawsに値をセットします
///
/// ## return
/// ```
/// AppSettings
/// ```
#[cfg(not(debug_assertions))]
fn load_app_config() -> anyhow::Result<AppSettings> {
  let host = std::env::var("HOST").expect("HOST must set");
  let port = std::env::var("PORT").expect("PORT must set");

  let api_settings = ApiSettings { host: host, port: port.parse::<u16>().expect("failed to parse port") };

  Ok(AppSettings { api: api_settings })
}

async fn root() -> Json<Value> {
  let order_id = OrderId::new();
  Json(json!({ "msg": order_id }))
}