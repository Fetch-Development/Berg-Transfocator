/***************************
COPYRIGHT FETCH DEVELOPMENT,
2020
LESTERRRY AUTHORSHIP
***************************/

mod meta;

use clap::App;
use clap::Arg;
use std::io::{self, Write};
use curl::easy::{Easy, List};
use std::path::PathBuf;
use std::fs::File;
use std::fs::OpenOptions;
use std::str;
extern crate colorful;
use colorful::Color as TermColor;
use colorful::Colorful;
extern crate serde;
use serde::{Deserialize, Serialize};
extern crate moon_loader;
use moon_loader::*;

#[derive(Serialize, Deserialize)]
struct Tweet {
	data: TweetData,
}

#[derive(Serialize, Deserialize, Debug)]
struct TweetData {
	id: String,
	text: String,
}

fn main() {
	let matches = App::new("retriever")
		.version("0.1.0")
		.about("Part of Berg Transfocator project")
		.author("Aydar N.")
		.arg(Arg::with_name("OUTPUT")
			.help("Path to output file")
			.required(true)
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

	let mut loader = MoonLoader::new(MoonLoaderVariant::Moon, true);

	let mut p = false;
	if matches.is_present("print") { p = true; }
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
		let d;
		match str::from_utf8(data) {
			Ok(dt) => d = dt,
			Err(_) => panic!("Couldn't convert from slice"),
		}

		if p {
			if d == "\r\n"{
				&loader.draw();
				print!("{}for new tweets...", "Waiting ".color(TermColor::Cyan).bold());
				io::stdout().flush().unwrap();
			}
			else{
				println!("\n{}new tweet:", "Retrieved ".color(TermColor::Green).bold());
				let t: Tweet = serde_json::from_str(d).unwrap();
				println!("{}", t.data.text);
				&loader.draw();
				print!("{}for new tweets...", "Looking ".color(TermColor::Cyan).bold());
				io::stdout().flush().unwrap();
			}
			//stdout().write_all(data).unwrap() 
		}
		write!(&mut file, "{}", d ).unwrap();
		
		Ok(data.len())
	}).unwrap();

	if p { 
		loader.draw();
		print!("{}retrieving", "Started ".color(TermColor::Cyan).bold());
		io::stdout().flush().unwrap();
	}
	easy.perform().unwrap();
}
