extern crate serenity;

command!(clear(_context, msg, args) {
	match msg.delete() {
		Ok(val)  => val,
		Err(_err) => (),
	};
	if args.len() == 1 {
		let countdown: u64 = match args[0].parse() {
			Ok(val)  => val,
			Err(_err) => 0,
		};
		for vec in msg.channel_id.messages(|g| g.before(msg.id).limit(countdown)) {
				let mut vec_id = Vec::new();
				for message in vec {
					vec_id.push(message.id);
				}
				match msg.channel_id.delete_messages(vec_id.as_slice()) {
					Ok(val)  => val,
					Err(_err) => (),
				};
		}
		let _=msg.channel_id.send_message(|m| m.content(format!("Deleted {} messages",args[0])));
	}
	else if args.len() == 2 {
		let countdown: u64 = match args[0].parse() {
			Ok(val)  => val,
			Err(_err) => 0,
		};
		let counter: u64 = match args[1].parse() {
			Ok(val)  => val,
			Err(_err) => 0,
		};
		let mut msg_id = msg.id;
		if counter !=0 {
			for vec in msg.channel_id.messages(|g| g.before(msg.id).limit(counter)) {
					for message in vec {
						msg_id = message.id;
					}
			}
		}
		for vec in msg.channel_id.messages(|g| g.before(msg_id).limit(countdown)) {
				let mut vec_id = Vec::new();
				for message in vec {
					vec_id.push(message.id);
				}
				match msg.channel_id.delete_messages(vec_id.as_slice()) {
					Ok(val)  => val,
					Err(_err) => (),
				};
		}
		let _=msg.channel_id.send_message(|m| m.content(format!("Deleted {} messages",args[0])));
	}
	for vec in msg.channel_id.messages(|g| g.after(msg.id).limit(1)) {
		for message in vec {
			match message.delete() {
				Ok(val)  => val,
				Err(_err) => (),
			};
		}
	}
	
	
});
