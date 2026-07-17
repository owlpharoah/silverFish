use std::{
    collections::HashSet,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering::Relaxed},
    },
    time::Duration,
};

use url::Url;

use crate::fish_legs::{
    config::Config, fetcher::Fetcher, frontier::Frontier, parser::Parser, scheduler::Scheduler,
    types::CrawlTask,
};

pub struct Crawler {
    pub config: Config,
    pub fetcher: Fetcher,
    pub frontier: Arc<Frontier>,
    pub parser: Parser,
    pub scheduler: Arc<Scheduler>,
}

impl Crawler {
    pub async fn run(&mut self) -> HashSet<Url> {
        let active_workers = Arc::new(AtomicU32::new(0));
        let mut handlers = Vec::new();
        for _ in 0..self.config.max_conc {
            handlers.push(tokio::spawn(worker_func(
                Crawler {
                    config: self.config.clone(),
                    fetcher: self.fetcher.clone(),
                    frontier: self.frontier.clone(),
                    parser: self.parser.clone(),
                    scheduler: self.scheduler.clone(),
                },
                active_workers.clone(),
            )));
        }
        for i in handlers {
            i.await.unwrap();
        }
        return self.scheduler.visited.lock().await.clone();
    }
}

async fn worker_func(c: Crawler, worker_count: Arc<AtomicU32>) {
    loop {
        let target = c.frontier.pop().await;
        if let Some(task) = target {
            if task.depth > c.config.max_depth {
                continue;
            }
            worker_count.fetch_add(1, Relaxed);
            let links = {
                let html = c.fetcher.fetch(&task.url).await;

                if html.is_err() {
                    worker_count.fetch_sub(1, Relaxed);
                    continue;
                }
                let html = html.unwrap();

                let mut link = task.url;
                if let Ok(mut path) = link.path_segments_mut() {
                    path.clear();
                }

                link.set_query(None);
                link.set_fragment(None);

                let new_urls = c.parser.extract_links(
                    &html,
                    &Url::parse(&link.as_str())
                        .expect(&format!("Couldn't resolve domain for: {}", link)),
                    &c.config,
                );
                new_urls
            };

            if let Ok(new_urls) = links {
                for i in new_urls {
                    if !c.scheduler.can_schedule(c.config.max_pages).await {
                        break;
                    }
                    c.scheduler
                        .run(
                            &CrawlTask {
                                url: i,
                                depth: task.depth + 1,
                            },
                            &c.frontier,
                        )
                        .await;
                }
            }
            worker_count.fetch_sub(1, Relaxed);
        } else {
            if worker_count.load(Relaxed) > 0 {
                tokio::time::sleep(Duration::from_millis(200)).await;
            } else {
                break;
            }
        }
    }
}
