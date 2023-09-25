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
            items(name: "MoonShine") {
                id
                name
                avg24hPrice
            }
        }
    "#;

    let client = reqwest::Client::new();
    match client.post("https://api.tarkov.dev/graphql")
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"query": "{}"}}"#, query))
        .send()
        .await {
            Ok(response) => {
                if let Ok(json) = response.json::<Value>().await {
                    json["data"]["items"][0]["avg24hPrice"].to_string()
                } else {
                    "Failed to parse response".to_string()
                }
            },
            Err(_) => "Failed to send request".to_string(),
    }
}
