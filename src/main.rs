extern crate serenity;
extern crate reqwest;
extern crate serde_json;

use serenity::{
    async_trait,
    framework::standard::StandardFramework,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serde_json::Value;
use dotenv::dotenv;
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!btc" {
            let price = get_btc_price().await;
            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Current BTC Price: {}", price)).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(StandardFramework::new())
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
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
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "query": query
    });

    match client.post("https://api.tarkov.dev/graphql")
        .header("Content-Type", "application/json")
        .json(&body)  // bodyをJSONとして送信する
        .send()
        .await {
            Ok(response) => {
                if let Ok(json) = response.json::<Value>().await {
                    println!("{:?}", json);
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
                    "Failed to parse response".to_string()
                }
            },
            Err(_) => "Failed to send request".to_string(),
    }
}
