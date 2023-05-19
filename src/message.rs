// https://www.rfc-editor.org/rfc/rfc1035 4.1.1
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

// https://www.rfc-editor.org/rfc/rfc1035 4.1.2
pub struct Question {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16
}

// https://www.rfc-editor.org/rfc/rfc1035 4.1.3
pub struct ResourceRecord {
    pub name: String,
    pub _type: u16,
    pub class: u16,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>
}

pub struct Message {
    pub hdr: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>
}

/*fn read_u8(buf: &Vec<u8>, pos: usize) -> u8 {
    return buf[pos];
}*/

fn read_u16(buf: &Vec<u8>, pos: usize) -> u16 {
    return u16::from(buf[pos]) * 0x100 + u16::from(buf[pos+ 1]);
}

/*fn read_u32(buf: &Vec<u8>, pos: usize) -> u32 {
    return 
        u32::from(buf[pos]) * 0x1000000 + 
        u32::from(buf[pos+ 1]) * 0x10000 +
        u32::from(buf[pos + 2]) *0x100 + 
        u32::from(buf[pos + 3]);
}*/

fn read_header(buf: &Vec<u8>) -> Header {
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

fn read_qname(buf: &Vec<u8>, pos: usize, qname: &mut String) -> usize {
    let mut max_pos = pos;
    let mut length: u8;
    let mut tmp_pos = pos;
    if buf[pos] == 0 {
        qname.push('.');
        return pos;
    }

    while buf[tmp_pos] != 0 {
        length = buf[tmp_pos];
        if ((length & (0x3 << 6)) >> 6) == 0x3 {
            if max_pos < tmp_pos {
                max_pos = tmp_pos + 1;
            }
            tmp_pos = usize::from(read_u16(buf, tmp_pos) - (0x3 << 14));
            length = buf[tmp_pos];
            for i in 0 .. length {
                qname.push(char::from(buf[tmp_pos + 1 + usize::from(i)]))
            }
            qname.push('.');
            tmp_pos += usize::from(length) + 1;
        } else {
            if max_pos < tmp_pos {
                max_pos = tmp_pos;
            }

            for i in 0 .. length {
                qname.push(char::from(buf[tmp_pos + 1 + usize::from(i)]))
            }
            qname.push('.');
            tmp_pos += usize::from(length) + 1;
        }
    }

    if max_pos < tmp_pos {
        max_pos = tmp_pos;
    }
    return max_pos + 1;
}

fn read_question(buf: &Vec<u8>, pos: usize) -> (Question, usize) {
    let mut qname = String::new();
    let current_pos = read_qname(buf, pos, &mut qname);

    let q = Question{
        qname: qname,
        qtype: read_u16(&buf, current_pos),
        qclass: read_u16(&buf, current_pos+ 2),
    };

    return (q, pos + 4);
}

pub fn get_message(buf: &Vec<u8>) -> Message {
    let header = read_header(buf);

    let mut pos = 12;
    let mut questions: Vec<Question> = Vec::new();
    for _ in 0 .. header.qdcount {
        let question: Question;
        (question, pos) = read_question(&buf, pos);
        questions.push(question)
    }

    return Message { 
        hdr: header, 
        questions: questions, 
        answers: Vec::new(), 
        authority_records: Vec::new(), 
        additional_records: Vec::new(),
    };
}