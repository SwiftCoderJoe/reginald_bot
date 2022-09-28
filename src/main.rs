extern crate rand;

use std::env;

use rand::Rng;
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

        // One in forty chance of ranomly sending a gifreply
        if msg.author.id == 289158192955392001 && rand::random::<f64>() < 0.025 {
            reply_true_false(ctx, &msg).await;
        }
    }
}

async fn reply_true_false(ctx: Context, msg: &Message) {

    let reply = match rand::thread_rng().gen_range(1..=15) {
        1..=7 => "true",
        8..=14 => "false",
        15 => "perhaps",
        _ => "Unknown random number generated. Report this to SwiftCoderJoe."
    };

    if let Err(why) = msg.reply(
        &ctx.http,
        reply
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