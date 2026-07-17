use std::collections::HashSet;

use tokio::sync::Mutex;
use url::Url;

use crate::fish_legs::{frontier::Frontier, types::CrawlTask};

pub struct Scheduler {
    pub visited: Mutex<HashSet<Url>>,
}

impl Scheduler {
    pub async fn run(&self, url: &CrawlTask, frontier: &Frontier) {
        if self.visited.lock().await.insert(url.url.clone()) {
            frontier.push(url).await;
        }
    }

    pub async fn can_schedule(&self, x: usize) -> bool {
        self.visited.lock().await.len() <= x
    }
}
