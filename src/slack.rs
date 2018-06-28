extern crate slack_hook;

use std::result;

use self::slack_hook::{Slack, PayloadBuilder};

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
            icon: icon.map(|x| format!(":{}:", x)).unwrap_or_default(),
        }
    }
}


pub fn notify(msg: &str, config: &NotifyConfig) -> result::Result<(), slack_hook::Error> {
    let slack = Slack::new(config.url.as_ref()).unwrap();
    let p = PayloadBuilder::new()
        .text(msg)
        .channel(config.channel.as_ref())
        .icon_emoji(config.icon.as_ref())
        .build()
        .unwrap();

    slack.send(&p)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notify_config_new() {
        let config = NotifyConfig::new("http://hoge.com", Some("channel"), Some("icon"));
        assert_eq!(config.url, "http://hoge.com");
        assert_eq!(config.channel, "channel");
        assert_eq!(config.icon, ":icon:");
    }

    #[test]
    fn notify_config_new_when_channel_none() {
        let config = NotifyConfig::new("http://hoge.com", None, Some("icon"));
        assert_eq!(config.channel, "");
    }

    #[test]
    fn notify_config_new_when_icon_none() {
        let config = NotifyConfig::new("http://hoge.com", Some("hoge"), None);
        assert_eq!(config.icon, "");
    }
}
