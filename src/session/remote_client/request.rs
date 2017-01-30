//request.rs

use std::collections::HashMap;
use uuid::Uuid;
use rustc_serialize::json;

#[derive(Clone)]
pub enum RequestType {
    Connect,
    Disconnect,
    Heartbeat,
}

//update this when adding new connection types
pub const REQUEST_TYPE_VERB_MAP: [(&'static str, RequestType); 3] = [
    ("CONNECT", RequestType::Connect),
    ("DISCONNECT", RequestType::Disconnect),
    ("HEARTBEAT", RequestType::Heartbeat)
];

#[derive(Clone)]
pub struct Request {
    pub id: Uuid,
    pub client_id: Uuid,
    pub req_type: RequestType,
    pub contents: Vec<u8>,
}

impl Request {
    pub fn new(serialized: String, type_hashes: HashMap<Uuid, RequestType>) -> Option<Request> {
        let req: SerializableRequest = match json::decode(serialized.as_str()) {
            Ok(r) => r,
            Err(_) => panic!("Encountered malformed request JSON: {}", serialized),
        };
        let req_type_hash: Uuid = req.req_type;
        if type_hashes.contains_key(&req_type_hash) {
            Some(Request {
                id: Uuid::new_v4(),
                client_id: req.client_id,
                req_type: type_hashes[&req_type_hash].clone(),
                contents: req.contents,
            })
        } else {
            None
        }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct SerializableRequest {
    pub client_id: Uuid,
    pub req_type: Uuid,
    pub contents: Vec<u8>,
}
