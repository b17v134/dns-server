use crate::protocol::Protocol;

pub struct Request {
    pub server: String,
    pub port: u16,
    pub protocol: Protocol,
    pub qname: String,
    pub type_: u16,
    pub class: String,
}
