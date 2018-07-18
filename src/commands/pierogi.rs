extern crate serenity;
extern crate rand;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::fs::DirBuilder;
use std::string::*;
use serenity::client::CACHE;
use rand::distributions::{IndependentSample, Range};
use std::path::PathBuf;
use std::env;
use json;
use commands;

pub fn read_pierogi(userid: &str, serverid: &str) -> u8 {
	let a=0;
	if Path::new(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid)).exists() {
		let path = PathBuf::from(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid));
		let text = commands::misc::read_to_string(&path);
		let parsed = json::parse(&text).unwrap();
		return parsed["pierogi"].as_u8().unwrap();
	}
	return a;
}
pub fn time_pierogi(userid: &str, serverid: &str) -> u64 {
	let a=0;
	if Path::new(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid)).exists() {
		let path = PathBuf::from(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid));
		let text = commands::misc::read_to_string(&path);
		let parsed = json::parse(&text).unwrap();
		return parsed["ptimeout"].as_u64().unwrap();
	}
	return a;
}
pub fn new_pierogi(userid: &str, serverid: &str, pierogi: u8, time: u64){
	let mut verify: u64 = 0;
	if Path::new(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid)).exists() {
		let path = PathBuf::from(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid));
		let text = commands::misc::read_to_string(&path);
		let parsed = json::parse(&text).unwrap();
		verify = parsed["verify"].as_u64().unwrap();
	}
	if !Path::new(&format!("{}/.lcpae/servers/{}", env::home_dir().unwrap().display().to_string(), serverid)).exists() {
		DirBuilder::new()
			.recursive(true)
			.create(&format!("{}/.lcpae/servers/{}/",env::home_dir().unwrap().display().to_string(), serverid)).unwrap();
	}
	let file = File::create(&format!("{}/.lcpae/servers/{}/{}.json",env::home_dir().unwrap().display().to_string(), serverid, userid)).unwrap();
	file.set_len(0).unwrap();
	let data = object!{
   		"pierogi" => pierogi,
    		"ptimeout" => time,
    		"verify" => verify
	};
	let mut writer = BufWriter::new(&file);
		writer.write_all(data.dump().as_bytes()).unwrap();;
}

command!(score(_context, msg) {
	let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You got {} pierogi", &msg.author.id.to_string(), &read_pierogi(&msg.author.id.to_string(), &guild_id.to_string()).to_string())));
});

command!(steal(_context, msg) {
	let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let between = Range::new(0, 4);
	let mut rng = rand::thread_rng();
	let a = between.ind_sample(&mut rng);
	if msg.mentions.len() > 0 {
		let mention = msg.mentions[0].id;
		if read_pierogi(&mention.to_string(),&guild_id.to_string()) > 0 {
			if a<=0 {
				new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), read_pierogi(&msg.author.id.to_string(),&guild_id.to_string()) + 1, time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
				new_pierogi(&mention.to_string(), &guild_id.to_string(), read_pierogi(&mention.to_string(),&guild_id.to_string()) -1, time_pierogi(&mention.to_string(),&guild_id.to_string()));
				let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You stole pierog from <@{}>", msg.author.id, mention)));
			}
			else {
				new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), 0, time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
				let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You lost all your pierogi because you were trying to steal them from <@{}>", msg.author.id, mention)));
			}
		}
		else{
			let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> doesn't have any pierogi, <@{}>", mention, msg.author.id)));
		}
	}
});

command!(give(_context, msg) {
	let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	if msg.mentions.len() > 0 {
		let mention = msg.mentions[0].id;
		if read_pierogi(&msg.author.id.to_string(),&guild_id.to_string()) > 0 {
			new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), read_pierogi(&msg.author.id.to_string(),&guild_id.to_string()) - 1, time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
			new_pierogi(&mention.to_string(), &guild_id.to_string(), read_pierogi(&mention.to_string(),&guild_id.to_string()) + 1, time_pierogi(&mention.to_string(),&guild_id.to_string()));
			let _= msg.channel_id.send_message(|m| m.content(format!("<@{}>, You gave pierog to <@{}>", msg.author.id, mention)));
		}
	}

});
