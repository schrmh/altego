#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate rand;
extern crate time;
extern crate typemap;

#[macro_use(object)] extern crate json;

use std::string::*;
use serenity::prelude::Mutex;
use std::sync::Arc;
use serenity::model::prelude::*;
use serenity::Client;
use serenity::framework::standard::help_commands;
use serenity::framework::StandardFramework;
use serenity::client::{EventHandler,Context, CACHE};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serenity::model::permissions::Permissions;
use std::path::Path;
use std::path::PathBuf;
use serenity::framework::standard::HelpBehaviour;
use serenity::client::bridge::voice::ClientVoiceManager;
use typemap::Key;

mod commands;

struct VoiceManager;

impl Key for VoiceManager {
    type Value = Arc<Mutex<ClientVoiceManager>>;
}

struct Handler;

impl EventHandler for Handler {
	fn ready(&self, ctx: Context, ready: Ready) {
        	println!("{} is connected!", ready.user.name);
        	//setting "playing" to promote git repo for bot
		ctx.set_game_name("with fire");
	}
	fn message(&self, _: Context, message: Message) {
		if !message.author.bot {
			if message.content.to_ascii_lowercase().contains("thx") || message.content.to_ascii_lowercase().contains("thank"){
				let start = SystemTime::now();
				let mut botmention = false;
	    			let since_the_epoch = start.duration_since(UNIX_EPOCH)
	       			.expect("Time went backwards");
	       			if message.mentions.len() > 0 {
					let guild_id = match CACHE.read().guild_channel(message.channel_id) {
						Some(channel) => channel.read().guild_id,
						None => {
							let _ = message.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
							return ();
						},
					};
				
				if since_the_epoch.as_secs() >= commands::pierogi::time_pierogi(&message.author.id.to_string(),&guild_id.to_string()) {
					let mut msg: String = "".to_string();
						for mention in message.mentions {
							if message.author.id != mention.id {
								msg = format!("{} <@{}>",msg,mention.id);
								let pierogi = commands::pierogi::read_pierogi(&mention.id.to_string(),&guild_id.to_string());
								commands::pierogi::new_pierogi(&mention.id.to_string(), &guild_id.to_string(), pierogi + 1, commands::pierogi::time_pierogi(&mention.id.to_string(),&guild_id.to_string()));
								commands::pierogi::new_pierogi(&message.author.id.to_string(), &guild_id.to_string(), commands::pierogi::read_pierogi(&message.author.id.to_string(),&guild_id.to_string()), since_the_epoch.as_secs() + 7140);
								if mention.id == CACHE.read().user.id {
									botmention = true;
									commands::pierogi::new_pierogi(&message.author.id.to_string(), &guild_id.to_string(), commands::pierogi::read_pierogi(&message.author.id.to_string(),&guild_id.to_string()) + 1, since_the_epoch.as_secs() + 7140);
								}
							}
						}
						if msg != "".to_string() {
							if botmention {
								let _ = message.channel_id.send_message(|m| m.content(&format!("You recived thank you pieróg {}. Oh wait, pierogi for me? ***THANK YOU SO MUCH, YOU TOO GET A PIERÓG***",msg)));
							}
							else {
								let _ = message.channel_id.send_message(|m| m.content(&format!("You recived thank you pieróg {}",msg)));
							}
						}
					}
					else {
						let _ = message.channel_id.send_message(|m| m.content(format!("<@{}>, I know your mommy told you to thank as much as you can, but this is too much",message.author.id)));
					}
				}
			}
			if Path::new(&format!("{}/.lcpae/commands/{}/{}.json", env::home_dir().unwrap().display().to_string(), guild_id, message.content)).exists() {
				/*
				Custom commands:
				even though most of the bot will be moving to db, this part will be json for simplicity's sake
				*/
				let path = PathBuf::from(&format!("{}/.lcpae/commands/{}/{}.json", env::home_dir().unwrap().display().to_string(), guild_id, message.content));
				let text = commands::misc::read_to_string(&path);
				let parsed = json::parse(&text).unwrap();
				// image and text parts should be connected
				if parsed["image"].as_str().unwrap() != "" {
					let _ = message.channel_id.send_message(|m| m
						.embed(|e| e
						.image(parsed["image"].as_str().unwrap())
					));
				}
				if parsed["text"].as_str().unwrap() != "" {
					let _ = message.channel_id.send_message(|m| m
						.embed(|e| e
						.description(parsed["text"].as_str().unwrap())
					));
				}
			}
		}
	}
}

