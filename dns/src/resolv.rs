use rand::Rng;
use std::io::Error;
use std::net::UdpSocket;

use crate::classes::dns_class_to_u16;
use crate::header::Header;
use crate::message::Message;
use crate::question::Question;
use crate::request::Request;

/// # Errors
///
/// Will return `Err` if cannot send or receive message on a socket.
pub fn resolv(request: Request) -> Result<Message, Error> {
    let question = Question {
        qname: request.qname,
        qtype: request.type_,
        qclass: dns_class_to_u16(request.class.as_str()),
    };
    let mut buf = [0; 2048];
    let length = create_request_buf(buf.as_mut_slice(), &question);

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

fn create_request_buf(buf: &mut [u8], question: &Question) -> usize {
    let header = create_header();
    let header_length = write_header(buf, &header);
    let length = write_question(&mut buf[header_length..], question);

    header_length + length
}

fn write_question(buf: &mut [u8], question: &Question) -> usize {
    let length = write_qname(buf, question.qname.as_str());
    write_u16(&mut buf[length + 1..], question.qtype);
    write_u16(&mut buf[length + 3..], question.qclass);

    length + 5
}

fn create_header() -> Header {
    let mut rnd = rand::thread_rng();
    Header {
        id: rnd.gen_range(0..0xffff),
        qr: 0,
        opcode: 0,
        aa: 0,
        tc: 0,
        rd: 1,
        ra: 0,
        z: 0,
        rcode: 0,
        qdcount: 1,
        ancount: 0,
        nscount: 0,
        arcount: 0,
    }
}

fn write_header(buf: &mut [u8], header: &Header) -> usize {
    write_u16(buf, header.id);
    write_u16(&mut buf[2..], get_flags(header));
    write_u16(&mut buf[4..], header.qdcount);
    write_u16(&mut buf[6..], header.ancount);
    write_u16(&mut buf[8..], header.ancount);
    write_u16(&mut buf[10..], header.arcount);

    12
}

fn write_u16(buf: &mut [u8], value: u16) {
    buf[0] = ((value & 0xFF00) >> 8) as u8;
    buf[1] = (value & 0xFF) as u8;
}

fn write_qname(buf: &mut [u8], qname: &str) -> usize {
    let s = qname.as_bytes();
    let mut length = s.len();
    if char::from(s[length - 1]) == '.' {
        length -= 1;
    }
    let mut write_count = true;
    let mut pos = 0;
    for i in 0..length {
        if write_count {
            let mut count = 0;
            let mut j = i;
            while (j < length) && (char::from(s[j]) != '.') {
                j += 1;
                count += 1;
            }
            buf[pos] = count;
            pos += 1;
            write_count = false;
        }
        if char::from(s[i]) == '.' {
            write_count = true;
            continue;
        }
        buf[pos] = s[i];
        pos += 1;
    }

    pos
}

fn get_flags(header: &Header) -> u16 {
    (u16::from(header.qr & 1) << 15) + (u16::from(header.rd & 1) << 8)
}
