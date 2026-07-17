use std::collections::HashSet;

use scraper::{Html, Selector};
use url::Url;

use crate::fish_legs::config::Config;

#[derive(Clone)]
pub struct Parser;

impl Parser {
    pub fn extract_links(
        &self,
        html: &Html,
        base: &Url,
        cnfg: &Config,
    ) -> Result<HashSet<Url>, url::ParseError> {
        let mut result: HashSet<Url> = HashSet::new();
        let link_selector = Selector::parse("a").unwrap();
        let document = html;

        // get all links into vector
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                if let Ok(link) = base.join(href) {
                    match link.scheme() {
                        "http" | "https" => {}
                        _ => continue,
                    }
                    let mut link = link;
                    link.set_fragment(None);
                    if cnfg.restricted_domains.is_none() {
                        result.insert(link);
                    } else {
                        let ru: HashSet<String> = cnfg.restricted_domains.clone().unwrap();
                        if !ru.contains(&link.domain().unwrap().to_string()) {
                            result.insert(link);
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}
