extern crate dotenv;
use async_trait::async_trait;
use dotenv::dotenv;
use serenity::all::{Message, UserId};
use serenity::{
    framework::standard::{
        help_commands::*, macros::group, macros::help, Args, CommandGroup, CommandResult,
        Configuration, HelpOptions, StandardFramework,
    },
    model::gateway::{GatewayIntents, Ready},
    prelude::*,
};
use std::collections::HashSet;
use std::env;

#[help]
#[individual_command_tip = "Nya <3, here's a list of shit I can do! P.S. commands with <> args are required and commands with () args are optional!"]
#[strikethrough_commands_tip_in_guild = ""]
#[strikethrough_commands_tip_in_dm = ""]
#[lacking_permissions = "hide"]
#[dm_and_guild_text = ""]
#[available_text = ""]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = with_embeds(ctx, msg, args, &help_options, groups, owners).await?;
    Ok(())
}

#[group]
#[commands(avatar, purge)]
struct Utility;
mod commands;
use commands::avatar::*;
use commands::purge::*;

#[group]
#[commands(gay)]
struct Fun;
use commands::gay::*;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let mut framework = StandardFramework::new()
        .group(&UTILITY_GROUP)
        .help(&MY_HELP);
    framework.configure(Configuration::new().prefix("."));
    framework.group_add(&FUN_GROUP);
    let token = env::var("TOKEN").expect("token");
    let intents =
        GatewayIntents::GUILDS | GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

        

    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
