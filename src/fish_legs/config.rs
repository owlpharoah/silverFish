use std::collections::HashSet;

#[derive(Clone)]
pub struct Config {
    pub max_depth: u32,
    pub restricted_domains: Option<HashSet<String>>,
    pub max_pages: usize,
    pub max_conc: usize,
}

impl Config {
    pub fn default() -> Self {
        Self {
            max_depth: 3,
            restricted_domains: None,
            max_pages: 1000usize,
            max_conc: 10usize,
        }
    }
}
