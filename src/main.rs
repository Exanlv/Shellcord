use std::env;
use std::process::Command;
use std::process::Output;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, _ctx: Context, msg: Message) {
		let mut args = Vec::new();
		args.push(format!("ID=\"{}\"", msg.id.as_u64()));
		args.push(format!("AUTHOR_ID=\"{}\"", msg.author.id.as_u64()));
		args.push(format!("AUTHOR_DISCRIMINATOR=\"{}\"", msg.author.discriminator));
		args.push(format!("AUTHOR_NAME=\"{}\"", msg.author.name));
		args.push(format!("CHANNEL_ID=\"{}\"", msg.channel_id.as_u64()));
		args.push(format!("CONTENT=\"{}\"", msg.content));

		let output = run_command("message", args, ctx).await;
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

async fn run_command(script_name: &str, args: Vec<String>, ctx: Context) {
	let output = if cfg!(target_os = "windows") {
		// windows LOL
		Command::new("cmd")
				.args(&["/C", "echo hello"])
				.output()
				.expect("failed to execute process")
	} else {
		Command::new("bash")
			.arg(script_name)
			.args(args)
			.output()
            .expect("failed to execute process")
	};

	if output.status.success() {
		handle_output(output).await;
	} else {
		println!("ERROR {:?}", output);
	}
}

async fn handle_output(output: Output, ctx: Context) {
	let mut stdout = String::from_utf8(output.stdout).unwrap();

	stdout.pop();

	let commands = stdout.split("\n");

	for command in commands {
		println!("{:?}", command);
	}
}