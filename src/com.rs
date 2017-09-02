extern crate regex;

use std::string::*;
use std::fs::File;
use std::io::Read;
use self::regex::Regex;

pub fn replace(replaced: &str, replacement: &str,replacing: &str) -> String {
	let re = match Regex::new(replaced) {
		Ok(val)  => val,
		Err(err) => return format!("{}",err),
	};
	return re.replace_all(replacement, replacing).to_string();
}

pub fn read_to_string(filename: &str) -> String {
	let mut rust = File::open(filename).expect("opening file");
	let mut rustext = String::new();
	rust.read_to_string(&mut rustext).expect("reading file");
	return rustext.to_string();
}

pub fn gnu_replacement(content: &str) -> String {
	let mut split = content.split(' ');
	let argument = split.nth(1).unwrap_or("");
	let game = split.next().unwrap_or("");
	let mut gnu = File::open("gnu.txt").expect("opening file");
	let mut gnutext = String::new();
	gnu.read_to_string(&mut gnutext).expect("reading file");
	let mut dude = argument.clone();
	let mut pal = game.clone();
	if game == "" {
		pal = "GNU";
	}
	if argument == "" {
		dude = "Linux";
	}
	if game == "Linux" {
		pal = "🿽";
	}
	let xx = replace("GNU", &gnutext, pal);
	let yy = replace("Linux", &xx, dude);
	let zz = replace("🿽", &yy, game);
	return replace("`", &zz, "");
}
