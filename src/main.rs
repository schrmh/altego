#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate ddg;
extern crate rand;
extern crate time;

use std::string::*;
use serenity::client::CACHE;
use serenity::model::*;
use serenity::Result as SerenityResult;
use serenity::Client;
use serenity::ext::framework::help_commands;

use std::env;
use std::fs::File;
use std::path::Path;
use std::ascii::AsciiExt;
use chrono::*;
use std::io::Read;
use ddg::Query;
use serenity::utils::Colour;
use serenity::utils::builder::CreateEmbedFooter;
use rand::distributions::{IndependentSample, Range};
use std::fs;

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
				.exec(info)))
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
	check_msg(msg.channel_id.say(&com::read_to_string("rust.txt")));
});

command!(lcphelp(_context, msg) {
	check_msg(msg.channel_id.say(&com::read_to_string("help.txt")));
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

command!(info(_context, msg, args) {
	if args.len() == 0 {
		let paths = fs::read_dir("distros").unwrap();
		let mut list = format!("**You can learn about:**");
		for path in paths {
			let welp = path.unwrap().path().display().to_string().clone();
			let xx = com::replace("distros", &welp, "");
			let yy = com::replace("/", &xx, "");
			let zz = com::replace(".txt", &yy, "");
			if !zz.to_string().contains("_") {
				
				list = format!("{}\n{}",&mut list, zz);
			}

		}
	check_msg(msg.channel_id.say(&list));
	}
	else {
		let mut adress = "".to_string();
		let mut image = "".to_string();
		let mut colour = Colour::new(0);
		let mut distro = &args[0];
		let paths = fs::read_dir("distros").unwrap();
		for path in paths {
			let welp = path.unwrap().path().display().to_string().clone();
			let xx = com::replace("distros", &welp, "");
			let yy = com::replace("/", &xx, "");
			let zz = com::replace(".txt", &yy, "");
			if distro.eq_ignore_ascii_case(&zz) {
				if !zz.to_string().contains("_") {
					let mut fulltext = "".to_string();
					let text = com::read_to_string(&format!("distros/{}.txt",&zz));
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
						.title(&zz)
						.color(colour)
						.thumbnail(&image)
						.description(&fulltext)
						.url(&adress)
					));
				}
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
