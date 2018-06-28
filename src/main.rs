extern crate clap;
extern crate slacky;

use std::io;
use clap::{Arg, App};
use slacky::slack;

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let matches = App::new(PKG_NAME)
        .version(PKG_VERSION)
        .author(PKG_AUTHOR)
        .arg(Arg::with_name("webhook-url")
            .short("u")
            .long("url")
            .value_name("URL")
            .help("slack webhook url")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("channel")
            .short("c")
            .long("channel")
            .value_name("CHANNEL_NAME")
            .help("slack channel name. ex) #general")
            .takes_value(true))
        .arg(Arg::with_name("icon")
            .short("i")
            .long("icon")
            .value_name("ICON_NAME")
            .help("slack icon name. ex) monkey")
            .takes_value(true))
        .arg(Arg::with_name("MESSAGE").help("slack channel name. ex) #general"))
        .get_matches();

    let webhook_url = matches.value_of("webhook-url").unwrap();
    let channel = matches.value_of("channel");
    let icon = matches.value_of("icon");
    let notify_config = slack::NotifyConfig::new(webhook_url, channel, icon);
    let msg = matches.value_of("MESSAGE")
        .map(|x| x.to_string())
        .unwrap_or_else(|| read_stdin().expect("MESSAGE is required!!!"));

    slack::notify(&msg, &notify_config).expect("Slack Error");
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}
