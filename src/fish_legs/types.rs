use std::collections::HashSet;
use url::Url;

#[derive(Clone)]
pub struct CrawlTask {
    pub url: Url,
    pub depth: u32,
}

#[derive(Clone)]
pub struct CrawlResult {
    pub url: Url,
    pub links: HashSet<Url>,
}
