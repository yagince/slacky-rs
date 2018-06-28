extern crate clap;
extern crate slack_hook;

use std::io;
use std::result;
use clap::{Arg, App};
use slack_hook::{Slack, PayloadBuilder};

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug)]
pub struct NotifyConfig {
    pub url: String,
    pub channel: String,
    pub icon: String,
}

impl NotifyConfig {
    pub fn new(webhook_url: &str, channel: Option<&str>, icon: Option<&str>) -> NotifyConfig {
        NotifyConfig {
            url: webhook_url.to_string(),
            channel: channel.map(|x| x.to_string()).unwrap_or_default(),
            icon: icon.map(|x| format!(":{}", x)).unwrap_or_default(),
        }
    }
}

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

    println!("{:?}", matches);

    let webhook_url = matches.value_of("webhook-url").unwrap();
    let channel = matches.value_of("channel");
    let icon = matches.value_of("icon");
    let notify_config = NotifyConfig::new(webhook_url, channel, icon);
    let msg = matches.value_of("MESSAGE")
        .map(|x| x.to_string())
        .unwrap_or_else(|| read_stdin().expect("MESSAGE is required!!!"));

    println!("url:\t{}", webhook_url);
    println!("chan:\t{:?}", channel);
    println!("msg:\t{}", msg);
    println!("config:\t{:?}", notify_config);

    notify_to_slack(&msg, notify_config).expect("Slack Error");
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn notify_to_slack(msg: &str, config: NotifyConfig) -> result::Result<(), slack_hook::Error> {
    let slack = Slack::new(config.url.as_ref()).unwrap();
    let p = PayloadBuilder::new()
        .text(msg)
        .channel(config.channel)
        .icon_emoji(config.icon)
        .build()
        .unwrap();

    slack.send(&p)?;
    Ok(())
}
