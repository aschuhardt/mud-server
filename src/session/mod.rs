//session.rs

mod remote_client;
mod request_cache;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::collections::{HashMap, VecDeque};
use std::thread::{Thread, JoinHandle, spawn};
use std::sync::mpsc::{Sender, channel};
use uuid::Uuid;
use configuration::Configuration;
use self::remote_client::{RemoteClient, Message, Request, Response};
use self::request_cache::RequestCache;

pub struct Session<'a> {
    config: &'a Configuration,
    message_queue: VecDeque<Message>,
    response_queue: VecDeque<Request>,
    remote_clients: HashMap<&'a Uuid, RemoteClient>,
    quit: bool,
}

impl<'a> Session<'a> {
    pub fn new(conf: &'a Configuration) -> Session {
        Session {
            config: conf,
            message_queue: VecDeque::new(),
            response_queue: VecDeque::new(),
            remote_clients: HashMap::new(),
            quit: false,
        }
    }

    pub fn run(&self) {
        let (req_tx, req_rs) = channel();
        self.init_listener(&req_tx);
        while !self.quit {
            if let Ok(c) = req_rs.recv() {
                //process request cache...
                for req in c.iter() {
                    if self.config.debug_mode { println!("Processing request: {}", req.id) };
                }
            }
        }
    }

    fn init_listener(&self, tx: &RequestCache) {
        for _ in 0..self.config.max_request_cache_count {
            let port = self.config.network_port;
            let listener_tx = tx.clone();
            spawn(move || {
                loop {

                }
            });
        }
    }
}
