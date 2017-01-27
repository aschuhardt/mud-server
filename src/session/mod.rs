//session.rs

mod remote_client;

use std::collections::{HashMap, VecDeque};
use uuid::Uuid;
use configuration::Configuration;
use self::remote_client::{RemoteClient, Message, Request, Response};

pub struct Session<'a> {
    config: &'a Configuration,
    message_queue: VecDeque<Message>,
    request_queue: VecDeque<Request>,
    remote_clients: HashMap<&'a Uuid, RemoteClient>,
}

impl<'a> Session<'a> {
    pub fn new(conf: &Configuration) -> Session {
        Session {
            config: conf,
            message_queue: VecDeque::new(),
            request_queue: VecDeque::new(),
            remote_clients: HashMap::new(),
        }
    }

    pub fn run() {

    }
}
