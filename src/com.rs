extern crate regex;

use std::string::*;
use std::fs::File;
use std::io::Read;
use self::regex::Regex;
use std::ascii::AsciiExt;

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

pub fn gnu_replacement(content: Vec<String>) -> String {
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
	let mut gnu = File::open("gnu.txt").expect("opening file");
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
