use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

/// Constants
const PREFIX: &str = "!";

/// Bot handlers
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == format!("{}ping", PREFIX) {
            if let Err(status) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", status);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} online!", ready.user.name);
    }
}

/// Run
#[tokio::main]
async fn main() {
    println!("The awesome Discord bot is now starting.");
    println!("Getting token...");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    println!("Starting bot...");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating bot");

    if let Err(status) = client.start().await {
        println!("Error starting bot: {:?}", status);
    }
}
