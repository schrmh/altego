extern crate serenity;
extern crate rand;

use commands;

use std::string::*;
use serenity::client::CACHE;
use rand::distributions::{IndependentSample, Range};

command!(score(_context, msg) {
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You got {} pierogi", &msg.author.id.to_string(), &commands::db::read_pierogi(&msg.author.id.to_string(), &guild_id.to_string()).to_string())));
});

command!(steal(_context, msg) {
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let between = Range::new(-1, 100);
	let mut rng = rand::thread_rng();
	let a = between.ind_sample(&mut rng);
	if msg.mentions.len() > 0 {
		let mention = msg.mentions[0].id;
		let pierogi = commands::db::read_pierogi(&mention.to_string(),&guild_id.to_string());
		if a<=0 {
			commands::db::new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), pierogi + 1, commands::db::time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
			commands::db::new_pierogi(&mention.to_string(), &guild_id.to_string(), pierogi - 1, commands::db::time_pierogi(&mention.to_string(),&guild_id.to_string()));
			let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You stole pierog from <@{}>", msg.author.id, mention)));
		}
		else {
			commands::db::new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), 0, commands::db::time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
			let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You lost all your pierogi because you were trying to steal them from <@{}>", msg.author.id, mention)));
		}
	}
});
