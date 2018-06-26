extern crate clap;
use clap::{Arg, App};

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author(PKG_AUTHOR)
        .arg(
            Arg::with_name("webhook-url")
                .short("u")
                .long("url")
                .value_name("URL")
                .help("slack webhook url")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("channnel")
                .short("c")
                .long("channel")
                .value_name("CHANNEL_NAME")
                .help("slack channel name. ex) #general")
                .takes_value(true)
        )
        .get_matches();

    println!("{:?}", matches);
}
