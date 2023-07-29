use std::env;
use serenity::{Client, prelude::{GatewayIntents, EventHandler}, async_trait, model::prelude::command::Command};

struct Handler;

pub mod commands;
pub mod logic;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("Me ready to be obnoxious.");

        let global_commands = Command::create_global_application_command(&ctx.http, |command| {

        })
        .await;
    }
}


#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Ye dumbass, get me token.");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(event_handler)
        .await
        .expect("Client couldn't be bothered.");

    if let Err(why) = client.start().await {
        println!("Dis succ: {:?}", why);
    }
}
