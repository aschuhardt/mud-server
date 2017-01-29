//session.rs

mod remote_client;
mod request_cache;

//TODO: figure out a better way of organizing imports so as to avoid this mess
use std::io::{Read};
use std::str::FromStr;
use std::net::{TcpListener, TcpStream, Ipv4Addr, SocketAddrV4};
use std::collections::{HashMap, VecDeque};
use std::thread::{spawn};
use std::sync::mpsc::{Sender, Receiver, channel};
use time::{Duration, PreciseTime};
use uuid::{Uuid, NAMESPACE_OID};
use configuration::Configuration;
use self::remote_client::{RemoteClient, Message, Request, RequestType};
use self::remote_client::request::REQUEST_TYPE_VERB_MAP;
use self::request_cache::RequestCache;

const CACHE_EMIT_INTERVAL: i64 = 60;

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
        self.init_listener(req_tx);
        while !self.quit {
            if let Ok(c) = req_rs.recv() {
                //process request cache...
                for (id, req) in &c.requests {
                    if self.config.debug_mode {
                        println!("Processing request: {} from client {}", id, req.client_id);
                    };
                }
            }
        }
    }

    fn init_listener(&self, tx: Sender<RequestCache>) {
        let local_port = self.config.network_port;
        let cache_capacity = self.config.max_request_cache_count;
        let debug_mode = self.config.debug_mode;
        let type_hashes = self.create_request_type_hashes(&self.config.request_validation_token);
        spawn(move || {
            let (l_tx, l_rx) = channel();
            //initialize loop that will wait for requests returned from listeners and add them to a
            //  request cache to be transmitted via master_tx
            Session::funnel_requests_into_cache(tx, l_rx, cache_capacity, debug_mode);

            //start listening on the configured port and start threads for each incoming request
            let ipv4 = Ipv4Addr::from_str("127.0.0.1").unwrap();
            let addr = SocketAddrV4::new(ipv4, local_port);
            if let Ok(listener) = TcpListener::bind(addr) {
                if debug_mode {
                    println!("Started listening on port {}...", local_port);
                }
                for stream in listener.incoming() {
                    if let Ok(mut s) = stream {
                        Session::init_listener_thread(l_tx.clone(), &mut s, type_hashes.clone());
                    }
                }
            } else {
                println!("Failed to bind to port {}!", local_port);
            }
        });
    }

    fn create_request_type_hashes(&self, validation_token: &str) -> HashMap<Uuid, RequestType> {
        // let validation_token = self.config.request_validation_token;
        let mut hashes: HashMap<Uuid, RequestType> = HashMap::new();
        //TODO: figure out why the linter says I need to write "ref t" here
        for &(s, ref t) in &REQUEST_TYPE_VERB_MAP {
            let verb_token = format!("{}_{}", s, validation_token);
            let hash = Uuid::new_v5(&NAMESPACE_OID, verb_token.as_str());
            hashes.insert(hash, t.clone());
            if self.config.debug_mode {
                println!("Hash created for request type \"{}\": {}", s, hash.hyphenated());
            }
        }
        hashes
    }

    fn funnel_requests_into_cache(cache_tx: Sender<RequestCache>, l_rx: Receiver<Request>,
                                  cache_cap: usize, debug_mode: bool) {
        let mut req_cache = RequestCache::new(debug_mode, cache_cap);
        spawn(move || {
            let mut emit_interval_start = PreciseTime::now();
            loop {
                //cache incoming requsts from listener threads
                if let Ok(req) = l_rx.try_recv() {
                    req_cache.add(req);
                }

                //here we are waiting a constant time interval before sending the request cache
                //  back to the main session thread.  We are doing this in order to give the cache
                //  enough time to build up some requests before sending them to the main thread,
                //  ensuring (hopefully) that the main session thread has got a healthy sizeable
                //  workload.
                //TODO: Consider coming up with a less-hokey way of ensuring that the cache gets utilized correctly.
                if emit_interval_start.to(PreciseTime::now()) >= Duration::microseconds(CACHE_EMIT_INTERVAL) {
                    emit_interval_start = PreciseTime::now();
                    if let Err(why) = cache_tx.send(req_cache.clone()) {
                        panic!("Failed to emit request cache to main session thread: {}", why);
                    }
                }
            }
        });
    }

    fn init_listener_thread(tx: Sender<Request>, stream: &mut TcpStream, type_hashes: HashMap<Uuid, RequestType>) {
        let mut buffer = String::new();
        if let Err(why) = stream.read_to_string(&mut buffer) {
            println!("Malformed or invalid stream buffer encountered from peer: {}. Reason: {}",
                     stream.peer_addr().unwrap(), why);
        } else if let Some(req) = Request::new(buffer, type_hashes) {
            if let Err(why) = tx.send(req) {
                println!("Unable to send request to caching thread: {}", why);
            }
        }
    }
}
