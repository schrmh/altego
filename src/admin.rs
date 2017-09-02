
use std::string::*;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufWriter;

pub fn adminrm(god: &str, game: &str) -> u64 {
	let mut a = 3;
		if Path::new(&god).exists() == true {
			if Path::new(&god).exists() == true {
				let mut count = 0;
				let mut file = File::open(&god).expect("opening file");
				let mut text = String::new();
				let mut save = String::new();
				file.read_to_string(&mut text).expect("reading file");
				for line in text.lines() {
					if line != game{
						if line !="" {
							save = format!("{}\n{}",&mut save, line);
						}
					}
					else {
						count=count+1;
					}
				}
				let done = OpenOptions::new()
						.write(true)
						.open(god)
						.unwrap();
					done.set_len(0).unwrap();
				let mut writer = BufWriter::new(&done);
					writer.write_all(save.as_bytes()).unwrap();;
				if count == 0 {
					a = 0;
				}
				else {
					a = 1;
				}
			}
		}
	return a;
}
pub fn adminadd(god: &str, game: &str) -> u64 {
	let mut a = 3;
	if Path::new(&god).exists() == true {
		let mut count = 0;
		let mut file = File::open(&god).expect("opening file");
		let mut text = String::new();
		file.read_to_string(&mut text).expect("reading file");
		for line in text.lines() {
			if line == game{	
				count=count+1;	
			}
		}
		if count == 0 {
			let done =
			OpenOptions::new()
				.write(true)
				.append(true)
				.open(god)
				.unwrap();
			let au = format!("\n{}",game);
			let mut writer = BufWriter::new(&done);
				writer.write_all(au.as_bytes()).unwrap();;
			a = 0;
		}
		else {
			a = 1;
		}
	}
	return a;
}
pub fn init(god: &str, author: &str, dir: &str, gamedir: &str) -> u64 {
	let mut a = 5;
	if Path::new("./servers").exists() == false {
		match fs::create_dir("servers"){
			Err(why) => println!("! {:?}", why.kind()),
			Ok(_) => {},
		}
		a = 0;
	}
	if Path::new(&dir).exists() == false {
			match fs::create_dir(dir.clone()) {
				Err(why) => println!("! {:?}", why.kind()),
				Ok(_) => {},		
			}
			a = 1;
		}
		if Path::new(&gamedir).exists() == false {
			File::create(&gamedir).unwrap();
			a = 2;
		}
		let mut c = 0;
		let mut file = File::open(&god).expect("opening file");
		let mut text = String::new();
		file.read_to_string(&mut text).expect("reading file");
		for line in text.lines() {
			if line == author{	
				c=c+1;	
			}
		}
		if c == 0 {
		let done =
			OpenOptions::new()
				.write(true)
				.append(true)
				.open(&god)
				.unwrap();
		let au = format!("\n{}",author);
		let mut writer = BufWriter::new(&done);
			writer.write_all(au.as_bytes()).unwrap();;
	}
	return a;
}
