use serde::{Deserialize, Serialize};
use std::net::Ipv6Addr;

use crate::header::Header;
use crate::outputformat::OutputFormat;
use crate::question::Question;
use crate::resourcerecord::ResourceRecord;
use crate::types::*;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>,
}

impl Message {
    pub fn print(&self, output_format: &OutputFormat) {
        match output_format {
            OutputFormat::Plain => self.print_message(),
            OutputFormat::Json => self.print_json(),
            OutputFormat::Yaml => self.print_yaml(),
        }
    }

    fn print_message(&self) {
        {
            println!("header:");
            println!("{}", self.header);

            println!("\nquestions:");
            for question in &self.questions {
                println!("{question}");
            }

            println!("\nanswers:");
            for answer in &self.answers {
                println!("{answer}");
            }

            println!("\nauthority records:");
            for authority_record in &self.authority_records {
                println!("{authority_record}");
            }

            println!("\nadditional records:");
            for additional_record in &self.additional_records {
                println!("{additional_record}");
            }
        }
    }

    fn print_json(&self) {
        let result = serde_json::to_string_pretty(&self);

        match result {
            Ok(str) => println!("{str}"),
            Err(e) => println!("{e}"),
        };
    }

    fn print_yaml(&self) {
        let result = serde_yaml::to_string(&self);

        match result {
            Ok(str) => println!("{str}"),
            Err(e) => println!("{e}"),
        };
    }
}

#[must_use]
pub fn get_message(buf: &[u8]) -> Message {
    let header = read_header(buf);

    let mut pos = 12;

    let mut questions: Vec<Question> = Vec::new();
    for _ in 0..header.qdcount {
        let question: Question;
        (question, pos) = read_question(buf, pos);
        questions.push(question);
    }

    let mut answers: Vec<ResourceRecord> = Vec::new();
    for _ in 0..header.ancount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(buf, pos);
        answers.push(resource_record);
    }

    let mut authority_records: Vec<ResourceRecord> = Vec::new();
    for _ in 0..header.nscount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(buf, pos);
        authority_records.push(resource_record);
    }

    let mut additional_records: Vec<ResourceRecord> = Vec::new();
    for _ in 0..header.arcount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(buf, pos);
        additional_records.push(resource_record);
    }

    Message {
        header,
        questions,
        answers,
        authority_records,
        additional_records,
    }
}

fn read_u8(buf: &[u8], pos: usize) -> u8 {
    buf[pos]
}

fn read_u16(buf: &[u8], pos: usize) -> u16 {
    u16::from(buf[pos]) * 0x100 + u16::from(buf[pos + 1])
}

fn read_u32(buf: &[u8], pos: usize) -> u32 {
    u32::from(buf[pos]) * 0x100_0000
        + u32::from(buf[pos + 1]) * 0x1_0000
        + u32::from(buf[pos + 2]) * 0x100
        + u32::from(buf[pos + 3])
}

fn read_header(buf: &[u8]) -> Header {
    let flags = read_u16(buf, 2);

    Header {
        id: read_u16(buf, 0),
        qr: ((flags & (1 << 15)) >> 15) as u8,
        opcode: ((flags & (0xF << 11)) >> 11) as u8,
        aa: ((flags & (1 << 10)) >> 10) as u8,
        tc: ((flags & (1 << 9)) >> 9) as u8,
        rd: ((flags & (1 << 8)) >> 8) as u8,
        ra: ((flags & (1 << 7)) >> 7) as u8,
        z: ((flags & (0x7 << 4)) >> 4) as u8,
        rcode: (flags & 0xF) as u8,

        qdcount: read_u16(buf, 4),
        ancount: read_u16(buf, 6),
        nscount: read_u16(buf, 8),
        arcount: read_u16(buf, 10),
    }
}

fn read_string(buf: &[u8], pos: usize, value: &mut String) -> usize {
    let size: usize = read_u8(buf, pos) as usize;
    for i in 1..=size {
        value.push(char::from(buf[pos + i]));
    }
    pos + size + 1
}

