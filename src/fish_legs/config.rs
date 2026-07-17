use std::collections::HashSet;

pub struct Config {
    pub max_depth: u32,
    pub restricted_domains: Option<HashSet<String>>,
    pub max_pages: usize,
    pub max_conc: usize,
}

impl Config {
    pub fn default() -> Self {
        Self {
            max_depth: 1,
            restricted_domains: None,
            max_pages: 50usize,
            max_conc: 1usize,
        }
    }
}
