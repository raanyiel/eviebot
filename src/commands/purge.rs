use serenity::{
 all::standard::Args, framework::standard::{macros::command, CommandResult}, futures::StreamExt, model::channel::Message, prelude::*,
};


#[command]
#[description = "Deletes a specified number of messages."]
#[usage = "<number_of_messages>"]
#[required_permissions(MANAGE_MESSAGES)]
async fn purge(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let num_messages: i32 = args.single::<i32>()?;
    if num_messages > 100 || num_messages < 0 {
        msg.channel_id.say(&ctx.http, "Nuh uh.").await?;
        return Ok(())
    }
    let history = msg.channel_id.messages_iter(&ctx).boxed();
    let mut messages = history.take(num_messages as usize + 1);
    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => message.delete(&ctx.http).await.unwrap_or_else(|err| eprintln!("meow: {}", err)),
            Err(_error) => eprintln!("what"),
        }
    }

    Ok(())
}
