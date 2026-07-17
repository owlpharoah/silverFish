use std::collections::HashSet;
use url::Url;

pub struct CrawlTask {
    pub url: Url,
    pub depth: u32,
}

pub struct CrawlResult {
    pub url: Url,
    pub links: HashSet<Url>,
}
