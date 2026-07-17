use std::collections::HashSet;

use url::Url;

use crate::fish_legs::{
    config::Config, fetcher::Fetcher, frontier::Frontier, parser::Parser, scheduler::Scheduler,
};

pub struct Crawler {
    pub config: Config,
    pub fetcher: Fetcher,
    pub frontier: Frontier,
    pub parser: Parser,
    pub scheduler: Scheduler,
}

impl Crawler {
    pub async fn run(&mut self) -> HashSet<Url> {
        let mut depth = self.config.max_depth;
        let mut n = self.frontier.len();
        while depth > 0 {
            for _ in 0..n {
                let target = self.frontier.pop();

                if let Some(link) = target {
                    let html = self.fetcher.fetch(&link).await;

                    if html.is_err() {
                        continue;
                    }
                    let html = html.unwrap();

                    let mut link = link;
                    if let Ok(mut path) = link.path_segments_mut() {
                        path.clear();
                    }

                    link.set_query(None);
                    link.set_fragment(None);

                    let new_urls = self.parser.extract_links(
                        &html,
                        &Url::parse(&link.as_str())
                            .expect(&format!("Couldn't resolve domain for: {}", link)),
                        &self.config,
                    );

                    if let Ok(new_urls) = new_urls {
                        for i in new_urls {
                            self.scheduler.run(&i, &mut self.frontier);
                        }
                    }
                }
            }
            depth -= 1;
            n = self.frontier.len();
        }
        return self.scheduler.visited.clone();
    }
}
