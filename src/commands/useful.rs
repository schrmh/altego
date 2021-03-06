extern crate serenity;
extern crate rand;
extern crate ddg;
extern crate glob;
extern crate json;

use commands;

use serenity::utils::Colour;
use self::ddg::Query;
use serenity::builder::CreateEmbedFooter;
use rand::distributions::{IndependentSample, Range};
use self::glob::glob;
use std::fs::{File,OpenOptions};
use std::path::Path;
use std::io::{BufWriter};
use std::fs;
use std::io::prelude::*;
use std::env;
use serenity::client::CACHE;

command!(ddgsearch(_context, msg) {
	let mut welp = commands::misc::replace("!ddg ", &msg.content, "");
	if welp == "" || welp == " " {
		welp = "!ddg".to_string();
	}
	let mut query: Query;
	let mut help = welp.to_string();
	if welp.to_string().len() > 500 {
		
		help = "buffer overflow".parse().unwrap_or_default();
		query = Query::new(help.clone(), "lcpapp".to_string()).no_html();
	}
	else {
		query = Query::new(help.clone(), "lcpapp".to_string()).no_html();
	}
	let duckurl = format!("http://duckduckgo.com/?q={}", &help);
	let response = query.execute().unwrap();
	let colour = Colour::from_rgb(153, 31, 163);
	let new = commands::misc::replace(" " ,&duckurl, "+");
	if response.definition_url != ""{
		let mut footer = CreateEmbedFooter::default()
			.text(&new)
			.icon_url("https://duckduckgo.com/assets/icons/meta/DDG-icon_256x256.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(&format!("Based on {} article",&response.definition_source))
			.footer(|_| footer)
			.colour(colour)
			.thumbnail(&response.image)
			.description(&format!("{}\n\n*Read more:* <{}>",&response.definition,&response.definition_url))
			.url(&new)
			));
	}
	else if response.abstract_url != ""{
		let mut footer = CreateEmbedFooter::default()
			.text(&new)
			.icon_url("https://duckduckgo.com/assets/icons/meta/DDG-icon_256x256.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(&format!("Based on {} article",&response.abstract_source))
			.colour(colour)
			.footer(|_| footer)
			.thumbnail(&response.image)
			.description(&format!("{}\n\n*Read more:* <{}>",&response.abstract_text,&response.abstract_url))
			.url(&new)
			));
	}
	else if response.redirect != ""{
		let mut footer = CreateEmbedFooter::default()
			.text(&new)
			.icon_url("https://duckduckgo.com/assets/icons/meta/DDG-icon_256x256.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title("Redirect!")
			.footer(|_| footer)
			.colour(colour)
			.thumbnail(&response.image)
			.description(&format!("{}",&response.redirect))
			.url(&new)
			));
	}
	else {
		let mut footer = CreateEmbedFooter::default()
			.text(&new)
			.icon_url("https://duckduckgo.com/assets/icons/meta/DDG-icon_256x256.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(&format!("Results from :duck::duck::goal: for query \"{}\"",&help))
			.footer(|_| footer)
			.colour(colour)
			.url(&new)
			));
}
});

