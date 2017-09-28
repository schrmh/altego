extern crate serenity;


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
