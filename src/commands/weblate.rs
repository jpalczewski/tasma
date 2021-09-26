use serenity::framework::standard::{macros::{command, help}, 
    CommandResult, help_commands};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::env;
use reqwest::{Client, header};

#[command]
#[aliases("s")]
#[description = "Checks weblate health, shows number of pending changes"]
async fn status(ctx: &Context, msg: &Message) -> CommandResult {
    let data = "status";
    let url = format!("{}/api/projects/{}/repository/", env::var("WEBLATE_HOST").expect("missing weblate url"), 
        env::var("WEBLATE_PROJECT").expect("missing project name from env") );

    let client = get_weblate_client();

    let res = client.get(url).send().await?.text().await?;
    let text = format!("{:#?}", res);
    msg.channel_id.say(&ctx.http, &text).await?;

    Ok(())
}


#[command]
async fn update(ctx: &Context, msg: &Message) -> CommandResult {
    let data = "update";

    msg.channel_id.say(&ctx.http, data).await?;


    Ok(())
}


fn get_weblate_client() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    let mut token = header::HeaderValue::from_str(&format!("Token {}",
    env::var("WEBLATE_TOKEN").expect("missing weblate token"))).expect("creating token failed");
    token.set_sensitive(true);
    headers.insert("Authorization", token);
    let client = reqwest::Client::builder()
    .default_headers(headers)
    .build();
    client.expect("creating client failed")
}