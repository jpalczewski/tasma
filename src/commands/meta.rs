use serenity::framework::standard::{macros::{command, help}, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let data = env::var("APPDATA")?;
    msg.channel_id.say(&ctx.http, data).await?;

    Ok(())
}

#[help]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let help ="test";
    msg.channel_id.say(&ctx.http, help).await?;

    Ok(())
}