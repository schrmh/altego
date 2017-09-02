#[macro_use]
extern crate serenity;
extern crate chrono;
extern crate ddg;

use std::string::*;
use serenity::client::CACHE;
use serenity::framework::StandardFramework;
use serenity::model::*;
use serenity::prelude::*;
use serenity::Result as SerenityResult;
use std::env;
use std::fs::File;
use std::path::Path;
use std::ascii::AsciiExt;
use chrono::*;
use std::io::Read;
use ddg::Query;
use serenity::utils::Colour;
use serenity::utils::builder::CreateEmbedFooter;

mod ddginc;
mod admin;
mod coop;
mod com;

struct Handler;

impl EventHandler for Handler {
	fn on_ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

fn main() {
	let token = env::var("DISCORD_TOKEN")
		.expect("Expected a token in the environment");
	let mut client = Client::new(&token, Handler);

	client.with_framework(StandardFramework::new()
		.configure(|c| c
			.prefix("!")
			.on_mention(true))
		.on("gnu", gnu)
		.on("ddg", ddg)
		.on("rust", rust)
		.on("roasted", roasted)
		.on("lcp help", lcphelp)
		.on("coop", coop)
		.on("donkey", donkey));
	let _ = client.start().map_err(|why| println!("Client ended: {:?}", why));
}

command!(coop(_ctx, msg) {
	let mut split = msg.content.split(' ');
	let argument = split.nth(1).unwrap_or("");
	let game = split.next().unwrap_or("");
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say("Groups and DMs not supported"));
			return Ok(());
		},
	};
	let dir = format!("servers/{}",guild_id);
	let gamedir=format!("{}/{}.txt",&dir,game);
	if let Some(guild) = CACHE.read().unwrap().guild(guild_id) {
		let joinerror = format!("<@!{}>, I'm so terribly sorry, but I couldn't add you to {} group, you already exist in this group",msg.author.id, game);
		let joinaccept = format!("<@!{}>, You got added to {} group, play with us anytime",msg.author.id, game);
		let leaveerror = format!("<@!{}>, You got removed from {} group :crying_cat_face: ",msg.author.id, game);
		let leaveaccept = format!("<@!{}>, You don't exist in {} group",msg.author.id, game);
		let initdeny = format!("<@!{}>, I don't recognize you as my real daddy, where is my daddy?",msg.author.id);
		let author = format!("{}",msg.author.id);
		let god = format!("servers/{}.txt", guild_id);
		if Path::new(&god).exists() == true || guild.read().unwrap().owner_id == msg.author.id{
			let mut count = 0;
			if Path::new(&god).exists() == false {
				match File::create(&god){
					Ok(val)  => val,
					Err(err) => return Err(err.to_string()),
				};
			}
			let mut file = File::open(&god).expect("opening file");
			let mut text = String::new();
			file.read_to_string(&mut text).expect("reading file");
			for line in text.lines() {
				if line == author{
					count=count+1;
				}
			}
			if guild.read().unwrap().owner_id == msg.author.id {
				count=count+1;
			}
			if argument.eq_ignore_ascii_case("init") {
				if count !=0{
					if admin::init(&god, &author, &dir, &gamedir) == 0 {
						check_msg(msg.channel_id.say(&format!("Initializing bot chief <@!{}>", msg.author.id)));
					}
					else if admin::init(&god, &author, &dir, &gamedir) == 1 {
						check_msg(msg.channel_id.say(&format!("Initializing this channel chief <@!{}>", msg.author.id)));
					}
					else if admin::init(&god, &author, &dir, &gamedir) == 2 {
						check_msg(msg.channel_id.say(&format!("Adding {} to game database chief", game)));
					}
					else {
						check_msg(msg.channel_id.say(&format!("Fucking hell")));
					}
				}
				else {
					check_msg(msg.channel_id.say(&initdeny));
					let paths = vec!["camgirl.png"];
					let _ = msg.channel_id.send_files(paths, |m| m.content(" "));
				}
			}
			else if argument.eq_ignore_ascii_case("adminadd") {
				if count !=0{
					if admin::adminadd(&god, &game) == 0 {
						check_msg(msg.channel_id.say(&format!("{}",leaveaccept)));
					}
					else {
						check_msg(msg.channel_id.say(&format!("{}",leaveerror)));
					}
				}
				else {
					check_msg(msg.channel_id.say(&initdeny));
					let paths = vec!["camgirl.png"];
					let _ = msg.channel_id.send_files(paths, |m| m.content(" "));
				}
			}
			else if argument.eq_ignore_ascii_case("adminrm") && count !=0 {
				if count !=0{
					if admin::adminrm(&god, &game) == 0 {
						check_msg(msg.channel_id.say(&format!("{}",leaveaccept)));
					}
					else {
						check_msg(msg.channel_id.say(&format!("{}",leaveerror)));
					}
				}
				else {
					check_msg(msg.channel_id.say(&initdeny));
					let paths = vec!["camgirl.png"];
					let _ = msg.channel_id.send_files(paths, |m| m.content(" "));
				}
			}
		}
		if argument.eq_ignore_ascii_case("leave") {
			if coop::leave(author.clone(), &gamedir) == 0 {
				check_msg(msg.channel_id.say(&format!("{}",leaveaccept)));
			}
			else {
				check_msg(msg.channel_id.say(&format!("{}",leaveerror)));
			}

		}
		else if argument.eq_ignore_ascii_case("play") {
			check_msg(msg.channel_id.say(coop::play(&gamedir, &game)));
		}
		else if argument.eq_ignore_ascii_case("join") {
			if coop::join(author.clone(), &gamedir) != 1 {
				check_msg(msg.channel_id.say(&joinaccept));
			}
			else {
				check_msg(msg.channel_id.say(&joinerror));
			}
		}
		else if argument.eq_ignore_ascii_case("list") {
			check_msg(msg.channel_id.say(coop::list(&dir)));
		}
	}
});

