use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    sync::Arc,
    time::{Duration, Instant},
};

use reqwest::Client;
use tokio::{self, sync::Mutex};

pub mod fish_legs;
use url::Url;

use crate::fish_legs::{
    config::Config, crawler::Crawler, fetcher::Fetcher, frontier::Frontier, parser::Parser,
    scheduler::Scheduler,
};

#[tokio::main]
async fn main() {
    let seed = Url::parse("https://www.scrapethissite.com/pages/").unwrap();
    let mut spider = Crawler{
        config: Config::default(),
        fetcher: Fetcher{
            client: Client::builder().timeout(Duration::from_millis(2000)).user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36").build().expect("Failed to hatch spider!")
        },
        frontier: Arc::new(Frontier::new(seed)),
        parser: Parser,
        scheduler: Arc::new(Scheduler{
            visited: Mutex::new(HashSet::new())
        }),
    };

    let start = Instant::now();
    let v = spider.run().await;
    let end = start.elapsed();
    println!("time elapsed:{}", end.as_secs_f32());
    let file = File::create("urls.txt").unwrap();
    let mut writer = BufWriter::new(file);
    for i in v {
        writeln!(writer, "{}", i).expect(&format!("Couldnt write to file: {}", i));
    }
}
