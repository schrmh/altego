extern crate serenity;

use commands;
use std::path::PathBuf;

command!(github(_context, msg) {
	let _= msg.channel_id.send_message(|m| m.content("I mean, Github something or other is like here or something:\nhttps://github.com/LelCP/altego"));
});



command!(rust(_context, msg) {
	let path = PathBuf::from("pastas/rust.txt");
	let _= msg.channel_id.send_message(|m| m.content(&commands::misc::read_to_string(&path)));
});
