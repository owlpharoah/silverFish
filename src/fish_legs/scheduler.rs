use std::collections::HashSet;

use url::Url;

use crate::fish_legs::frontier::Frontier;

pub struct Scheduler {
    pub visited: HashSet<Url>,
}

impl Scheduler {
    pub fn run(&mut self, url: &Url, frontier: &mut Frontier) {
        if self.visited.insert(url.clone()) {
            frontier.push(url);
        }
    }
}
