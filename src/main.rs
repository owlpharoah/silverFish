use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
    time::Duration,
};

use reqwest::{Client, Error, StatusCode, header::CONTENT_TYPE};
use scraper::{Html, Selector};
use tokio::{sync::Semaphore, time};
use tracing::info;
use tracing_subscriber::EnvFilter;
use url::Url;

struct Spider {
    client: Client,
    restricted_domains: Option<Vec<String>>,
    max_pages: Option<u32>,
    max_conc: Arc<Semaphore>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let restricted = None; //Some(Vec::from(["en.wikipedia.org".to_string()]));
    let max_pages = Some(u16::MAX as u32);
    let peter = Spider::new(restricted, max_pages, None);
    let link = Url::parse("https://www.scrapethissite.com/pages/").unwrap();
    let start_time = time::Instant::now();
    let crawled_links = peter.bfs_crawl(&link, 2).await;
    let end_time = start_time.elapsed();
    println!("end_time:{}", end_time.as_secs_f32());
    let file = File::create("urls.txt").unwrap();
    let mut writer = BufWriter::new(file);
    for i in crawled_links {
        writeln!(writer, "{}", i.to_string()).unwrap();
    }
}

impl Spider {
    fn new(
        restricted_domains: Option<Vec<String>>,
        max_pages: Option<u32>,
        max_conc: Option<u32>,
    ) -> Self {
        let max_pages = max_pages.unwrap_or(1000);
        let max_conc = max_conc.unwrap_or(10);
        Self{
            client: Client::builder().timeout(Duration::from_millis(2000)).user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36").build().expect("Failed to hatch spider!"),
            restricted_domains,
            max_pages: Some(max_pages),
            max_conc: Arc::new(Semaphore::new(max_conc as usize))
        }
    }

    async fn extract_links(&self, url: &Url) -> Result<HashSet<Url>, Error> {
        let mut result: HashSet<Url> = HashSet::new();

        // client
        let client = &self.client;

        // get html
        let data = client
            .get(url.as_str())
            .send()
            .await
            .expect("Spider didnt pull data!");

        if !data.status().is_success()
            || !(data
                .headers()
                .get(CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                != Some("text/html"))
        {
            return Ok(result);
        }

        let data = data.text().await.expect("Spider didnt pull text data!");

        let document = Html::parse_document(&data);

        //link selector
        let link_selector = Selector::parse("a").unwrap();

        // get all links into vector
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(link) = url.join(href) {
                    match link.scheme() {
                        "http" | "https" => {}
                        _ => continue,
                    }
                    let mut link = link;
                    link.set_fragment(None);
                    if self.restricted_domains.is_none() {
                        result.insert(link);
                    } else {
                        let ru: Vec<String> = self.restricted_domains.clone().unwrap();
                        if !ru.contains(&link.domain().unwrap().to_string()) {
                            result.insert(link);
                        }
                    }
                }
            }
        }
        Ok(result)
    }

    async fn bfs_crawl(&self, url: &Url, mut depth: u32) -> HashSet<Url> {
        let mut frontier: VecDeque<Url> = VecDeque::new();
        let mut visited: HashSet<Url> = HashSet::new();
        frontier.push_back(url.clone());
        visited.insert(url.clone());
        let mut n = frontier.len();

        while depth > 0 {
            for _ in 0..n {
                if let Some(current) = frontier.pop_front() {
                    let links = self.extract_links(&current).await.unwrap();
                    for i in links {
                        if visited.len() >= self.max_pages.unwrap() as usize {
                            return visited;
                        }
                        if visited.insert(i.clone()) {
                            frontier.push_back(i);
                        }
                    }
                }
            }
            depth -= 1;
            n = frontier.len();
        }
        return visited;
    }
}
