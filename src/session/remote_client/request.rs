//request.rs

use uuid::Uuid;

#[derive(RustcDecodable, RustcEncodable)]
pub struct Request {
    pub id: Uuid,
}
