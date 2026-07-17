use std::collections::VecDeque;

use url::Url;

pub struct Frontier {
    structure: VecDeque<Url>,
}

impl Frontier {
    pub fn new(seed: Vec<Url>) -> Self {
        let structure = VecDeque::from(seed);
        Self { structure }
    }
    pub fn push(&mut self, url: &Url) {
        self.structure.push_back(url.clone());
    }
    pub fn pop(&mut self) -> Option<Url> {
        self.structure.pop_front()
    }
    pub fn len(&self) -> usize {
        self.structure.len()
    }
    pub fn is_empty(&self) -> bool {
        self.structure.is_empty()
    }
}
