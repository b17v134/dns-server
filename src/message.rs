// https://www.rfc-editor.org/rfc/rfc1035 4.1.1
pub struct Header {
    // Identifier.
    pub id: u16,

    // A one bit field that specifies whether this message is a query (0), or a response (1).
    qr: u8,

    // A four bit field that specifies kind of query.
    opcode: u8,

    // Authoritative Answer.
    aa: u8,

    // Specifies that this message was truncated.
    tc: u8,

    // Recursion Desired.
    rd: u8,

    // Recursion Available.
    ra: u8,

    // Reserved for future use. Must be zero in all queries and responses.
    z: u8,

    // Response code.
    rcode: u8,

    // Number of entries in the question section.
    qdcount: u16,

    // Number of resource records in the answer section.
    ancount: u16,

    // Number of name server resource records in the authority records section.
    nscount: u16,

    // Number of resource records in the additional records section.
    arcount: u16,
}

// https://www.rfc-editor.org/rfc/rfc1035 4.1.2
pub struct Question {
    qname: String,
    qtype: u16,
    qclass: u16
}

// https://www.rfc-editor.org/rfc/rfc1035 4.1.3
pub struct ResourceRecord {
    name: String,
    _type: u16,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: Vec<u8>
}

pub struct Message {
    pub Hdr: Header,
    questions: Vec<Question>,
    answers: Vec<ResourceRecord>,
    authority_records: Vec<ResourceRecord>,
    additional_records: Vec<ResourceRecord>
}

fn read_u8(buf: &Vec<u8>, pos: usize) -> u8 {
    return buf[pos];
}

fn read_u16(buf: &Vec<u8>, pos: usize) -> u16 {
    return u16::from(buf[pos]) * 0x100 + u16::from(buf[pos+ 1]);
}

fn read_u32(buf: &Vec<u8>, pos: usize) -> u32 {
    return 
        u32::from(buf[pos]) * 0x1000000 + 
        u32::from(buf[pos+ 1]) * 0x10000 +
        u32::from(buf[pos + 2]) *0x100 + 
        u32::from(buf[pos + 3]);
}

fn read_header(buf: Vec<u8>) -> Header {
    let flags = read_u16(&buf, 2);

    let header = Header{
        id: read_u16(&buf, 0),
        qr: ((flags & (1 << 15)) >> 15) as u8,
        opcode: ((flags & (0xF << 11)) >> 11) as u8,
        aa: ((flags & (1 << 10)) >> 10) as u8,
        tc: ((flags & (1 << 9)) >> 9) as u8,
        rd: ((flags & (1 << 8)) >> 8) as u8,
        ra: ((flags & (1 << 7)) >> 7) as u8,
        z: ((flags & (0x7 << 4)) >> 4) as u8,
        rcode: (flags & 0xF) as u8,

        qdcount: read_u16(&buf, 4),
        ancount: read_u16(&buf, 6),
        nscount: read_u16(&buf, 8),
        arcount: read_u16(&buf, 10),
    };

    return header;
}

pub fn GetMessage(buf: Vec<u8>) -> Message {
    let header = read_header(buf);

    return Message { 
        Hdr: header, 
        questions: Vec::new(), 
        answers: Vec::new(), 
        authority_records: Vec::new(), 
        additional_records: Vec::new(),
    };

}