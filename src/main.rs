#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate ddg;
extern crate rand;
extern crate time;
extern crate glob;

use std::string::*;
use serenity::client::CACHE;
use serenity::model::*;
use serenity::Result as SerenityResult;
use serenity::Client;
use serenity::ext::framework::help_commands;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::ascii::AsciiExt;
use chrono::*;
use std::io::Read;
use ddg::Query;
use serenity::utils::Colour;
use serenity::utils::builder::CreateEmbedFooter;
use rand::distributions::{IndependentSample, Range};
use std::fs;
use glob::glob;
use std::path::PathBuf;

mod ddginc;
mod coop;
mod com;

fn main() {
	let token = env::var("DISCORD_TOKEN")
		.expect("Expected a token in the environment");
	let mut client = Client::new(&token);
	
	client.with_framework(|f| f
		.bucket("basic", 5, 60, 3)
		.configure(|c| c
			.prefix("!")
			.on_mention(true))
		.group("Miscellaneous", |g| g
			.command("gnu", |c| c
				.desc("GNU Interjection copypasta")
				.usage("<GNU replacement> <Linux replacement>")
				.use_quotes(true)
				.exec(gnu))
			.command("whiteface", |c| c
				.desc("This is not the gif on the internet")
				.exec(whiteface))
			.command("godtellmeyourways", |c| c
				.desc("Quote God himself, Richard Matthew Stallman")
				.exec(god))
			.command("donkey", |c| c
				.desc("Show current bot time and set nickname and playing message")
				.exec(donkey)))
		.group("Useful shit", |g| g
			.command("ddg", |c| c
				.desc("Powerful search with embeds")
				.usage("<search terms>")
				.exec(ddg))
			.command("emoji", |c| c
				.desc(":regional_indicator_s::regional_indicator_p::regional_indicator_e::regional_indicator_a::regional_indicator_k:   :regional_indicator_i::regional_indicator_n:   :regional_indicator_e::regional_indicator_m::regional_indicator_o::regional_indicator_j::regional_indicator_i:")
				.usage("<your text>")
				.bucket("basic")
				.exec(emoji))
			.command("info", |c| c
				.desc("Distro database")
				.usage("<distro name>")
				.exec(info))
			.command("wget", |c| c
				.desc("Sends wget script")
				.usage("<amount of messages in hundreds to download from channel>")
				.exec(wget)))
		.group("About bot", |g| g
			.command("github", |c| c
				.desc("Shilling open source nature of this project")
				.exec(github))
			.command("rust", |c| c
				.desc("Shilling rust nature of this project")
				.exec(rust))
			.command("lcpae", |c| c
			.exec_help(help_commands::with_embeds)))
		.group("Tux", |g| g
			.command("roasted", |c| c
				.desc("Daaaaaamn!")
				.exec(roasted))
			.command("hypertux", |c| c
				.desc("Laugh all you want")
				.exec(hypertux))
			.command("hyperthink", |c| c
				.desc("TFW hacked mainframe")
				.exec(hyperthink)))
		.group("Coop", |g| g
			.prefix("coop")

			.command("leave", |c| c
				.desc("TFW hacked mainframe")
				.exec(leave))
			.command("play", |c| c
				.desc("TFW hacked mainframe")
				.exec(play))
			.command("join", |c| c
				.desc("TFW hacked mainframe")
				.exec(join))
			.command("list", |c| c
				.desc("TFW hacked mainframe")
				.exec(list)))); //I don't exactly know why reading server id doesn't work, TODO
	client.on_ready(|ctx, _| {ctx.set_game_name("lelcp.github.io");});
	let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}

command!(list(_ctx, msg) {
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say("Groups and DMs not supported"));

			return Ok(());
		},
	};
	CACHE.read().unwrap().guild(guild_id);
		let dir = format!("servers/{}",guild_id);
		check_msg(msg.channel_id.say(&coop::list(&dir)));
});

command!(join(_ctx, msg, args) {
	let game = &args[0];
	let joinerror = format!("<@!{}>, I'm so terribly sorry, but I couldn't add you to {} group, you already exist in this group",msg.author.id, game);
	let joinaccept = format!("<@!{}>, You got added to {} group, play with us anytime",msg.author.id, game);
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say("Groups and DMs not supported"));

			return Ok(());
		},
	};
	CACHE.read().unwrap().guild(guild_id);
		let dir = format!("servers/{}",guild_id);
		let gamedir=format!("{}/{}.txt",&dir,game);
		if coop::join(msg.author.id.to_string(), &gamedir) != 1 {
			check_msg(msg.channel_id.say(&&joinaccept));
		}
		else {
			check_msg(msg.channel_id.say(&&joinerror));
		}
});

