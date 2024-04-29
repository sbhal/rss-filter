
use crate::post;
// use rss::Post;
use reqwest::Client;
// use lettre::{SmtpTransport, Message, Transport};

#[derive(Debug)]
pub enum NotificationChannel {
    Telegram(String),
    // Slack(String),
    // Email(String),
}

pub fn send_notification(post: &post::Post, channel: &NotificationChannel) -> Result<(), Box<dyn std::error::Error>> {
    match channel {
        NotificationChannel::Telegram(bot_token) => send_telegram_notification(bot_token, post),
        // NotificationChannel::Slack(webhook_url) => send_slack_notification(webhook_url, post),
        // NotificationChannel::Email(email_address) => send_email_notification(email_address, post),
    }
}

fn send_telegram_notification(bot_token: &str, post: &post::Post) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://api.telegram.org/bot{}/sendMessage", bot_token);
    let message = format!("New RSS Post:\n\nTitle: {}\nLink: {}", post.title, post.url);

    let params = [
        ("chat_id", "@your_channel_name"),
        ("text", &message),
    ];

    let _response = client.post(&url)
        .form(&params)
        .send();

    Ok(())
}
//
// fn send_slack_notification(webhook_url: &str, post: &Post) -> Result<(), Box<dyn std::error::Error>> {
//     let client = Client::new();
//     let message = format!("New RSS Post:\n\nTitle: {}\nLink: {}", post.title(), post.link());
//
//     let payload = json!({
//         "text": message,
//     });
//
//     let _response = client.post(webhook_url)
//         .json(&payload)
//         .send()?;
//
//     Ok(())
// }
//
// fn send_email_notification(email_address: &str, post: &Post) -> Result<(), Box<dyn std::error::Error>> {
//     let email = Message::builder()
//         .from("your_email@example.com".parse()?)
//         .to(email_address.parse()?)
//         .subject("New RSS Post")
//         .body(format!("Title: {}\nLink: {}", post.title(), post.link()))?;
//
//     let creds = Credentials::new();
//
//     let mailer = SmtpTransport::relay("smtp.example.com")?
//         .credentials(creds)
//         .build();
//
//     mailer.send(&email)?;
//
//     Ok(())
// }