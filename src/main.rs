use std::env;
use std::process::Command;
use std::process::Output;
use serde_json::json;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::collections::HashMap;
use regex::Regex;
use serenity::http::CacheHttp;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
		let mut args = HashMap::new();

		args.insert(
			"ID".to_string(),
			msg.id.as_u64().to_string()
		);

		args.insert(
			"AUTHOR_ID".to_string(),
			msg.author.id.as_u64().to_string()
		);

		args.insert(
			"AUTHOR_DISCRIMINATOR".to_string(),
			msg.author.discriminator.to_string()
		);

		args.insert(
			"AUTHOR_NAME".to_string(),
			msg.author.name
		);

		args.insert(
			"CHANNEL_ID".to_string(),
			msg.channel_id.as_u64().to_string()
		);

		args.insert(
			"CONTENT".to_string(),
			msg.content
		);

		let output = run_command("message", args, ctx).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn run_command(script_name: &str, args: HashMap<String, String>, ctx: Context) {
	let output = if cfg!(target_os = "windows") {
		// windows LOL
		Command::new("cmd")
				.args(&["/C", "echo hello"])
				.output()
				.expect("failed to execute process")
	} else {
		Command::new("sh")
			.arg(script_name)
			.envs(args)
			.output()
            .expect("failed to execute process")
	};

	if output.status.success() {
		handle_output(output, ctx).await;
	} else {
		println!("ERROR {:?}", output);
	}
}

async fn handle_output(output: Output, ctx: Context) {
	let mut stdout = String::from_utf8(output.stdout).unwrap();

	if stdout.is_empty() {
		return;
	}

	stdout.pop();

	let commands = stdout.split("\n");

	for command in commands {
		let regex = Regex::new(r"^SEND_MESSAGE (\d*) (.*)$").unwrap();

		let matches = regex.captures(command).unwrap();

		let json = json!({
			"content": matches.get(2).map_or("", |m| m.as_str())
		});

		ctx.http().send_message(
			matches.get(1).map_or("", |m| m.as_str()).parse::<u64>().unwrap(),
			&json
		).await;
	}
}