command!(leave(_ctx, msg, args) {
	let game = &args[0];
	let leaveerror = format!("<@!{}>, You got removed from {} group :crying_cat_face: ",msg.author.id, game);
	let leaveaccept = format!("<@!{}>, You don't exist in {} group",msg.author.id, game);
		let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say("Groups and DMs not supported"));

			return Ok(());
		},
	};
	CACHE.read().unwrap().guild(guild_id);
		let dir = format!("servers/{}",guild_id);
		let gamedir=format!("{}/{}.txt",&dir,game);
		if coop::leave(msg.author.id.to_string(), &gamedir) == 0 {
			check_msg(msg.channel_id.say(&&format!("{}",leaveaccept)));
		}
		else {
			check_msg(msg.channel_id.say(&&format!("{}",leaveerror)));
		}

});

command!(play(_ctx, msg, args) {
	let game = &args[0];
		let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say("Groups and DMs not supported"));

			return Ok(());
		},
	};
	CACHE.read().unwrap().guild(guild_id);
		let dir = format!("servers/{}",guild_id);
		let gamedir=format!("{}/{}.txt",&dir,game);
		check_msg(msg.channel_id.say(&coop::play(&gamedir, &game)));
});


command!(github(_context, msg) {
	check_msg(msg.channel_id.say("I mean, Github something or other is like here or something:\nhttps://github.com/LelCP/altego"));
});

command!(rust(_context, msg) {
	let path = PathBuf::from("rust.txt");
	check_msg(msg.channel_id.say(&com::read_to_string(&path)));
});

command!(roasted(_context, msg) {
	let paths = vec!["images/roasted.png"];
	let _ = msg.channel_id.send_files(paths, |m| m.content(""));

});

command!(hypertux(_context, msg) {
	let paths = vec!["images/hypertux.png"];
	let _ = msg.channel_id.send_files(paths, |m| m.content(""));

});

command!(hyperthink(_context, msg) {
	let paths = vec!["images/hyperthink.png"];
	let _ = msg.channel_id.send_files(paths, |m| m.content(""));

});

command!(gnu(_context, msg, args) {
	let paths = vec!["images/interjection.png"];
	let _ = msg.channel_id.send_files(paths, |m| m.content(&format!("```{}```",&com::gnu_replacement(args.to_vec()))));

});
command!(whiteface(_context, msg) {
	check_msg(msg.channel_id.say(&"https://i.redd.it/fhrd8f2gpxjz.gif"));
});


