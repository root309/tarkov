extern crate reqwest;
extern crate serde_json;
extern crate twilight_gateway;
extern crate twilight_http;
extern crate twilight_model;

use dotenv::dotenv;
use std::env;
use twilight_gateway::{cluster::{ShardScheme, Cluster}, Intents};
use twilight_http::Client as HttpClient;
use futures::stream::StreamExt;
use serde_json::Value;

#[tokio::main]
async fn main() {
    dotenv().ok(); // .envファイルから環境変数を読み込む

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment"); // 環境変数からトークンを取得

    let http = HttpClient::new(token.clone()); // HTTPクライアントを初期化

    // クラスタを作成してShardSchemeを自動に設定
    let cluster = Cluster::builder(token, Intents::GUILD_MESSAGES)
        .shard_scheme(ShardScheme::Auto)
        .build()
        .await
        .expect("Cluster create failed");

    let mut events = cluster.events(); // イベントストリームを取得

    // 別の非同期タスクとしてクラスタを起動
    let cluster_spawn_handle = tokio::spawn(async move {
        cluster.up().await;
    });

    println!("Bot is now running."); // Bot起動

    // イベントストリームからイベントを受け取り続ける
    while let Some(( _ , event)) = events.next().await {
        match event {
            twilight_gateway::Event::MessageCreate(msg) => { // メッセージ作成イベントを処理
                if msg.content == "!btc" { // コマンドをチェック
                    let price = get_btc_price().await; // BTC価格を取得
                    let _ = http
                        .create_message(msg.channel_id) // メッセージを作成
                        .content(format!("Current BTC Price: {}", price)) // 価格情報を含むメッセージを設定
                        .expect("Message content error")
                        .await
                        .expect("HTTP request failed");
                }
            }
            _ => {},
        }
    }

    cluster_spawn_handle.await.expect("Cluster task failed"); // クラスタタスクの完了を待つ
}

// ビットコインの価格を取得する非同期関数
async fn get_btc_price() -> String {
    let query = r#"
        {
            items(name: "Physical bitcoin") {
                id
                name
                traderPrices {
                    trader {
                        name
                    }
                    price
                    currency
                }
            }
        }
    "#;
    let client = reqwest::Client::new(); // HTTPクライアントを初期化
    let body = serde_json::json!({
        "query": query // GraphQLクエリを設定
    });

    // GraphQLエンドポイントにPOSTリクエストを送信し、レスポンスを解析する
    match client.post("https://api.tarkov.dev/graphql")
        .header("Content-Type", "application/json") // ヘッダーを設定
        .body(serde_json::to_string(&body).unwrap())  // bodyをJSONとして送信する
        .send()
        .await {
            Ok(response) => {
                if let Ok(json) = response.json::<Value>().await {
                    //println!("{:?}", json); // レスポンスを表示
                    // アイテムとトレーダーの価格情報を解析し、整形する
                    let item = &json["data"]["items"][0];
                    let trader_prices = &item["traderPrices"];
                    let prices: Vec<String> = trader_prices.as_array().unwrap_or(&vec![]).iter().map(|tp| {
                        format!(
                            "Trader: {}, Price: {} {}",
                            tp["trader"]["name"].as_str().unwrap_or("Unknown"),
                            tp["price"].as_u64().unwrap_or(0),
                            tp["currency"].as_str().unwrap_or("₽")
                        )
                    }).collect();
                    prices.join(", ")
                } else {
                    "Failed to parse response".to_string() // レスポンスの解析に失敗した場合のエラーメッセージ
                }
            },
            Err(_) => "Failed to send request".to_string(), // リクエストの送信に失敗した場合のエラーメッセージ
    }
}
