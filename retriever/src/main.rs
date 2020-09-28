/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020
LESTERRRY AUTHORSHIP
***************************/

mod meta;

use clap::App;
use clap::Arg;
use std::io::{stdout, Write};
use curl::easy::{Easy, List};
use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::str;

fn main() {
	let matches = App::new("retriever")
		.version("0.1.0")
		.about("Part of Berg Transfocator project")
		.author("Aydar N.")
		.arg(Arg::with_name("OUTPUT")
			.help("Path to output file")
			.required(false)
			.index(1))
		.arg(Arg::with_name("print")
			.short("p")
			.long("print")
			.help("Print retrieved data to stdout"))
		.arg(Arg::with_name("create new")
			.short("c")
			.long("new")
			.help("Create new file instead of looking for existing one"))
		.get_matches();

	let mut easy = Easy::new();
	easy.url("https://api.twitter.com/2/tweets/search/stream").unwrap();

	let mut list = List::new();
	list.append(&("Authorization: Bearer ".to_owned() + meta::BEARER)).unwrap();
	easy.http_headers(list).unwrap();

	let path = PathBuf::from(matches.value_of("OUTPUT").unwrap());
	let mut file;
	if matches.is_present("create new") { file = File::create(path).unwrap(); }
	else { 
		file = OpenOptions::new()
			.write(true)
			.append(true)
			.open(path)
			.unwrap(); 
		}

	easy.write_function(move |data| {
		if matches.is_present("print") { stdout().write_all(data).unwrap() }
		match str::from_utf8(data) {
			Ok(d) => write!(&mut file, "{}", if d == "\r\n" { "" } else { d }).unwrap(),
			Err(_) => panic!("Couldn't convert from slice"),
		}
		
		Ok(data.len())
	}).unwrap();

	easy.perform().unwrap();
}