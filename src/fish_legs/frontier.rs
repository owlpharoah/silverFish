use std::collections::VecDeque;

use tokio::sync::Mutex;
use url::Url;

use crate::fish_legs::types::CrawlTask;

pub struct Frontier {
    structure: Mutex<VecDeque<CrawlTask>>,
}

impl Frontier {
    pub fn new(seed: Url) -> Self {
        let mut structure: VecDeque<CrawlTask> = VecDeque::new();
        structure.push_back(CrawlTask {
            url: seed,
            depth: 1,
        });
        Self {
            structure: Mutex::from(structure),
        }
    }
    pub async fn push(&self, url: &CrawlTask) {
        self.structure.lock().await.push_back(url.clone());
    }
    pub async fn pop(&self) -> Option<CrawlTask> {
        self.structure.lock().await.pop_front()
    }
    pub async fn len(&self) -> usize {
        self.structure.lock().await.len()
    }
    pub async fn is_empty(&self) -> bool {
        self.structure.lock().await.is_empty()
    }
}
