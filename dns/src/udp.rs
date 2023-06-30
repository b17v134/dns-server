use std::io::Error;
use std::net::UdpSocket;

use crate::classes::dns_class_to_u16;
use crate::message::Message;
use crate::question::Question;
use crate::request::Request;
use crate::resolv::Resolv;

pub struct Udp {}

impl Resolv for Udp {
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
        let result = UdpSocket::bind("[::]:0");
        let socket: UdpSocket = match result {
            Ok(s) => s,
            Err(e) => return Err(e),
        };

        let send_buf: &[u8] = &buf[0..length];
        let result = socket.send_to(
            send_buf,
            request.server + ":" + request.port.to_string().as_str(),
        );
        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        let mut buf2 = [0; 2048];
        let result = socket.recv_from(&mut buf2);

        match result {
            Ok(_) => {}
            Err(e) => return Err(e),
        }
        Ok(Message::from(&buf2))
    }
}
