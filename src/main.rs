extern crate rand;

use std::env;

use serenity::{
    async_trait,
    model::{channel::Message,},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if let Some(ref reference) = msg.referenced_message {
            if msg.content == "rb force" {
                if let Err(why) = msg.delete(&ctx.http).await {
                    println!("Error deleting message: {:?}", why)
                }
                reply_true_false(ctx, reference).await;
                return;
            }
        }

        if msg.author.id == 289158192955392001 && rand::random::<f64>() < 0.1 {
            reply_true_false(ctx, &msg).await;
        }
    }
}

async fn reply_true_false(ctx: Context, msg: &Message) {
    if let Err(why) = msg.reply(
        &ctx.http, 
        if rand::random::<f64>() < 0.5 { "true" } else { "false" }
    ).await {
        println!("Error sending message: {:?}", why)
    };
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}