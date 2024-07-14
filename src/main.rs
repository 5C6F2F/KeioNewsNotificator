mod config;
mod error;
mod line;
mod news;
use crate::error::RecordError;
use config::Config;
use error::Error;
use news::{NewNews, NewsContent, OldNews};
use std::process::exit;

#[macro_use]
extern crate serde_derive;

static HOMEPAGE_URL: &str = "https://www.hs.keio.ac.jp";

#[tokio::main]
async fn main() {
    let mut current_exe = std::env::current_exe().expect("Failed to get current exe path");
    current_exe.pop();
    std::env::set_current_dir(current_exe).expect("Failed to set directory");

    match exe().await {
        Ok(()) => exit(0),
        Err(err) => {
            err.record();
            exit(1);
        }
    }
}

async fn exe() -> Result<(), Error> {
    let new_news = NewNews::new(HOMEPAGE_URL.to_string());
    let new_news = new_news.fetch_news().await?;

    let old_news = OldNews::new();
    let old_news = old_news.load()?;

    let difference = new_news.extract_difference(old_news);

    if difference.is_empty() {
        exit(0);
    }

    new_news.update_old_news()?;

    let message = to_line_message(difference);

    Config::new().line.send(message)?;

    Ok(())
}

fn to_line_message(new_contents: Vec<NewsContent>) -> String {
    let now = chrono::Local::now().format("%Y年%m月%d日%H時%M分");
    let mut message = format!("\n{}\n{}\n{}", now, HOMEPAGE_URL, "~~新しいニュース~~");

    for contents in new_contents {
        message.push_str(&contents.to_line_message());
    }

    message
}
