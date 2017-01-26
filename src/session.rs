//session.rs

use std::collections::HashMap;
use std::collections::VecDeque;
use uuid::Uuid;
use configuration::Configuration;
use message::Message;
use request::Request;
use remote_client::RemoteClient;

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