command!(rust(_context, msg) {
	check_msg(msg.channel_id.say(com::read_to_string("rust.txt")));
});
command!(lcphelp(_context, msg) {
	check_msg(msg.channel_id.say(com::read_to_string("help.txt")));
});

command!(roasted(_context, msg) {
	let paths = vec!["roasted.png"];
	let _ = msg.channel_id.send_files(paths, |m| m.content(""));

});

command!(gnu(_context, msg) {
	let paths = vec!["interjection.png"];
	let _ = msg.channel_id.send_files(paths, |m| m.content(&format!("```{}```",&com::gnu_replacement(&msg.content))));

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
			.title(format!("Based on {} article",&response.definition_source))
			.footer(|_| footer)
			.colour(colour)
			.thumbnail(&response.image)
			.description(format!("{}\n\n*Read more:* <{}>",&response.definition,&response.definition_url))
			.url(&new)
			));
	}
	else if response.abstract_url != ""{
		let mut footer = CreateEmbedFooter::default()
			.text(&new)
			.icon_url("https://duckduckgo.com/assets/icons/meta/DDG-icon_256x256.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(format!("Based on {} article",&response.abstract_source))
			.colour(colour)
			.footer(|_| footer)
			.thumbnail(&response.image)
			.description(format!("{}\n\n*Read more:* <{}>",&response.abstract_text,&response.abstract_url))
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
			.description(format!("{}",&response.redirect))
			.url(&new)
			));
	}
	else {
		let mut footer = CreateEmbedFooter::default()
			.text(&new)
			.icon_url("https://duckduckgo.com/assets/icons/meta/DDG-icon_256x256.png");
		let _ = msg.channel_id.send_message(|m| m
			.embed(|e| e
			.title(format!("Results from :duck::duck::goal: for query \"{}\"",&help))
			.footer(|_| footer)
			.colour(colour)
			.url(&new)
			.description(ddginc::read_ddg(&help, 5))
			));
	}
});

command!(donkey(ctx, msg) {
	let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
		Some(channel) => channel.read().unwrap().guild_id,
		None => {
			check_msg(msg.channel_id.say("Groups and DMs not supported"));
			return Ok(());
		},
	};
	let user_id = CACHE.read().unwrap().user.id;
	ctx.set_game_name("lelcp.github.io");
	match guild_id.edit_nickname(Some("ԀƆ˥")) {
		Ok(val)  => val,
		Err(err) => return Err(err.to_string()),
	};
	let now = Local::now();
	let dt=format!("{}", now.format("Kong! %Y-%m-%d %H:%M:%S").to_string());
	check_msg(msg.channel_id.say(&format!("{}, <@!{}>", dt,user_id )));
});

fn check_msg(result: SerenityResult<Message>) {
	if let Err(why) = result {
		println!("Error sending message: {:?}", why);
	}
}
