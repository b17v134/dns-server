use crate::message::Message;
use crate::request::Request;
use std::io::Error;

pub trait Resolv {
    /// # Errors
    ///
    /// Will return `Err` if cannot send or receive message on a socket or can't
    /// read/write message.
    fn resolv(request: Request) -> Result<Message, Error>;
}
