use serde::{Deserialize, Serialize};
use std::fmt;

// https://www.rfc-editor.org/rfc/rfc1035 4.1.1
#[derive(Serialize, Deserialize)]
pub struct Header {
    // Identifier.
    pub id: u16,

    // A one bit field that specifies whether this message is a query (0), or a response (1).
    pub qr: u8,

    // A four bit field that specifies kind of query.
    pub opcode: u8,

    // Authoritative Answer.
    pub aa: u8,

    // Specifies that this message was truncated.
    pub tc: u8,

    // Recursion Desired.
    pub rd: u8,

    // Recursion Available.
    pub ra: u8,

    // Reserved for future use. Must be zero in all queries and responses.
    pub z: u8,

    // Response code.
    pub rcode: u8,

    // Number of entries in the question section.
    pub qdcount: u16,

    // Number of resource records in the answer section.
    pub ancount: u16,

    // Number of name server resource records in the authority records section.
    pub nscount: u16,

    // Number of resource records in the additional records section.
    pub arcount: u16,
}

impl fmt::Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "id: {}\nresponse: {}\nopcode: {}\nauthoritative: {}\ntruncated: {}\nrecursion desired: {}\nrecursion available: {}\nreserved: {}\nrcode: {}\nquestion: {}\nanswer: {}\nauthority: {}\nadditional: {}",
            self.id,
            self.qr,
            self.opcode,
            self.aa,
            self.tc,
            self.rd,
            self.ra,
            self.z,
            self.rcode,
            self.qdcount,
            self.ancount,
            self.nscount,
            self.arcount,
        )
    }
}
