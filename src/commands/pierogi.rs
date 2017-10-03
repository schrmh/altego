extern crate serenity;
extern crate rand;

use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::fs::DirBuilder;
use std::string::*;
use serenity::client::CACHE;
use rand::distributions::{IndependentSample, Range};

fn lines_from_file<P>(filename: P) -> Vec<String>
where
	P: AsRef<Path>,
{
	let file = File::open(filename).expect("no such file");
	let buf = BufReader::new(file);
	buf.lines()
		.map(|l| l.expect("Could not parse line"))
		.collect()
}

pub fn read_pierogi(userid: &str, serverid: &str) -> i8 {
	let a=0;
	if Path::new(&format!("servers/{}/{}.txt", serverid, userid)).exists() {
		let lines = lines_from_file(format!("servers/{}/{}.txt", serverid, userid));
		return lines[0].parse::<i8>().unwrap();
	}
	return a;
}
pub fn time_pierogi(userid: &str, serverid: &str) -> u64 {
	let a=0;
	if Path::new(&format!("servers/{}/{}.txt", serverid, userid)).exists() {
		let lines = lines_from_file(format!("servers/{}/{}.txt", serverid, userid));
		return lines[1].parse::<u64>().unwrap();
	}
	return a;
}
pub fn new_pierogi(userid: &str, serverid: &str, pierogi: i8, time: u64){
	let save = format!("{}\n{}", pierogi, time);
	if !Path::new(&format!("servers/{}/", serverid)).exists() {
		DirBuilder::new()
			.recursive(true)
			.create(&format!("servers/{}/", serverid)).unwrap();
	}
	let file = File::create(&format!("servers/{}/{}.txt", serverid, userid)).unwrap();
	file.set_len(0).unwrap();
	let mut writer = BufWriter::new(&file);
		writer.write_all(save.as_bytes()).unwrap();;
}

command!(score(_context, msg) {
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You got {} pierogi", &msg.author.id.to_string(), &read_pierogi(&msg.author.id.to_string(), &guild_id.to_string()).to_string())));
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
		let pierogi = read_pierogi(&mention.to_string(),&guild_id.to_string());
		if a<=0 {
			new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), pierogi + 1, time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
			new_pierogi(&mention.to_string(), &guild_id.to_string(), pierogi - 1, time_pierogi(&mention.to_string(),&guild_id.to_string()));
			let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You stole pierog from <@{}>", msg.author.id, mention)));
		}
		else {
			new_pierogi(&msg.author.id.to_string(), &guild_id.to_string(), 0, time_pierogi(&msg.author.id.to_string(),&guild_id.to_string()));
			let _= msg.channel_id.send_message(|m| m.content(format!("<@{}> You lost all your pierogi because you were trying to steal them from <@{}>", msg.author.id, mention)));
		}
	}
});
