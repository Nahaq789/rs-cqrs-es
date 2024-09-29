use anyhow::Result;
use axum::routing::get;
use axum::Router;
use config::Config;
use serde::Deserialize;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tracing::{info, Level};

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

/// 読み込み用サーバーの起動用関数です
///
/// 0.0.0.0:18080で起動します
///
/// # return
/// ```
/// anyhow::Result<()>
/// ```
#[tokio::main]
async fn main() -> Result<()> {
    // ログ出力の設定
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_ansi(false)
        .with_target(false)
        .init();

    // 仮でルーティング設定
    // Module側を作成したら置き換え
    let app = Router::new()
        .route("/", get(|| async { "Hello World" }));

    // configの読み込み
    let app_settings = load_app_config()?;

    // 起動用のアドレス
    let socket_addr = SocketAddr::new(
        IpAddr::from_str(&app_settings.api.host)?,
        app_settings.api.port,
    );
    info!("Read server started!!");
    info!("Running on: http://{}", &socket_addr);
    let listener = tokio::net::TcpListener::bind(socket_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// read-api-serverから設定ファイルを読み込みます
///
/// AppSettingsのapiとawsに値をセットします
///
/// ## return
/// ```
/// AppSettings
/// ```
fn load_app_config() -> Result<AppSettings> {
    let settings = Config::builder()
        .add_source(config::File::with_name("../../config/read-api-server"))
        .add_source(config::Environment::with_prefix("APP"))
        .build()?;
    let config = settings.try_deserialize::<AppSettings>()?;
    Ok(config)
}