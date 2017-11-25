extern crate regex;
extern crate rand;

use std::fs::File;
use std::io::Read;
use self::regex::Regex;
use std::path::PathBuf;
use std::string::*;
use serenity::utils::Colour;
use serenity::utils::builder::CreateEmbedFooter;
use serenity::client::CACHE;
use serenity::utils::parse_quotes;
use chrono::*;
use rand::distributions::{IndependentSample, Range};
use std::path::Path;

pub fn replace(replaced: &str, replacement: &str,replacing: &str) -> String {
	let re = match Regex::new(replaced) {
		Ok(val)  => val,
		Err(err) => return format!("{}",err),
	};
	return re.replace_all(replacement, replacing).to_string();
}

pub fn read_to_string(filename: &PathBuf) -> String {
	let mut rust = File::open(filename).expect("opening file");
	let mut rustext = String::new();
	rust.read_to_string(&mut rustext).expect("reading file");
	return rustext.to_string();
}

pub fn gnu_replacement(content: Vec<String>) -> String {
	// This is badly made, should replace both things at the same time
	let mut first: &str;
	let mut second: &str;
	if &content.len()==&1 {
		first = &content[0];
		second = "";
	}
	else if &content.len()==&2 {
		first = &content[1];
		second = &content[0];
	}
	else {
		first = "";
		second = "";
	}
	let mut gnu = File::open("pastas/gnu.txt").expect("opening file");
	let mut gnutext = String::new();
	gnu.read_to_string(&mut gnutext).expect("reading file");
	if second == "" {
		second = "GNU";
	}
	if first == "" {
		first = "Linux";
	}
	let replacing_gnu = replace("GNU", &gnutext, &second);
	let replacing_linux = replace("Linux", &replacing_gnu, &first);
	return replace("`", &replacing_linux, "");
}

command!(gnu(_context, msg, args) {
	// Embeded Gnu text with custom GNU and Linux
	let _ = msg.channel_id.send_message(|m| m
		.embed(|e| e
		.thumbnail("https://raw.githubusercontent.com/LelCP/altego/master/images/interjection.png")
		.description(&format!("```{}```",&gnu_replacement(parse_quotes(&args.full()))))
		));
});

command!(god(_ctx, msg) {
	// Show rms quotes from ./pastas/stallman.txt
	let mut count = 0;
	let between = Range::new(-1, 47);
	let mut rng = rand::thread_rng();
	let a = between.ind_sample(&mut rng);
	let mut lineset = format!(" ").to_string();
	if Path::new("pastas/stallman.txt").exists() == true {
		let mut file = File::open("pastas/stallman.txt").expect("opening file");
		let mut text = String::new();
		file.read_to_string(&mut text).expect("reading file");
		for line in text.lines() {
			if a == count{	
			
				lineset = format!("{}", line);
				count += 1;
			}
			else {
				count += 1;
			}
			
		}
		
	}

		let colour = Colour::from_rgb(153, 31, 163);
		let mut footer = CreateEmbedFooter::default()
			.text("STALLMAN RULEZ")
			.icon_url("https://upload.wikimedia.org/wikipedia/commons/thumb/2/22/Heckert_GNU_white.svg/220px-Heckert_GNU_white.svg.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(&format!("He has spoken!"))
			.colour(colour)
			.footer(|_| footer)
			.thumbnail("http://i1-news.softpedia-static.com/images/news2/Richard-Stallman-Says-He-Created-GNU-Which-Is-Called-Often-Linux-482416-2.jpg")
			.description(&format!("{}",&lineset))
			.url("https://stallman.org/")
			));
});
command!(donkey(_ctx, msg) {
	// Show information about server on which bot in run
	let user_id = CACHE.read().unwrap().user.id;
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			let _= msg.channel_id.send_message(|m| m.content(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let now = Local::now();
	let dt=format!("{}", now.format("Kong! %Y-%m-%d %H:%M:%S").to_string());
	let _= msg.channel_id.send_message(|m| m.content(&format!("{}, <@!{}>", dt,user_id )));
	match guild_id.edit_nickname(Some("ԀƆ˥")) {
		Ok(val)  => val,
		Err(err) => return Err(err.into()),
	};
});
command!(roll(_ctx, msg, args) {
	if args.len() == 0 {
		let between = Range::new(1, 7);
		let mut rng = rand::thread_rng();
		let random = between.ind_sample(&mut rng);
		let _= msg.channel_id.send_message(|m| m.content(format!("{} is the answer <@{}>", random, msg.author.id)));
	}
	else if args.len() == 1 {
	let finish: u64 = match args[0].parse() {
		Ok(val)  => val,
		Err(_err) => 6,
	};
		let between = Range::new(1, finish);
		let mut rng = rand::thread_rng();
		let random = between.ind_sample(&mut rng);
		let _= msg.channel_id.send_message(|m| m.content(format!("{} is the answer <@{}>", random, msg.author.id)));
	}
	else if args.len() == 2 {
	let start: u64 = match args[0].parse() {
		Ok(val)  => val,
		Err(_err) => 6,
	};
	let finish: u64 = match args[1].parse() {
		Ok(val)  => val,
		Err(_err) => 6,
	};
		let between = Range::new(start, finish+1);
		let mut rng = rand::thread_rng();
		let random = between.ind_sample(&mut rng);
		let _= msg.channel_id.send_message(|m| m.content(format!("{} is the answer <@{}>", random, msg.author.id)));
	}
	else {
		let between = Range::new(0, args.len());
		let mut rng = rand::thread_rng();
		let random = between.ind_sample(&mut rng);
		let _= msg.channel_id.send_message(|m| m.content(format!("{} is the answer <@{}>", args[random], msg.author.id)));
	}
});
