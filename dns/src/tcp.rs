use std::io::prelude::*;
use std::io::{Error, Write};
use std::net::TcpStream;

use crate::classes::dns_class_to_u16;
use crate::message::Message;
use crate::question::Question;
use crate::request::Request;
use crate::resolv::Resolv;

pub struct Tcp {}

impl Resolv for Tcp {
    fn resolv(request: Request) -> Result<Message, Error> {
        let question = Question {
            qname: request.qname,
            qtype: request.type_,
            qclass: dns_class_to_u16(request.class.as_str()),
        };
        let mut buf = [0; 2048];
        let message = Message::new(question);
        let result = message.create_request_buf(buf.as_mut_slice());
        let length = match result {
            Ok(ln) => ln,
            Err(e) => return Err(e),
        };
        let addr = request.server + ":" + request.port.to_string().as_str();
        let result = TcpStream::connect(addr);
        let mut stream = match result {
            Ok(tcp_stream) => tcp_stream,
            Err(e) => return Err(e),
        };

        let mut size_buf: [u8; 2] = [0; 2];

        let bytes = length.to_be_bytes();
        size_buf[0] = bytes[6];
        size_buf[1] = bytes[7];

        let result: Result<usize, Error> = stream.write(&size_buf);
        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        let send_buf: &[u8] = &buf[0..length];
        let result: Result<usize, Error> = stream.write(send_buf);
        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        let mut buf2 = [0; 2048];
        let result = stream.read(&mut buf2);

        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        Ok(Message::from(&buf2[2..]))
    }
}
