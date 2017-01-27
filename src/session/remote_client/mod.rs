//remote_client.rs

mod message;
mod request;
mod response;

pub use self::message::Message;
pub use self::request::Request;
pub use self::response::Response;

pub struct RemoteClient {

}
