#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate rand;
extern crate time;

use std::string::*;
use serenity::model::*;
use serenity::Result as SerenityResult;
use serenity::Client;
use serenity::framework::standard::help_commands;
use serenity::framework::StandardFramework;
use serenity::client::{EventHandler,Context, CACHE};
use std::env;
use std::ascii::AsciiExt;
use std::time::{SystemTime, UNIX_EPOCH};

mod commands;

struct Handler;

impl EventHandler for Handler {
    fn on_ready(&self, ctx: Context, ready: Ready) {
        	println!("{} is connected!", ready.user.name);
		ctx.set_game_name("lelcp.github.io/bot");
	}
	fn on_message(&self, _: Context, message: Message) {
            if message.content.to_ascii_lowercase().contains("thanks") {
			let guild_id = match CACHE.read().unwrap().guild_channel(message.channel_id) {
				Some(channel) => channel.read().unwrap().guild_id,
				None => {
					check_msg(message.channel_id.say(&"Groups and DMs not supported"));
					return ();
				},
			};
			let start = SystemTime::now();
    			let since_the_epoch = start.duration_since(UNIX_EPOCH)
       			.expect("Time went backwards");
			if since_the_epoch.as_secs() >= commands::db::time_pierogi(&message.author.id.to_string(),&guild_id.to_string()) {
				let mut msg: String = "".to_string();
				for mention in message.mentions {
					if message.author.id != mention.id {
						msg = format!("{} <@{}>",msg,mention.id);
						let pierogi = commands::db::read_pierogi(&mention.id.to_string(),&guild_id.to_string());
						commands::db::new_pierogi(&mention.id.to_string(), &guild_id.to_string(), pierogi + 1, commands::db::time_pierogi(&mention.id.to_string(),&guild_id.to_string()));
						commands::db::new_pierogi(&message.author.id.to_string(), &guild_id.to_string(), commands::db::read_pierogi(&message.author.id.to_string(),&guild_id.to_string()), since_the_epoch.as_secs() + 28740);
					}
				}
				if msg != "".to_string() {
					check_msg(message.channel_id.say(&format!("You recived thank you pieróg {}",msg)));
				}
			}
			else {
				check_msg(message.channel_id.say("I know your mommy told you to thank as much as you can, but this is too much"));
			}
		}
	}
}

fn main() {
	
	let token = env::var("DISCORD_TOKEN")
		.expect("Expected a token in the environment");
	let mut client = Client::new(&token, Handler);
	client.with_framework(StandardFramework::new()
		.bucket("basic", 5, 60, 3)
		.configure(|c| c
			.prefix("!")
			.on_mention(true))
		.group("Miscellaneous", |g| g
			.command("gnu", |c| c
				.desc("GNU Interjection copypasta")
				.usage("<GNU replacement> <Linux replacement>")
				.exec(commands::misc::gnu))
			.command("whiteface", |c| c
				.desc("This is not the gif on the internet")
				.exec(commands::misc::whiteface))
			.command("godtellmeyourways", |c| c
				.desc("Quote God himself, Richard Matthew Stallman")
				.exec(commands::misc::god))
			.command("donkey", |c| c
				.desc("Show current bot time and set nickname and playing message")
				.exec(commands::misc::donkey)))
		.group("Useful shit", |g| g
			.command("ddg", |c| c
				.desc("Powerful search with embeds")
				.usage("<search terms>")
				.exec(commands::useful::ddg))
			.command("emoji", |c| c
				.desc(":regional_indicator_s::regional_indicator_p::regional_indicator_e::regional_indicator_a::regional_indicator_k:   :regional_indicator_i::regional_indicator_n:   :regional_indicator_e::regional_indicator_m::regional_indicator_o::regional_indicator_j::regional_indicator_i:")
				.usage("<your text>")
				.bucket("basic")
				.exec(commands::useful::emoji))
			.command("info", |c| c
				.desc("Distro database")
				.usage("<distro name>")
				.exec(commands::useful::info))
			.command("wget", |c| c
				.desc("Sends wget script")
				.usage("<amount of messages in hundreds to download from channel>")
				.exec(commands::useful::wget)))
		.group("About bot", |g| g
			.command("github", |c| c
				.desc("Shilling open source nature of this project")
				.exec(commands::about::github))
			.command("rust", |c| c
				.desc("Shilling rust nature of this project")
				.exec(commands::about::rust))
			.command("lcpae", |c| c
				.exec_help(help_commands::with_embeds)))
		.group("Tux", |g| g
			.command("roasted", |c| c
				.desc("Daaaaaamn!")
				.exec(commands::tux::roasted))
			.command("hypertux", |c| c
				.desc("Laugh all you want")
				.exec(commands::tux::hypertux))
			.command("hyperthink", |c| c
				.desc("TFW hacked mainframe")
				.exec(commands::tux::hyperthink)))
		.group("Pierogi", |g| g
			.prefix("pierog")
			.command("score", |c| c
				.desc("Check your own score")
				.exec(commands::pierogi::score))
			.command("steal", |c| c
				.desc("Steal pierogi, but be careful")
				.exec(commands::pierogi::steal))));
	let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}


fn check_msg(result: SerenityResult<Message>) {
	if let Err(why) = result {
		println!("Error sending message: {:?}", why);
	}
}