command!(emoji(_context, msg, args) {
	// shows emoji text replacing original message with it
	match msg.delete() {
		Ok(val)  => val,
		Err(_err) => (),
	};
	let mut hello = "".to_string();
	let mut world = "".to_string();
	let mut woman = "".to_string();
	let mut b = 0;
	for arg in args.full().split(' ') {
		if !arg.contains("@") && !arg.contains("#") && !arg.contains(":"){
			for letter in arg.bytes() {
				let between = Range::new(0, 5);
				let mut rng = rand::thread_rng();
				let mut a = between.ind_sample(&mut rng);
				while a == b {
					a = between.ind_sample(&mut rng);
				}
				if a > 0 {
					woman = format!(":skin-tone-{}:",a);
				}
				if letter >= b'A' && letter <= b'Z' || letter >= b'a' && letter <= b'z' {
			
					hello = format!("{}:regional_indicator_{}:",hello, (letter as char).to_lowercase());
					world = format!("{}:ok_woman:{}",world, woman);
				}
				else if letter >= b'0' && letter <= b'9' {
					match letter {
						b'0' => hello = format!("{}:zero:",hello),
						b'1' => hello = format!("{}:one:",hello),
						b'2' => hello = format!("{}:two:",hello),
						b'3' => hello = format!("{}:three:",hello),
						b'4' => hello = format!("{}:four:",hello),
						b'5' => hello = format!("{}:five:",hello),
						b'6' => hello = format!("{}:six:",hello),
						b'7' => hello = format!("{}:seven:",hello),
						b'8' => hello = format!("{}:eight:",hello),
						b'9' => hello = format!("{}:nine:",hello),
						_ => (),
					}
					world = format!("{}:ok_woman:{}",world, woman);
				}
				else if letter == b' ' {
					hello = format!("{}   ",hello);
					world = format!("{}   ",world);
				}
				else if letter == b'!' {
					hello = format!("{}:exclamation:",hello);
					world = format!("{}:ok_woman:{}",world, woman);
				}
				else if letter == b'?' {
					hello = format!("{}:question:",hello);
					world = format!("{}:ok_woman:{}",world, woman);
				}
				b = a;
			}
			
			
		}
		hello = format!("{}   ",hello);
		world = format!("{}   ",world);
	}
	if !hello.is_empty() && hello.len() < 2000 {
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(&format!("{}",msg.author.name))
			.colour(Colour::from_rgb(0, 80, 80))
			.description(&format!("{}\n{}",hello,world))
		));
	}
});

command!(info(_context, msg, arg) {
	let args: Vec<String> = arg.multiple().unwrap();
	// Should be a module, basically shows informations from folder, about important tech stuff
	if args.len() == 0 {
		let mut list = format!("**You can learn about:**");
		let mut infocommand = "".to_string();
		for entry in glob("info/*").unwrap() {
		match entry {
			Ok(path_dir) => {
			let dir = path_dir.display().to_string().clone();
			let mut split_dir = dir.split('/');
			let folder = split_dir.nth(1).unwrap_or_default();
			if !folder.contains("_") {
				infocommand = format!("{}\n**{}**", &infocommand, &folder);
			}
				for file in glob(&format!("{}/*.json", dir)).unwrap() {
					match file {
						Ok(path_file) => {
							let welp = path_file.display().to_string().clone();
							let mut split = welp.split('/');
							let noextension = commands::misc::replace(".json", &split.nth(2).unwrap_or_default(), "");
							if !noextension.to_string().contains("_") {
								infocommand = format!("{}\n{}", &infocommand, &noextension);
							}
						},
						Err(e) => println!("{:?}", e),
						
					}
				}
			},
			Err(e) => println!("{:?}", e),
			}
		}
	list = format!("{}\n{}",&mut list, infocommand);
	let _=msg.channel_id.send_message(|m| m.content(&list));
	}
	else {
		let mut distro = &args[0];
		for entry in glob("info/**/*.json").unwrap() {
		match entry {
			Ok(path) => {
				let welp = path.display().to_string().clone();
				let mut split = welp.split('/');
				let noextension = commands::misc::replace(".json", &split.nth(2).unwrap_or_default(), "");
				if distro.eq_ignore_ascii_case(&noextension) {
					if !noextension.to_string().contains("_") {
						let text = commands::misc::read_to_string(&path);
						let parsed = json::parse(&text).unwrap();
						let _ = msg.channel_id.send_message(|m| m
							.embed(|e| e
							.title(&noextension)
							.color(Colour::new(parsed["colour"].as_u32().unwrap()))
							.thumbnail(parsed["image"].as_str().unwrap())
							.description(parsed["text"].as_str().unwrap())
							.url(parsed["link"].as_str().unwrap())
						));
					}
				}
			},
			Err(e) => println!("{:?}", e),
			}
		}
	}
});