fn main() {
	
	let token = env::var("DISCORD_TOKEN")
		.expect("Expected a token in the environment");

	let mut client = Client::new(&token, Handler).expect("Err creating client");

	{
        	let mut data = client.data.lock();
        	data.insert::<VoiceManager>(Arc::clone(&client.voice_manager));
	}

	client.with_framework(StandardFramework::new()
		.before(|_ctx, msg, _cmd_name| {
			//inform of every command (except custom command, karma (should be fixed))
			println!("{} in {}: {}", msg.author.id, msg.channel_id, msg.content);
			true
		})
		.after(|_ctx, _msg, cmd_name, error| {
			//post error if one happened
			if let Err(why) = error {
				println!("Error in {}: {:?}", cmd_name, why);
			}
		})
		.bucket("basic", 5, 60, 3)
		.configure(|c| c
			.prefix("!")
			.on_mention(true)
			.case_insensitivity(true))
		.customised_help(help_commands::with_embeds, |c| {
		        c.individual_command_tip("Hi, I didn't see you there\n\
		        If you want more information about a specific command, just pass the command as argument.")
		        .suggestion_text("Did you mean {} command?")
		        .lacking_permissions(HelpBehaviour::Hide)
		        .lacking_role(HelpBehaviour::Nothing)
		        .wrong_channel(HelpBehaviour::Strike)
			})
		.group("Miscellaneous", |g| g
			.command("gnu", |c| c
				.desc("GNU Interjection copypasta")
				.usage("<GNU replacement> <Linux replacement>")
				.cmd(commands::misc::gnu))
			.command("roll", |c| c
				.desc("Rolling is fucking complex, I have no idea how to describe it shortly")
				.cmd(commands::misc::roll))
			.command("godtellmeyourways", |c| c
				.desc("Quote God himself, Richard Matthew Stallman")
				.cmd(commands::misc::god))
			.command("donkey", |c| c
				.desc("Show current bot time and set nickname and playing message")
				.cmd(commands::misc::donkey)))
		.group("Useful shit", |g| g
			.command("ddg", |c| c
				.desc("Powerful search with embeds")
				.usage("<search terms>")
				.cmd(commands::useful::ddgsearch))
			.command("emoji", |c| c
				.desc(":regional_indicator_s::regional_indicator_p::regional_indicator_e::regional_indicator_a::regional_indicator_k:   :regional_indicator_i::regional_indicator_n:   :regional_indicator_e::regional_indicator_m::regional_indicator_o::regional_indicator_j::regional_indicator_i:")
				.usage("<your text>")
				.bucket("basic")
				.cmd(commands::useful::emoji))
			.command("info", |c| c
				.desc("Distro database")
				.usage("<distro name>")
				.cmd(commands::useful::info))
			.command("wget", |c| c
				.desc("Sends wget script")
				.usage("<amount of messages in hundreds to download from channel>")
				.cmd(commands::useful::wget))
			.command("clist", |c| c
				.desc("List custom commands")
				.cmd(commands::useful::clist)))
		.group("About bot", |g| g
			.command("github", |c| c
				.desc("Shilling open source nature of this project")
				.cmd(commands::about::github))
			.command("rust", |c| c
				.desc("Shilling rust nature of this project")
				.cmd(commands::about::rust)))
		.group("Admin", |g| g
			.required_permissions(Permissions::ADMINISTRATOR)
			.command("clear", |c| c
				.desc("Clean previous messages")
				.cmd(commands::admin::clear))
			.command("cremove", |c| c
				.desc("Remove custom command")
				.cmd(commands::admin::cremove))
			.command("ccommand", |c| c
				.desc("Custom commands")
				.cmd(commands::admin::ccommand)))
		.group("Pierogi", |g| g
			.prefix("pierog")
			.command("score", |c| c
				.desc("Check your own score")
				.cmd(commands::pierogi::score))
			.command("steal", |c| c
				.desc("Steal pierogi, but be careful")
				.cmd(commands::pierogi::steal))
			.command("give", |c| c
				.desc("Steal pierogi, but be careful")
				.cmd(commands::pierogi::give)))
		.group("Voice commands", |g| g
			.prefix("voice")
			.cmd("deafen", commands::voice::deafen)
        		.cmd("join", commands::voice::join)
        		.cmd("leave", commands::voice::leave)
        		.cmd("mute", commands::voice::mute)
        		.cmd("play", commands::voice::play)
        		.cmd("ping", commands::voice::ping)
        		.cmd("undeafen", commands::voice::undeafen)
			.cmd("unmute", commands::voice::unmute)));

	if let Err(why) = client.start() {
		println!("Client error: {:?}", why);
	}
}
