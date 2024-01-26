use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let user = msg.mentions.first().unwrap_or(&msg.author);
    let avatar_url = user.face();

    msg.channel_id.say(&ctx.http, avatar_url).await?;

    Ok(())
}