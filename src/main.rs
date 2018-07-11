extern crate clap;
extern crate slacky;
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::io;
use std::io::prelude::*;
use std::env;
use std::fs;
use std::path::*;
use clap::{Arg, App, SubCommand};
use slacky::slack;

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_AUTHOR: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug, Deserialize, Serialize, Default)]
struct SlackyConf {
    webhook_url: String,
    channel: Option<String>,
    icon: Option<String>,
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
            .takes_value(true))
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
        .subcommand(SubCommand::with_name("init").about("init slacky config"))
        .get_matches();

    if let Some(_) = matches.subcommand_matches("init") {
        create_conf().expect("create conf");
    } else {
        let mut conf = read_conf().expect("read conf");
        println!("{:?}", conf);

        matches.value_of("webhook-url").map(|url| {
            conf.webhook_url = url.into();
        });
        matches.value_of("channel").map(|channel| {
            conf.channel = Some(channel.into());
        });
        matches.value_of("icon").map(|icon| {
            conf.icon = Some(icon.into());
        });
        let notify_config = slack::NotifyConfig::new(&conf.webhook_url, conf.channel, conf.icon);
        let msg = matches.value_of("MESSAGE")
            .map(|x| x.to_string())
            .unwrap_or_else(|| read_stdin().expect("MESSAGE is required!!!"));
        println!("{:?}\t{:?}", notify_config, msg);
        slack::notify(&msg, &notify_config).expect("Slack Error");
    }
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf)?;
    Ok(buf)
}

fn read_conf() -> io::Result<SlackyConf> {
    let path = conf_file_path();
    if path.exists() {
        let mut file = fs::File::open(&path)?;
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        Ok(toml::from_str(&buf).unwrap())
    } else {
        Ok(SlackyConf::default())
    }
}

fn create_conf() -> io::Result<()> {
    let conf = SlackyConf {
        webhook_url: "http://hoge.com".into(),
        channel: Some("hoge".into()),
        icon: Some("hoge".into()),
    };
    println!("config path:\t{:?}", conf_file_path());
    println!("config toml:\n{}", toml::to_string(&conf).unwrap());
    write_conf_file(&conf, &conf_file_path())?;
    Ok(())
}

fn conf_file_path() -> PathBuf {
    let mut dest_dir = env::home_dir().unwrap_or_default();
    dest_dir.push(format!(".{}", PKG_NAME));
    dest_dir.push("config.toml");
    dest_dir
}

fn write_conf_file(conf: &SlackyConf, path: &Path) -> io::Result<()> {
    let directory = path.parent().unwrap();
    // println!("{:?}", directory);

    if !directory.exists() {
        fs::create_dir_all(directory)?;
    }

    fs::write(path, toml::to_string(&conf).unwrap())?;
    Ok(())
}