command!(wget(_context, msg, args) {
	let mut verylongwgetlist = "".to_string();
	let mut msg_id = msg.id;
	let mut counter=0;
	let mut finished = false;
	if args.len() == 0 {
		while !finished {
			for vec in msg.channel_id.messages(|g| g.before(msg_id).limit(100)) {
				for message in vec {
					for attachment in message.attachments {
						if attachment.url.contains("jpg") || attachment.url.contains("png") || attachment.url.contains("jpeg") {
							verylongwgetlist = format!("{}{}\n",verylongwgetlist,attachment.url);
						}
					}
					let mut split = message.content.split(' ');
					for link in split {
						if link.contains("http") {
							if link.contains("jpg") || link.contains("png") || link.contains("jpeg") {
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
						if attachment.url.contains("jpg") || attachment.url.contains("png") || attachment.url.contains("jpeg") {
							verylongwgetlist = format!("{}{}\n",verylongwgetlist,attachment.url);
						}
					}
					let mut split = message.content.split(' ');
					for link in split {
						if link.contains("http") {
							if link.contains("jpg") || link.contains("png") || link.contains("jpeg") {
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
		File::create("wget_list").unwrap();
		if Path::new("wget_list").exists() == true {
			let writes = OpenOptions::new()
				.write(true)
				.open("wget_list")
				.unwrap();
			writes.set_len(0).unwrap();
			let mut writer = BufWriter::new(&writes);
				writer.write_all(verylongwgetlist.as_bytes()).unwrap();;
		}
	let path = vec!["wget_list"];
	let _ = msg.channel_id.send_files(path, |m| m.content("Use it as `wget --input-file=wget-list` in directory in which you want files to save files\n\nHave fun :D"));
	fs::remove_file("wget_list").unwrap();
	}
});

command!(info(_context, msg, args) {
	if args.len() == 0 {
		let mut list = format!("**You can learn about:**");
		let mut distro = "**Distros:**".to_string();
		let mut de = "**DEs & WMs**".to_string();
		let mut other = "**Others**".to_string();
		for entry in glob("info/**/*.txt").unwrap() {
		match entry {
			Ok(path) => {
				let welp = path.display().to_string().clone();
				let mut split = welp.split('/');
				let directory = split.nth(1).unwrap_or_default();
				let noextension = com::replace(".txt", &split.next().unwrap_or_default(), "");
				if !noextension.to_string().contains("_") {
					if directory == "Distros" {
						distro = format!("{}\n{}", &distro, &noextension);
					}
					else if directory == "DEs&WMs" {
						de = format!("{}\n{}", &de, &noextension);
					}
					else if directory == "Others" {
						other = format!("{}\n{}", &other, &noextension);
					}
				}
			},
			Err(e) => println!("{:?}", e),
			}
		}
	list = format!("{}\n{}\n{}\n{}",&mut list, distro, de, other);
	check_msg(msg.channel_id.say(&list));
	}
	else {
		let mut adress = "".to_string();
		let mut image = "".to_string();
		let mut colour = Colour::new(0);
		let mut distro = &args[0];
		for entry in glob("info/**/*.txt").unwrap() {
		match entry {
			Ok(path) => {
				let welp = path.display().to_string().clone();
				let mut split = welp.split('/');
				let noextension = com::replace(".txt", &split.nth(2).unwrap_or_default(), "");
				if distro.eq_ignore_ascii_case(&noextension) {
					if !noextension.to_string().contains("_") {
						let mut fulltext = "".to_string();
						let text = com::read_to_string(&path);
						for line in text.to_string().lines() {
							if line.contains(".png") {
								image = line.to_string();
							}
							else if line.contains("http") && adress == "" {
								adress = line.to_string();
								fulltext = format!("{}\n{}",fulltext, line.to_string());
							}
							else if line.contains("#") {
								colour = Colour::new(com::replace("#", &line, "").parse::<u32>().unwrap());
							}
							else {
								fulltext = format!("{}\n{}",fulltext, line.to_string());
							}
						}
						let _ = msg.channel_id.send_message(|m| m
							.embed(|e| e
							.title(&noextension)
							.color(colour)
							.thumbnail(&image)
							.description(&fulltext)
							.url(&adress)
						));
					}
				}
			},
			Err(e) => println!("{:?}", e),
			}
		}
	}
});

command!(emoji(_context, msg, args) {
	match msg.delete() {
		Ok(val)  => val,
		Err(_err) => (),
	};
	let mut hello = "".to_string();
	let mut world = "".to_string();
	let mut woman = "".to_string();
	let mut b = 0;
	for arg in args.iter() {
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
	if hello.len() < 2000 {
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(&format!("{}",msg.author.name))
			.colour(Colour::from_rgb(0, 80, 80))
			.description(&format!("{}\n{}",hello,world))
		));
	}
});

command!(ddg(_context, msg) {
	let mut welp = com::replace("!ddg ", &msg.content, "");
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
	let new = com::replace(" " ,&duckurl, "+");
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
			.description(&ddginc::read_ddg(&help, 3))
			));
	}
});

command!(god(_ctx, msg) {
	let mut count = 0;
	let between = Range::new(-1, 47);
	let mut rng = rand::thread_rng();
	let a = between.ind_sample(&mut rng);
	let mut lineset = format!(" ").to_string();
	if Path::new("stallman.txt").exists() == true {
		let mut file = File::open("stallman.txt").expect("opening file");
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
	let user_id = CACHE.read().unwrap().user.id;
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say(&"Groups and DMs not supported"));
			return Ok(());
		},
	};
	let now = Local::now();
	let dt=format!("{}", now.format("Kong! %Y-%m-%d %H:%M:%S").to_string());
	check_msg(msg.channel_id.say(&format!("{}, <@!{}>", dt,user_id )));
	match guild_id.edit_nickname(Some("ԀƆ˥")) {
		Ok(val)  => val,
		Err(err) => return Err(err.to_string()),
	};
});

fn check_msg(result: SerenityResult<Message>) {
	if let Err(why) = result {
		println!("Error sending message: {:?}", why);
	}
}