fn read_qname(buf: &[u8], pos: usize, qname: &mut String) -> usize {
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
            for i in 0..length {
                qname.push(char::from(buf[tmp_pos + 1 + usize::from(i)]));
            }
            qname.push('.');
            tmp_pos += usize::from(length) + 1;
        } else {
            if max_pos < tmp_pos {
                max_pos = tmp_pos;
            }

            for i in 0..length {
                qname.push(char::from(buf[tmp_pos + 1 + usize::from(i)]));
            }
            qname.push('.');
            tmp_pos += usize::from(length) + 1;
        }
    }

    if max_pos < tmp_pos {
        max_pos = tmp_pos;
    }

    max_pos + 1
}

fn read_question(buf: &[u8], pos: usize) -> (Question, usize) {
    let mut qname = String::new();
    let current_pos = read_qname(buf, pos, &mut qname);

    let q = Question {
        qname,
        qtype: read_u16(buf, current_pos),
        qclass: read_u16(buf, current_pos + 2),
    };

    (q, current_pos + 4)
}

fn read_ipv4(buf: &[u8], pos: usize) -> String {
    format!(
        "{}.{}.{}.{}",
        buf[pos],
        buf[pos + 1],
        buf[pos + 2],
        buf[pos + 3],
    )
}

fn read_ipv6(buf: &[u8], pos: usize) -> String {
    let addr = Ipv6Addr::new(
        read_u16(buf, pos),
        read_u16(buf, pos + 2),
        read_u16(buf, pos + 4),
        read_u16(buf, pos + 6),
        read_u16(buf, pos + 8),
        read_u16(buf, pos + 10),
        read_u16(buf, pos + 12),
        read_u16(buf, pos + 14),
    );

    addr.to_string()
}

fn read_hinfo(buf: &[u8], pos: usize) -> String {
    let mut cpu = String::new();
    let cur_pos = read_string(buf, pos, &mut cpu);

    let mut os = String::new();
    read_string(buf, cur_pos, &mut os);
    format!("\"{cpu}\" \"{os}\"")
}

fn read_mx(buf: &[u8], pos: usize) -> String {
    let preference = read_u16(buf, pos);
    let mut qname = String::new();
    read_qname(buf, pos + 2, &mut qname);

    format!("{preference} {qname}")
}

fn read_soa(buf: &[u8], pos: usize) -> String {
    let mut nameserver = String::new();
    let mut cur_pos = read_qname(buf, pos, &mut nameserver);

    let mut mailbox = String::new();
    cur_pos = read_qname(buf, cur_pos, &mut mailbox);
    format!(
        "{nameserver} {mailbox} {} {} {} {} {}",
        read_u32(buf, cur_pos),
        read_u32(buf, cur_pos + 4),
        read_u32(buf, cur_pos + 8),
        read_u32(buf, cur_pos + 12),
        read_u32(buf, cur_pos + 16),
    )
}

fn read_resource_record(buf: &[u8], pos: usize) -> (ResourceRecord, usize) {
    let mut qname = String::new();
    let mut tmp_current_pos = read_qname(buf, pos, &mut qname);

    let resource_record_type = read_u16(buf, tmp_current_pos + 1);
    let mut rdata: String = String::new();
    match resource_record_type {
        DNS_TYPE_A => {
            rdata = read_ipv4(buf, tmp_current_pos + 11);
        }
        DNS_TYPE_AAAA => {
            rdata = read_ipv6(buf, tmp_current_pos + 11);
        }
        DNS_TYPE_HINFO => {
            rdata = read_hinfo(buf, tmp_current_pos + 11);
        }
        DNS_TYPE_MX => {
            rdata = read_mx(buf, tmp_current_pos + 11);
        }
        DNS_TYPE_SOA => {
            rdata = read_soa(buf, tmp_current_pos + 11);
        }
        _ => {
            read_qname(buf, tmp_current_pos + 11, &mut rdata);
        }
    }

    let resource_record = ResourceRecord {
        name: qname,
        type_: resource_record_type,
        class: read_u16(buf, tmp_current_pos + 3),
        ttl: read_u32(buf, tmp_current_pos + 5),
        rdlength: read_u16(buf, tmp_current_pos + 9),
        rdata,
    };

    tmp_current_pos += 11 + usize::from(resource_record.rdlength);
    (resource_record, tmp_current_pos)
}
