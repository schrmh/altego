use std::string::*;
use std::fs::File;
use std::path::Path;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::fs::OpenOptions;
use std::io::BufWriter;
use com;

pub fn join(author: String, gamedir: &str) -> u64 {
	let mut a = 3;
	if Path::new(&gamedir).exists() == true {
		let mut count = 0;
		let mut file = File::open(&gamedir).expect("opening file");
		let mut text = String::new();
		file.read_to_string(&mut text).expect("reading file");
		for line in text.lines() {
			if line == author{	
				count=count+1;	
			}
		}
		if count == 0 {
			let done =
				OpenOptions::new()
					.write(true)
					.append(true)
					.open(gamedir)
					.unwrap();
				let au = format!("\n{}",author);
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
pub fn list(dir: &str) -> String {
	let paths = fs::read_dir(&dir.clone()).unwrap();
	let mut list = format!("Available groups are:");
	for path in paths {
		let welp = path.unwrap().path().display().to_string().clone();
		let xx = com::replace(&dir, &welp, "");
		let yy = com::replace("/", &xx, "");
		let zz = com::replace(".txt", &yy, "");
		list = format!("{} {},",&mut list, zz);

	}
	return list;
}
pub fn play(gamedir: &str, game: &str) -> String {
	let mut lineset = format!("Do you want to play {} with us?", game).to_string();
	if Path::new(&gamedir).exists() == true {
		let mut file = File::open(&gamedir).expect("opening file");
		let mut text = String::new();
		file.read_to_string(&mut text).expect("reading file");
		for line in text.lines() {
			if line != ""
			{
				lineset = format!("{} <@!{}>",&mut lineset, line);
			}
			
		}
		
	}
	return lineset;
}
pub fn leave(author: String, gamedir: &str) -> u64 {
	let mut a = 3;
	if Path::new(&gamedir).exists() == true {
		if Path::new(&gamedir).exists() == true {
			let mut count = 0;
			let mut file = File::open(&gamedir).expect("opening file");
			let mut text = String::new();
			let mut save = String::new();
			file.read_to_string(&mut text).expect("reading file");
			for line in text.lines() {
				if line != author{
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
					.open(gamedir)
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


