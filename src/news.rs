use crate::error::{NewNewsError, OldNewsError};
use scraper::Selector;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Write},
};

static CONTENTS_PATH: &str = "contents.txt";
static NEWS_CSS_SELECTOR: &str = "#news-area > div:nth-child(1) > ul";

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NewsContent {
    pub tag: String,
    pub date: String,
    pub content: String,
}

impl NewsContent {
    pub fn new(tag: String, date: String, content: String) -> Self {
        NewsContent { tag, date, content }
    }

    pub fn to_line_message(&self) -> String {
        format!("\n{} - {}\n{}", self.date, self.tag, self.content)
    }

    fn to_file_contents(&self) -> String {
        format!("{}\n{}\n{}\n", self.tag, self.date, self.content)
    }
}

#[derive(Clone)]
pub struct NewNews {
    homepage_url: String,
    contents: Vec<NewsContent>,
}

impl NewNews {
    pub fn new(homepage_url: String) -> Self {
        NewNews {
            homepage_url,
            contents: vec![],
        }
    }

    pub async fn fetch_news(&self) -> Result<NewNews, NewNewsError> {
        let document = self.fetch_homepage_document().await?;
        let news_contents = self.extract_news_contents(document)?;

        let mut new_news = self.clone();
        new_news.contents = news_contents;
        Ok(new_news)
    }

    async fn fetch_homepage_document(&self) -> Result<scraper::Html, NewNewsError> {
        let res = reqwest::get(&self.homepage_url).await?;
        let res_text = res.text().await?;
        let document = scraper::Html::parse_document(&res_text);
        Ok(document)
    }

    fn extract_news_contents(
        &self,
        document: scraper::Html,
    ) -> Result<Vec<NewsContent>, NewNewsError> {
        let mut news_contents = vec![];
        let mut index = 1;

        loop {
            let li_selector_str = format!("{} > li:nth-child({})", NEWS_CSS_SELECTOR, index);

            let tag_selector_str = format!("{} > div.tag", li_selector_str);
            let tag = self.extract_element(&document, tag_selector_str);

            let date_selector_str = format!("{} > div:nth-child(2) > span", li_selector_str);
            let date = self.extract_element(&document, date_selector_str);

            let content_selector_str = format!("{} > div:nth-child(2) > a", li_selector_str);
            let content = self.extract_element(&document, content_selector_str);

            match (tag, date, content) {
                (Some(tag), Some(date), Some(content)) => {
                    news_contents.push(NewsContent::new(tag, date, content));
                }
                _ => break,
            }

            index += 1;
        }

        Ok(news_contents)
    }

    fn extract_element(&self, document: &scraper::Html, selector_str: String) -> Option<String> {
        let selector = self.parse_selector(selector_str);
        let element = match document.select(&selector).next() {
            Some(elem) => elem,
            None => return None,
        };

        Some(element.text().collect::<Vec<_>>()[0].to_string())
    }

    fn parse_selector(&self, selector_str: String) -> Selector {
        scraper::Selector::parse(&selector_str).expect("エラーハンドリングできません")
    }

    pub fn extract_difference(&self, old_news: OldNews) -> Vec<NewsContent> {
        let old_news: HashSet<_> = old_news.contents.into_iter().collect();
        let new_news: HashSet<_> = self.contents.clone().into_iter().collect();

        // new_newsにはあるがold_newsには無いものを抽出
        // new_newsには無いがold_newsにはあるものは無視
        let difference: HashSet<_> = new_news.difference(&old_news).collect();
        difference.into_iter().cloned().collect()
    }

    pub fn update_old_news(&self) -> Result<(), NewNewsError> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(CONTENTS_PATH)?;

        for contents in &self.contents {
            file.write_all(contents.to_file_contents().as_bytes())?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct OldNews {
    pub contents: Vec<NewsContent>,
}

impl OldNews {
    pub fn new() -> Self {
        OldNews { contents: vec![] }
    }
    pub fn load(&self) -> Result<Self, OldNewsError> {
        let mut contents = vec![];

        for line in BufReader::new(File::open(CONTENTS_PATH)?).lines() {
            contents.push(line?)
        }

        let mut news = vec![];

        for line in (0..contents.len()).step_by(3) {
            news.push(NewsContent::new(
                contents[line].clone(),
                contents[line + 1].clone(),
                contents[line + 2].clone(),
            ));
        }

        let mut old_news = self.clone();
        old_news.contents = news;
        Ok(old_news)
    }
}
