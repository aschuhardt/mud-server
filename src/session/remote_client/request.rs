//request.rs

use uuid::Uuid;

#[derive(RustcDecodable, RustcEncodable, Clone, Copy)]
pub struct Request {
    pub id: Uuid,
}
