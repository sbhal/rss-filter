
use chrono::{DateTime, FixedOffset};


pub struct Post {
    pub url: String,
    pub title: String,
    pub description: String,
    pub md: String,
    pub pubdate: DateTime<FixedOffset>,
    pub guid: String,
}

impl Post {
    pub fn new(md: &str, url: &str, title: &str, description: &str, pubdate: &str, guid: String) -> Self {
        let pubdate = match DateTime::parse_from_rfc2822(pubdate) {
            Ok(d) => d,
            Err(err) => panic!("{}", err),
        };
        let post = Post {
            pubdate,
            url: String::from(url),
            title: String::from(title),
            description: String::from(description),
            md: String::from(md),
            guid: String::from(guid),
        };
        post
    }

    fn slug(&self) -> Result<&str, String> {
        let parts = self.url.split("/");
        let slug = match parts.last() {
            Some(slug) => Ok(slug),
            None => Err(String::from("unable to split url to find the slug")),
        };
        slug
    }

    fn filename(&self) -> String {
        let slug = self.slug().expect("unable to get slug for filename");
        let date = self.pubdate.format("%m-%d-%Y");
        format!("{}-{}.md", date, slug)
    }
}