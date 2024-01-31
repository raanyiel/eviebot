use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use rand::Rng;


fn gen() -> i8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..101)
}

#[command]
#[description = "Calculates using trans magic to identify how gay you are! <3"]
#[usage = ".gay"]
async fn gay(ctx: &Context, msg: &Message) -> CommandResult {
    let r = gen();
    let c = format!("You are {}% gay!", r);
    msg.channel_id.say(&ctx.http, c).await?;

    Ok(())
}
