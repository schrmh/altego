extern crate serenity;
extern crate regex;
extern crate rand;
extern crate json;

use std::io::prelude::*;
use serenity::utils::parse_quotes;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::fs::DirBuilder;
use std::string::*;
use serenity::client::CACHE;
use std::fs;
use std::env;

command!(clear(_context, msg, args) {
	if args.len() == 1 {
		let countdown: u64 = args.find().unwrap_or_default();
		for vec in msg.channel_id.messages(|g| g.before(msg.id).limit(countdown)) {
				let mut vec_id = Vec::new();
				for message in vec {
					vec_id.push(message.id);
				}
				vec_id.push(msg.id);
				match msg.channel_id.delete_messages(vec_id.as_slice()) {
					Ok(val)  => val,
					Err(_err) => (),
				};
		}
		let _=msg.channel_id.send_message(|m| m.content(format!("Deleted {} messages",countdown)));
	}
	else if args.len() == 2 {
		let countdown: u64 = args.find().unwrap_or_default();
		let counter: u64 = args.find().unwrap_or_default();
		let full = countdown + counter;
		for vec in msg.channel_id.messages(|g| g.before(msg.id).limit(full)) {
				let mut vec_id = Vec::new();
				let mut i = 0;
				for message in vec.iter().rev() {
					if i < countdown {
						vec_id.push(message.id);
					}
					i += 1;
				}
				vec_id.push(msg.id);
				match msg.channel_id.delete_messages(vec_id.as_slice()) {
					Ok(val)  => val,
					Err(_err) => (),
				};
		}
		let _=msg.channel_id.send_message(|m| m.content(format!("Deleted {} messages",countdown)));
	}
});

command!(ccommand(_context, msg, args) {
	let arg_vec = parse_quotes(&args.full());
	let mut image = "".to_string();
	if arg_vec.len() == 1 {
		let alias = &arg_vec[0].to_lowercase();
		let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
			Some(channel) => channel.read().unwrap().guild_id,
			None => {
				let _ = msg.channel_id.send_message(|m| m.content("Groups and DMs not supported"));
				return Ok(());
			},
		};
		for attachment in msg.clone().attachments {
			if attachment.url.contains("jpg") || attachment.url.contains("png") || attachment.url.contains("jpeg") || attachment.url.contains("gif") {
				image = attachment.url;
			}
		}
		if !Path::new(&format!("{}/.lcpae/commands/{}",env::home_dir().unwrap().display().to_string(), guild_id)).exists() {
			DirBuilder::new()
				.recursive(true)
				.create(format!("{}/.lcpae/commands/",env::home_dir().unwrap().display().to_string())).unwrap();
			DirBuilder::new()
				.recursive(true)
				.create(&format!("{}/.lcpae/commands/{}/",env::home_dir().unwrap().display().to_string(), guild_id)).unwrap();
		}
		let file = File::create(&format!("{}/.lcpae/commands/{}/{}.json",env::home_dir().unwrap().display().to_string(), guild_id, alias)).unwrap();
		file.set_len(0).unwrap();
		let data = object!{
	   		"text" => "",
	    		"image" => image
		};
		let mut writer = BufWriter::new(&file);
			writer.write_all(data.dump().as_bytes()).unwrap();;
	}
	else if arg_vec.len() == 2 {
		let alias = &arg_vec[0].to_lowercase();
		let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
			Some(channel) => channel.read().unwrap().guild_id,
			None => {
				let _ = msg.channel_id.send_message(|m| m.content("Groups and DMs not supported"));
				return Ok(());
			},
		};
		for attachment in msg.clone().attachments {
			if attachment.url.contains("jpg") || attachment.url.contains("png") || attachment.url.contains("jpeg") || attachment.url.contains("gif") {
				image = attachment.url;
			}
		}
		if !Path::new(&format!("{}/.lcpae/commands/{}",env::home_dir().unwrap().display().to_string(), guild_id)).exists() {
			DirBuilder::new()
				.recursive(true)
				.create(format!("{}/.lcpae/commands/",env::home_dir().unwrap().display().to_string())).unwrap();
			DirBuilder::new()
				.recursive(true)
				.create(&format!("{}/.lcpae/commands/{}/",env::home_dir().unwrap().display().to_string(), guild_id)).unwrap();
		}
		let file = File::create(&format!("{}/.lcpae/commands/{}/{}.json",env::home_dir().unwrap().display().to_string(), guild_id, alias)).unwrap();
		file.set_len(0).unwrap();
		let data = object!{
	   		"text" => arg_vec[1].as_str(),
	    		"image" => image
		};
		let mut writer = BufWriter::new(&file);
			writer.write_all(data.dump().as_bytes()).unwrap();;
	}
});

command!(cremove(_context, msg, args) {
	let arg_vec = parse_quotes(&args.full());
	if arg_vec.len() == 1 {
		let alias = &arg_vec[0].to_lowercase();
		let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
			Some(channel) => channel.read().unwrap().guild_id,
			None => {
				let _ = msg.channel_id.send_message(|m| m.content("Groups and DMs not supported"));
				return Ok(());
			},
		};
		if Path::new(&format!("{}/.lcpae/commands/{}/{}.json",env::home_dir().unwrap().display().to_string(), guild_id, alias)).exists() == true {
			fs::remove_file(format!("{}/.lcpae/commands/{}/{}.json",env::home_dir().unwrap().display().to_string(), guild_id, alias)).unwrap();
		}
	}
});
