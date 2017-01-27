//request_cache.rs

use uuid::Uuid;
use std::collections::HashMap;
use super::remote_client::request::Request;

pub struct RequestCache<'a> {
    pub requests: HashMap<&'a Uuid, &'a Request>,
    show_debug: &'a bool,
    max_size: &'a u32,
}

impl<'a> RequestCache<'a> {
    pub fn new(debug_mode: &'a bool, capacity: &'a u32) -> RequestCache<'a> {
        RequestCache {
            requests: HashMap::new(),
            show_debug: debug_mode,
            max_size: capacity,
        }
    }

    pub fn add(&mut self, req: &Request) {
        //clear cache if size exceeds limit
        if self.should_clear() {
            self.requests.clear();
        }
        //add request to cache if it isn't already present
        if !self.requests.contains_key(&req.id) {
            self.requests.insert(&req.id, req);
        } else if self.show_debug {
            println!("Dropped a duplicate packet: {}", req.id);
        }
    }

    fn should_clear(&self) -> bool {
        self.requests.len() >= self.max_size
    }
}
