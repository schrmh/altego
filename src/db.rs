//this needs more sensible rewrite to rusqlite
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::fs::DirBuilder;

fn lines_from_file<P>(filename: P) -> Vec<String>
where
	P: AsRef<Path>,
{
	let file = File::open(filename).expect("no such file");
	let buf = BufReader::new(file);
	buf.lines()
		.map(|l| l.expect("Could not parse line"))
		.collect()
}

pub fn read_pierogi(userid: &str, serverid: &str) -> i8 {
	let a=0;
	if Path::new(&format!("servers/{}/{}.txt", serverid, userid)).exists() {
		let lines = lines_from_file(format!("servers/{}/{}.txt", serverid, userid));
		return lines[0].parse::<i8>().unwrap();
	}
	return a;
}
pub fn time_pierogi(userid: &str, serverid: &str) -> u64 {
	let a=0;
	if Path::new(&format!("servers/{}/{}.txt", serverid, userid)).exists() {
		let lines = lines_from_file(format!("servers/{}/{}.txt", serverid, userid));
		return lines[1].parse::<u64>().unwrap();
	}
	return a;
}
pub fn new_pierogi(userid: &str, serverid: &str, pierogi: i8, time: u64){
	let save = format!("{}\n{}", pierogi, time);
	if !Path::new(&format!("servers/{}/", serverid)).exists() {
		DirBuilder::new()
			.recursive(true)
			.create(&format!("servers/{}/", serverid)).unwrap();
	}
	let file = File::create(&format!("servers/{}/{}.txt", serverid, userid)).unwrap();
	file.set_len(0).unwrap();
	let mut writer = BufWriter::new(&file);
		writer.write_all(save.as_bytes()).unwrap();;
}
