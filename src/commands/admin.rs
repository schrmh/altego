extern crate serenity;

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
				for message in vec {
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
