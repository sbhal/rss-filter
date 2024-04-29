use std::collections::HashSet;
// use std::hash::{Hash, Hasher};


pub struct Deduplicator {
    recent_posts: HashSet<String>,
    max_size: usize,
}

impl Deduplicator {
    pub fn new(max_size: usize) -> Self {
        Deduplicator {
            recent_posts: HashSet::with_capacity(max_size),
            max_size,
        }
    }

    pub fn dedup(&mut self, guid: String) -> bool {
        if self.recent_posts.len() >= self.max_size {
            self.recent_posts.drain().next();
        }
        self.recent_posts.insert(guid)
    }
}