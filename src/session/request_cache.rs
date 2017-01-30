//request_cache.rs

use uuid::Uuid;
use std::clone::Clone;
use std::collections::HashMap;
use super::remote_client::request::Request;

#[derive(Clone)]
pub struct RequestCache {
    pub requests: HashMap<Uuid, Request>,
    show_debug: bool,
    max_size: usize,
}

impl RequestCache {
    pub fn new(debug_mode: bool, capacity: usize) -> RequestCache {
        RequestCache {
            requests: HashMap::new(),
            show_debug: debug_mode,
            max_size: capacity,
        }
    }

    pub fn add(&mut self, req: Request) {
        //clear cache if size exceeds limit
        let count = self.requests.len();
        if count >= self.max_size {
            if self.show_debug {
                println!("Clearing {} requests from cache.", count);
            }
            self.requests.clear();
        }
        //add request to cache if it isn't already present
        let cache_req = req.clone();
        self.requests.entry(req.id).or_insert(cache_req);
        if self.show_debug && count == self.requests.len() {
            println!("Duplicate request was not cached: {}.", req.id);
        }
    }

    pub fn clear(&mut self) {
        self.requests.clear();
    }
}
