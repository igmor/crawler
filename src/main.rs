// Simple Web cralwer written in Rust
// mostly as an exersice project to get faimilar with Rust
// language and ecosystem


extern crate curl;
extern crate url;
extern crate getopts;

use getopts::Options;
use std::env;


use std::io::{stdout, Write};
use url::{Url, Host};
use curl::easy::Easy;

fn spawn_crawler(target_url: Option<String>) {
    let mut easy = Easy::new();
    easy.url(target_url.unwrap().as_str()).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}


// Print a web page onto stdout
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("u", "url", "set target url", "URL");
    opts.optflag("h", "help", "print this help menu");
    if args.len() < 2 {
        print_usage(&program, opts);
        return;    
    }
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let target_url = match Url::parse(matches.opt_str("u").unwrap().as_str()) {
    	Ok(x) => { matches.opt_str("u") }
	Err(f) => { panic!(f.to_string()) }
    };
    spawn_crawler(target_url);    
}