command!(wget(_context, msg, arg) {
	// Used for archival reason, TODO making it more precise than 100 messages
	let args: Vec<String> = arg.multiple().unwrap();
	let mut verylongwgetlist = "".to_string();
	let mut msg_id = msg.id;
	let mut counter=0;
	let mut finished = false;
	if args.len() == 0 {
		while !finished {
			for vec in msg.channel_id.messages(|g| g.before(msg_id).limit(100)) {
				for message in vec {
					for attachment in message.attachments {
						if attachment.url.starts_with("http") && attachment.url.ends_with("jpg") || attachment.url.ends_with("png") || attachment.url.ends_with("jpeg") {
							verylongwgetlist = format!("{}{}\n",verylongwgetlist,attachment.url);
						}
					}
					let mut split = message.content.split(' ');
					for link in split {
						if link.starts_with("http") {
							if link.ends_with("jpg") || link.ends_with("png") || link.ends_with("jpeg") {
								verylongwgetlist = format!("{}{}\n",verylongwgetlist,link);
							}
						}
					}
					msg_id = message.id;
				}
			}
			if counter >=10 {
				counter=0;
				finished = true;
			}
			counter += 1;	
		}
	}
	else if args.len() == 1 {
		let countdown: u64 = match args[0].parse() {
			Ok(val)  => val,
			Err(_err) => 10,
		};
		while !finished {
			for vec in msg.channel_id.messages(|g| g.after(msg_id).limit(100)) {
				for message in vec {
					for attachment in message.attachments {
						if attachment.url.starts_with("http") && attachment.url.ends_with("jpg") || attachment.url.ends_with("png") || attachment.url.ends_with("jpeg") {
							verylongwgetlist = format!("{}{}\n",verylongwgetlist,attachment.url);
						}
					}
					let mut split = message.content.split(' ');
					for link in split {
						if link.starts_with("http") {
							if link.ends_with("jpg") || link.ends_with("png") || link.ends_with("jpeg") {
								verylongwgetlist = format!("{}{}\n",verylongwgetlist,link);
							}
						}
					}
					msg_id = message.id;
				}
			}
			if counter >= countdown {
				counter=0;
				finished = true;
			}
			counter += 1;	
		}
	}
	if verylongwgetlist != "" {
		File::create(format!("{}/.lcpae/wget_list", env::home_dir().unwrap().display().to_string())).unwrap();
		if Path::new(&format!("{}/.lcpae/wget_list", env::home_dir().unwrap().display().to_string())).exists() == true {
			let writes = OpenOptions::new()
				.write(true)
				.open(format!("{}/.lcpae/wget_list", env::home_dir().unwrap().display().to_string()))
				.unwrap();
			writes.set_len(0).unwrap();
			let mut writer = BufWriter::new(&writes);
				writer.write_all(verylongwgetlist.as_bytes()).unwrap();;
		}
	let file = &format!("{}/.lcpae/wget_list", env::home_dir().unwrap().display().to_string());
	let path: Vec<&str> = vec![file];
	let _ = msg.channel_id.send_files(path, |m| m.content("Use it as `wget --input-file=wget-list` in directory in which you want files to save files\n\nHave fun :D"));
	fs::remove_file(format!("{}/.lcpae/wget_list", env::home_dir().unwrap().display().to_string())).unwrap();
	}
});
command!(clist(_context, msg) {
	// Listing custom commands, folder of server containing jsons with commmands
	let mut list = format!("**You can use commands:**");
	let guild_id = match CACHE.read().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().guild_id,
		None => {
			let _ = msg.channel_id.send_message(|m| m.content("Groups and DMs not supported"));
			return Ok(());
		},
	};
	for file in glob(&format!("{}/.lcpae/commands/{}/*.json",env::home_dir().unwrap().display().to_string(),guild_id)).unwrap() {
		match file {
			Ok(path_file) => {
				let welp = path_file.display().to_string().clone();
				let mut split = welp.split('/');
				let noextension = commands::misc::replace(".json", &split.last().unwrap_or_default(), "");
				if !noextension.to_string().contains("_") {
					list = format!("{}\n{}", &list, &noextension);
				}
			},
			Err(e) => println!("{:?}", e),
						
			}
		}
	let _=msg.channel_id.send_message(|m| m.content(&list));
});


