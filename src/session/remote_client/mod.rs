//remote_client.rs

pub mod message;
pub mod request;
pub mod response;

pub use self::message::Message;
pub use self::request::Request;
pub use self::response::Response;

pub struct RemoteClient {

}
