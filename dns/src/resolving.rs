use std::io::Error;

use crate::message::Message;
use crate::protocol::Protocol;
use crate::request::Request;
use crate::resolv::Resolv;
use crate::tcp::Tcp;
use crate::udp::Udp;

/// # Errors
///
/// Will return `Err` if cannot send or receive message on a socket.
///
/// # Panics
///
/// protocol doesn't supported now.
pub fn resolv(request: Request) -> Result<Message, Error> {
    match request.protocol {
        Protocol::Tcp => Tcp::resolv(request),
        Protocol::Udp => Udp::resolv(request),
        _ => todo!(),
    }
}
