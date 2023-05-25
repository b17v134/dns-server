extern crate rand;

use std::net::UdpSocket;

use rand::Rng;

use std::io::Error;

// https://www.iana.org/assignments/dns-parameters/dns-parameters.xhtml#dns-parameters-4;
pub const DNS_TYPE_ERROR: u16 = 0;
pub const DNS_TYPE_A: u16 = 1;
pub const DNS_TYPE_NS: u16 = 2;
pub const DNS_TYPE_MD: u16 = 3;
pub const DNS_TYPE_MF: u16 = 4;
pub const DNS_TYPE_CNAME: u16 = 5;
pub const DNS_TYPE_SOA: u16 = 6;
pub const DNS_TYPE_MB: u16 = 7;
pub const DNS_TYPE_MG: u16 = 8;
pub const DNS_TYPE_MR: u16 = 9;
pub const DNS_TYPE_NULL: u16 = 10;
pub const DNS_TYPE_WKS: u16 = 11;
pub const DNS_TYPE_PTR: u16 = 12;
pub const DNS_TYPE_HINFO: u16 = 13;
pub const DNS_TYPE_MINFO: u16 = 14;
pub const DNS_TYPE_MX: u16 = 15;
pub const DNS_TYPE_TXT: u16 = 16;
pub const DNS_TYPE_RP: u16 = 17;
pub const DNS_TYPE_AFSDB: u16 = 18;
pub const DNS_TYPE_X25: u16 = 19;
pub const DNS_TYPE_ISDN: u16 = 20;
pub const DNS_TYPE_RT: u16 = 21;
pub const DNS_TYPE_NSAP: u16 = 22;
pub const DNS_TYPE_NSAP_PTR: u16 = 23;
pub const DNS_TYPE_SIG: u16 = 24;
pub const DNS_TYPE_KEY: u16 = 25;
pub const DNS_TYPE_PX: u16 = 26;
pub const DNS_TYPE_GPOS: u16 = 27;
pub const DNS_TYPE_AAAA: u16 = 28;
pub const DNS_TYPE_LOC: u16 = 29;
pub const DNS_TYPE_NXT: u16 = 30;
pub const DNS_TYPE_EID: u16 = 31;
pub const DNS_TYPE_NIMLOC: u16 = 32;
pub const DNS_TYPE_SRV: u16 = 33;
pub const DNS_TYPE_ATMA: u16 = 34;
pub const DNS_TYPE_NAPTR: u16 = 35;
pub const DNS_TYPE_KX: u16 = 36;
pub const DNS_TYPE_CERT: u16 = 37;
pub const DNS_TYPE_A6: u16 = 38;
pub const DNS_TYPE_DNAME: u16 = 39;
pub const DNS_TYPE_SINK: u16 = 40;
pub const DNS_TYPE_OPT: u16 = 41;
pub const DNS_TYPE_APL: u16 = 42;
pub const DNS_TYPE_DS: u16 = 43;
pub const DNS_TYPE_SSHFP: u16 = 44;
pub const DNS_TYPE_IPSECKEY: u16 = 45;
pub const DNS_TYPE_RRSIG: u16 = 46;
pub const DNS_TYPE_NSEC: u16 = 47;
pub const DNS_TYPE_DNSKEY: u16 = 48;
pub const DNS_TYPE_DHCID: u16 = 49;
pub const DNS_TYPE_NSEC3: u16 = 50;
pub const DNS_TYPE_NSEC3PARAM: u16 = 51;
pub const DNS_TYPE_TLSA: u16 = 52;
pub const DNS_TYPE_SMIMEA: u16 = 53;

pub const DNS_TYPE_HIP: u16 = 55;
pub const DNS_TYPE_NINFO: u16 = 56;
pub const DNS_TYPE_RKEY: u16 = 57;
pub const DNS_TYPE_TALINK: u16 = 58;
pub const DNS_TYPE_CDS: u16 = 59;
pub const DNS_TYPE_CDNSKEY: u16 = 60;
pub const DNS_TYPE_OPENPGPKEY: u16 = 61;
pub const DNS_TYPE_CSYNC: u16 = 62;
pub const DNS_TYPE_ZONEMD: u16 = 63;
pub const DNS_TYPE_SVCB: u16 = 64;
pub const DNS_TYPE_HTTPS: u16 = 65;

pub const DNS_TYPE_SPF: u16 = 99;
pub const DNS_TYPE_UINFO: u16 = 100;
pub const DNS_TYPE_UID: u16 = 101;
pub const DNS_TYPE_GID: u16 = 102;
pub const DNS_TYPE_UNSPEC: u16 = 103;
pub const DNS_TYPE_NID: u16 = 104;
pub const DNS_TYPE_L32: u16 = 105;
pub const DNS_TYPE_L64: u16 = 106;
pub const DNS_TYPE_LP: u16 = 107;
pub const DNS_TYPE_EUI48: u16 = 108;
pub const DNS_TYPE_EUI64: u16 = 109;

pub const DNS_TYPE_TKEY: u16 = 249;
pub const DNS_TYPE_TSIG: u16 = 250;
pub const DNS_TYPE_IXFR: u16 = 251;
pub const DNS_TYPE_AXFR: u16 = 252;
pub const DNS_TYPE_MAILB: u16 = 253;
pub const DNS_TYPE_MAILA: u16 = 254;

pub const DNS_TYPE_ANY: u16 = 255;

pub const DNS_TYPE_URI: u16 = 256;
pub const DNS_TYPE_CAA: u16 = 257;
pub const DNS_TYPE_AVC: u16 = 258;
pub const DNS_TYPE_DOA: u16 = 259;
pub const DNS_TYPE_AMTREPLAY: u16 = 260;

pub const DNS_TYPE_TA: u16 = 32768;
pub const DNS_TYPE_DLV: u16 = 32769;

pub const DNS_STR_TYPE_A: &str = "A";
pub const DNS_STR_TYPE_NS: &str = "NS";
pub const DNS_STR_TYPE_MD: &str = "MD";
pub const DNS_STR_TYPE_MF: &str = "MF";
pub const DNS_STR_TYPE_CNAME: &str = "CNAME";
pub const DNS_STR_TYPE_SOA: &str = "SOA";
pub const DNS_STR_TYPE_MB: &str = "MB";
pub const DNS_STR_TYPE_MG: &str = "MG";
pub const DNS_STR_TYPE_MR: &str = "MR";
pub const DNS_STR_TYPE_NULL: &str = "NULL";
pub const DNS_STR_TYPE_WKS: &str = "WKS";
pub const DNS_STR_TYPE_PTR: &str = "PTR";
pub const DNS_STR_TYPE_HINFO: &str = "HINFO";
pub const DNS_STR_TYPE_MINFO: &str = "MINFO";
pub const DNS_STR_TYPE_MX: &str = "MX";
pub const DNS_STR_TYPE_TXT: &str = "TXT";
pub const DNS_STR_TYPE_RP: &str = "RP";
pub const DNS_STR_TYPE_AFSDB: &str = "AFSDB";
pub const DNS_STR_TYPE_X25: &str = "X25";
pub const DNS_STR_TYPE_ISDN: &str = "ISDN";
pub const DNS_STR_TYPE_RT: &str = "RT";
pub const DNS_STR_TYPE_NSAP: &str = "NSAP";
pub const DNS_STR_TYPE_NSAP_PTR: &str = "NSAP-PTR";
pub const DNS_STR_TYPE_SIG: &str = "SIG";
pub const DNS_STR_TYPE_KEY: &str = "KEY";
pub const DNS_STR_TYPE_PX: &str = "PX";
pub const DNS_STR_TYPE_GPOS: &str = "GPOS";
pub const DNS_STR_TYPE_AAAA: &str = "AAAA";
pub const DNS_STR_TYPE_LOC: &str = "LOC";
pub const DNS_STR_TYPE_NXT: &str = "NXT";
pub const DNS_STR_TYPE_EID: &str = "EID";
pub const DNS_STR_TYPE_NIMLOC: &str = "NIMLOC";
pub const DNS_STR_TYPE_SRV: &str = "SRV";
pub const DNS_STR_TYPE_ATMA: &str = "ATMA";
pub const DNS_STR_TYPE_NAPTR: &str = "NAPTR";
pub const DNS_STR_TYPE_KX: &str = "KX";
pub const DNS_STR_TYPE_CERT: &str = "CERT";
pub const DNS_STR_TYPE_A6: &str = "A6";
pub const DNS_STR_TYPE_DNAME: &str = "DNAME";
pub const DNS_STR_TYPE_SINK: &str = "SINK";
pub const DNS_STR_TYPE_OPT: &str = "OPT";
pub const DNS_STR_TYPE_APL: &str = "APL";
pub const DNS_STR_TYPE_DS: &str = "DS";
pub const DNS_STR_TYPE_SSHFP: &str = "SSHFP";
pub const DNS_STR_TYPE_IPSECKEY: &str = "IPSECKEY";
pub const DNS_STR_TYPE_RRSIG: &str = "RRSIG";
pub const DNS_STR_TYPE_NSEC: &str = "NSEC";
pub const DNS_STR_TYPE_DNSKEY: &str = "DNSKEY";
pub const DNS_STR_TYPE_DHCID: &str = "DHCID";
pub const DNS_STR_TYPE_NSEC3: &str = "NSEC3";
pub const DNS_STR_TYPE_NSEC3PARAM: &str = "NSEC3PARAM";
pub const DNS_STR_TYPE_TLSA: &str = "TLSA";
pub const DNS_STR_TYPE_SMIMEA: &str = "SMIMEA";
pub const DNS_STR_TYPE_HIP: &str = "HIP";
pub const DNS_STR_TYPE_NINFO: &str = "NINFO";
pub const DNS_STR_TYPE_RKEY: &str = "RKEY";
pub const DNS_STR_TYPE_TALINK: &str = "TALINK";
pub const DNS_STR_TYPE_CDS: &str = "CDS";
pub const DNS_STR_TYPE_CDNSKEY: &str = "CDNSKEY";
pub const DNS_STR_TYPE_OPENPGPKEY: &str = "OPENPGPKEY";
pub const DNS_STR_TYPE_CSYNC: &str = "CSYNC";
pub const DNS_STR_TYPE_ZONEMD: &str = "ZONEMD";
pub const DNS_STR_TYPE_SVCB: &str = "SVCB";
pub const DNS_STR_TYPE_HTTPS: &str = "HTTPS";
pub const DNS_STR_TYPE_SPF: &str = "SPF";
pub const DNS_STR_TYPE_UINFO: &str = "UINFO";
pub const DNS_STR_TYPE_UID: &str = "UID";
pub const DNS_STR_TYPE_GID: &str = "GID";
pub const DNS_STR_TYPE_UNSPEC: &str = "UNSPEC";
pub const DNS_STR_TYPE_NID: &str = "NID";
pub const DNS_STR_TYPE_L32: &str = "L32";
pub const DNS_STR_TYPE_L64: &str = "L64";
pub const DNS_STR_TYPE_LP: &str = "LP";
pub const DNS_STR_TYPE_EUI48: &str = "EUI48";
pub const DNS_STR_TYPE_EUI64: &str = "EUI64";
pub const DNS_STR_TYPE_TKEY: &str = "TKEY";
pub const DNS_STR_TYPE_TSIG: &str = "TSIG";
pub const DNS_STR_TYPE_IXFR: &str = "IXFR";
pub const DNS_STR_TYPE_AXFR: &str = "AXFR";
pub const DNS_STR_TYPE_MAILB: &str = "MAILB";
pub const DNS_STR_TYPE_MAILA: &str = "MAILA";

pub const DNS_STR_TYPE_ANY: &str = "ANY";

pub const DNS_STR_TYPE_URI: &str = "URI";
pub const DNS_STR_TYPE_CAA: &str = "CAA";
pub const DNS_STR_TYPE_AVC: &str = "AVC";
pub const DNS_STR_TYPE_DOA: &str = "DOA";
pub const DNS_STR_TYPE_AMTREPLAY: &str = "AMTREPLAY";
pub const DNS_STR_TYPE_TA: &str = "TA";
pub const DNS_STR_TYPE_DLV: &str = "DLV";

pub fn dns_type_to_u16(v: &str) -> u16 {
    match v {
        DNS_STR_TYPE_A => DNS_TYPE_A,
        DNS_STR_TYPE_NS => DNS_TYPE_NS,
        DNS_STR_TYPE_MD => DNS_TYPE_MD,
        DNS_STR_TYPE_MF => DNS_TYPE_MF,
        DNS_STR_TYPE_CNAME => DNS_TYPE_CNAME,
        DNS_STR_TYPE_SOA => DNS_TYPE_SOA,
        DNS_STR_TYPE_MB => DNS_TYPE_MB,
        DNS_STR_TYPE_MG => DNS_TYPE_MG,
        DNS_STR_TYPE_MR => DNS_TYPE_MR,
        DNS_STR_TYPE_NULL => DNS_TYPE_NULL,
        DNS_STR_TYPE_WKS => DNS_TYPE_WKS,
        DNS_STR_TYPE_PTR => DNS_TYPE_PTR,
        DNS_STR_TYPE_HINFO => DNS_TYPE_HINFO,
        DNS_STR_TYPE_MINFO => DNS_TYPE_MINFO,
        DNS_STR_TYPE_MX => DNS_TYPE_MX,
        DNS_STR_TYPE_TXT => DNS_TYPE_TXT,
        DNS_STR_TYPE_RP => DNS_TYPE_RP,
        DNS_STR_TYPE_AFSDB => DNS_TYPE_AFSDB,
        DNS_STR_TYPE_X25 => DNS_TYPE_X25,
        DNS_STR_TYPE_ISDN => DNS_TYPE_ISDN,
        DNS_STR_TYPE_RT => DNS_TYPE_RT,
        DNS_STR_TYPE_NSAP => DNS_TYPE_NSAP,
        DNS_STR_TYPE_NSAP_PTR => DNS_TYPE_NSAP_PTR,
        DNS_STR_TYPE_SIG => DNS_TYPE_SIG,
        DNS_STR_TYPE_KEY => DNS_TYPE_KEY,
        DNS_STR_TYPE_PX => DNS_TYPE_PX,
        DNS_STR_TYPE_GPOS => DNS_TYPE_GPOS,
        DNS_STR_TYPE_AAAA => DNS_TYPE_AAAA,
        DNS_STR_TYPE_LOC => DNS_TYPE_LOC,
        DNS_STR_TYPE_NXT => DNS_TYPE_NXT,
        DNS_STR_TYPE_EID => DNS_TYPE_EID,
        DNS_STR_TYPE_NIMLOC => DNS_TYPE_NIMLOC,
        DNS_STR_TYPE_SRV => DNS_TYPE_SRV,
        DNS_STR_TYPE_ATMA => DNS_TYPE_ATMA,
        DNS_STR_TYPE_NAPTR => DNS_TYPE_NAPTR,
        DNS_STR_TYPE_KX => DNS_TYPE_KX,
        DNS_STR_TYPE_CERT => DNS_TYPE_CERT,
        DNS_STR_TYPE_A6 => DNS_TYPE_A6,
        DNS_STR_TYPE_DNAME => DNS_TYPE_DNAME,
        DNS_STR_TYPE_SINK => DNS_TYPE_SINK,
        DNS_STR_TYPE_OPT => DNS_TYPE_OPT,
        DNS_STR_TYPE_APL => DNS_TYPE_APL,
        DNS_STR_TYPE_DS => DNS_TYPE_DS,
        DNS_STR_TYPE_SSHFP => DNS_TYPE_SSHFP,
        DNS_STR_TYPE_IPSECKEY => DNS_TYPE_IPSECKEY,
        DNS_STR_TYPE_RRSIG => DNS_TYPE_RRSIG,
        DNS_STR_TYPE_NSEC => DNS_TYPE_NSEC,
        DNS_STR_TYPE_DNSKEY => DNS_TYPE_DNSKEY,
        DNS_STR_TYPE_DHCID => DNS_TYPE_DHCID,
        DNS_STR_TYPE_NSEC3 => DNS_TYPE_NSEC3,
        DNS_STR_TYPE_NSEC3PARAM => DNS_TYPE_NSEC3PARAM,
        DNS_STR_TYPE_TLSA => DNS_TYPE_TLSA,
        DNS_STR_TYPE_SMIMEA => DNS_TYPE_SMIMEA,
        DNS_STR_TYPE_HIP => DNS_TYPE_HIP,
        DNS_STR_TYPE_NINFO => DNS_TYPE_NINFO,
        DNS_STR_TYPE_RKEY => DNS_TYPE_RKEY,
        DNS_STR_TYPE_TALINK => DNS_TYPE_TALINK,
        DNS_STR_TYPE_CDS => DNS_TYPE_CDS,
        DNS_STR_TYPE_CDNSKEY => DNS_TYPE_CDNSKEY,
        DNS_STR_TYPE_OPENPGPKEY => DNS_TYPE_OPENPGPKEY,
        DNS_STR_TYPE_CSYNC => DNS_TYPE_CSYNC,
        DNS_STR_TYPE_ZONEMD => DNS_TYPE_ZONEMD,
        DNS_STR_TYPE_SVCB => DNS_TYPE_SVCB,
        DNS_STR_TYPE_HTTPS => DNS_TYPE_HTTPS,
        DNS_STR_TYPE_SPF => DNS_TYPE_SPF,
        DNS_STR_TYPE_UINFO => DNS_TYPE_UINFO,
        DNS_STR_TYPE_UID => DNS_TYPE_UID,
        DNS_STR_TYPE_GID => DNS_TYPE_GID,
        DNS_STR_TYPE_UNSPEC => DNS_TYPE_UNSPEC,
        DNS_STR_TYPE_NID => DNS_TYPE_NID,
        DNS_STR_TYPE_L32 => DNS_TYPE_L32,
        DNS_STR_TYPE_L64 => DNS_TYPE_L64,
        DNS_STR_TYPE_LP => DNS_TYPE_LP,
        DNS_STR_TYPE_EUI48 => DNS_TYPE_EUI48,
        DNS_STR_TYPE_EUI64 => DNS_TYPE_EUI64,
        DNS_STR_TYPE_TKEY => DNS_TYPE_TKEY,
        DNS_STR_TYPE_TSIG => DNS_TYPE_TSIG,
        DNS_STR_TYPE_IXFR => DNS_TYPE_IXFR,
        DNS_STR_TYPE_AXFR => DNS_TYPE_AXFR,
        DNS_STR_TYPE_MAILB => DNS_TYPE_MAILB,
        DNS_STR_TYPE_MAILA => DNS_TYPE_MAILA,
    
        DNS_STR_TYPE_ANY => DNS_TYPE_ANY,
    
        DNS_STR_TYPE_URI => DNS_TYPE_URI,
        DNS_STR_TYPE_CAA => DNS_TYPE_CAA,
        DNS_STR_TYPE_AVC => DNS_TYPE_AVC,
        DNS_STR_TYPE_DOA => DNS_TYPE_DOA,
        DNS_STR_TYPE_AMTREPLAY => DNS_TYPE_AMTREPLAY,
        DNS_STR_TYPE_TA => DNS_TYPE_TA,
        DNS_STR_TYPE_DLV => DNS_TYPE_DLV, 
        _ => DNS_TYPE_ERROR,
    }
}

pub fn u16_to_dns_type(v: u16) -> &'static str {
    match v {
        DNS_TYPE_A => DNS_STR_TYPE_A,
        DNS_TYPE_NS => DNS_STR_TYPE_NS,
        DNS_TYPE_MD => DNS_STR_TYPE_MD,
        DNS_TYPE_MF => DNS_STR_TYPE_MF,
        DNS_TYPE_CNAME => DNS_STR_TYPE_CNAME,
        DNS_TYPE_SOA => DNS_STR_TYPE_SOA,
        DNS_TYPE_MB => DNS_STR_TYPE_MB,
        DNS_TYPE_MG => DNS_STR_TYPE_MG,
        DNS_TYPE_MR => DNS_STR_TYPE_MR,
        DNS_TYPE_NULL => DNS_STR_TYPE_NULL,
        DNS_TYPE_WKS => DNS_STR_TYPE_WKS,
        DNS_TYPE_PTR => DNS_STR_TYPE_PTR,
        DNS_TYPE_HINFO => DNS_STR_TYPE_HINFO,
        DNS_TYPE_MINFO => DNS_STR_TYPE_MINFO,
        DNS_TYPE_MX => DNS_STR_TYPE_MX,
        DNS_TYPE_TXT => DNS_STR_TYPE_TXT,
        DNS_TYPE_RP => DNS_STR_TYPE_RP,
        DNS_TYPE_AFSDB => DNS_STR_TYPE_AFSDB,
        DNS_TYPE_X25 => DNS_STR_TYPE_X25,
        DNS_TYPE_ISDN => DNS_STR_TYPE_ISDN,
        DNS_TYPE_RT => DNS_STR_TYPE_RT,
        DNS_TYPE_NSAP => DNS_STR_TYPE_NSAP,
        DNS_TYPE_NSAP_PTR => DNS_STR_TYPE_NSAP_PTR,
        DNS_TYPE_SIG => DNS_STR_TYPE_SIG,
        DNS_TYPE_KEY => DNS_STR_TYPE_KEY,
        DNS_TYPE_PX => DNS_STR_TYPE_PX,
        DNS_TYPE_GPOS => DNS_STR_TYPE_GPOS,
        DNS_TYPE_AAAA => DNS_STR_TYPE_AAAA,
        DNS_TYPE_LOC => DNS_STR_TYPE_LOC,
        DNS_TYPE_NXT => DNS_STR_TYPE_NXT,
        DNS_TYPE_EID => DNS_STR_TYPE_EID,
        DNS_TYPE_NIMLOC => DNS_STR_TYPE_NIMLOC,
        DNS_TYPE_SRV => DNS_STR_TYPE_SRV,
        DNS_TYPE_ATMA => DNS_STR_TYPE_ATMA,
        DNS_TYPE_NAPTR => DNS_STR_TYPE_NAPTR,
        DNS_TYPE_KX => DNS_STR_TYPE_KX,
        DNS_TYPE_CERT => DNS_STR_TYPE_CERT,
        DNS_TYPE_A6 => DNS_STR_TYPE_A6,
        DNS_TYPE_DNAME => DNS_STR_TYPE_DNAME,
        DNS_TYPE_SINK => DNS_STR_TYPE_SINK,
        DNS_TYPE_OPT => DNS_STR_TYPE_OPT,
        DNS_TYPE_APL => DNS_STR_TYPE_APL,
        DNS_TYPE_DS => DNS_STR_TYPE_DS,
        DNS_TYPE_SSHFP => DNS_STR_TYPE_SSHFP,
        DNS_TYPE_IPSECKEY => DNS_STR_TYPE_IPSECKEY,
        DNS_TYPE_RRSIG => DNS_STR_TYPE_RRSIG,
        DNS_TYPE_NSEC => DNS_STR_TYPE_NSEC,
        DNS_TYPE_DNSKEY => DNS_STR_TYPE_DNSKEY,
        DNS_TYPE_DHCID => DNS_STR_TYPE_DHCID,
        DNS_TYPE_NSEC3 => DNS_STR_TYPE_NSEC3,
        DNS_TYPE_NSEC3PARAM => DNS_STR_TYPE_NSEC3PARAM,
        DNS_TYPE_TLSA => DNS_STR_TYPE_TLSA,
        DNS_TYPE_SMIMEA => DNS_STR_TYPE_SMIMEA,
        DNS_TYPE_HIP => DNS_STR_TYPE_HIP,
        DNS_TYPE_NINFO => DNS_STR_TYPE_NINFO,
        DNS_TYPE_RKEY => DNS_STR_TYPE_RKEY,
        DNS_TYPE_TALINK => DNS_STR_TYPE_TALINK,
        DNS_TYPE_CDS => DNS_STR_TYPE_CDS,
        DNS_TYPE_CDNSKEY => DNS_STR_TYPE_CDNSKEY,
        DNS_TYPE_OPENPGPKEY => DNS_STR_TYPE_OPENPGPKEY,
        DNS_TYPE_CSYNC => DNS_STR_TYPE_CSYNC,
        DNS_TYPE_ZONEMD => DNS_STR_TYPE_ZONEMD,
        DNS_TYPE_SVCB => DNS_STR_TYPE_SVCB,
        DNS_TYPE_HTTPS => DNS_STR_TYPE_HTTPS,
        DNS_TYPE_SPF => DNS_STR_TYPE_SPF,
        DNS_TYPE_UINFO => DNS_STR_TYPE_UINFO,
        DNS_TYPE_UID => DNS_STR_TYPE_UID,
        DNS_TYPE_GID => DNS_STR_TYPE_GID,
        DNS_TYPE_UNSPEC => DNS_STR_TYPE_UNSPEC,
        DNS_TYPE_NID => DNS_STR_TYPE_NID,
        DNS_TYPE_L32 => DNS_STR_TYPE_L32,
        DNS_TYPE_L64 => DNS_STR_TYPE_L64,
        DNS_TYPE_LP => DNS_STR_TYPE_LP,
        DNS_TYPE_EUI48 => DNS_STR_TYPE_EUI48,
        DNS_TYPE_EUI64 => DNS_STR_TYPE_EUI64,
        DNS_TYPE_TKEY => DNS_STR_TYPE_TKEY,
        DNS_TYPE_TSIG => DNS_STR_TYPE_TSIG,
        DNS_TYPE_IXFR => DNS_STR_TYPE_IXFR,
        DNS_TYPE_AXFR => DNS_STR_TYPE_AXFR,
        DNS_TYPE_MAILB => DNS_STR_TYPE_MAILB,
        DNS_TYPE_MAILA => DNS_STR_TYPE_MAILA,

        DNS_TYPE_ANY => DNS_STR_TYPE_ANY,

        DNS_TYPE_URI => DNS_STR_TYPE_URI,
        DNS_TYPE_CAA => DNS_STR_TYPE_CAA,
        DNS_TYPE_AVC => DNS_STR_TYPE_AVC,
        DNS_TYPE_DOA => DNS_STR_TYPE_DOA,
        DNS_TYPE_AMTREPLAY => DNS_STR_TYPE_AMTREPLAY,
        DNS_TYPE_TA => DNS_STR_TYPE_TA,
        DNS_TYPE_DLV => DNS_STR_TYPE_DLV,
        _ => "ERROR",
    }
}

pub const DNS_CLASS_ERROR: u16 = 0;
pub const DNS_CLASS_IN: u16 = 1;
pub const DNS_CLASS_CH: u16 = 3;
pub const DNS_CLASS_HS: u16 = 4;

pub const DNS_STR_CLASS_IN: &str = "IN";
pub const DNS_STR_CLASS_CH: &str = "CH";
pub const DNS_STR_CLASS_HS: &str = "HS";

pub fn dns_class_to_u16(v: &str) -> u16 {
    match v {
        DNS_STR_CLASS_IN => DNS_CLASS_IN,
        DNS_STR_CLASS_CH => DNS_CLASS_CH,
        DNS_STR_CLASS_HS => DNS_CLASS_HS,
        _ => DNS_CLASS_ERROR,
    }
}

pub fn u16_to_dns_class(v: u16) -> &'static str {
    match v {
        DNS_CLASS_IN => DNS_STR_CLASS_IN,
        DNS_CLASS_CH => DNS_STR_CLASS_CH,
        DNS_CLASS_HS => DNS_STR_CLASS_HS,
        _ => "ERROR",
    }
}

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
    pub rdata: String,
}

pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<ResourceRecord>,
    pub authority_records: Vec<ResourceRecord>,
    pub additional_records: Vec<ResourceRecord>
}

pub enum Protocol {
    Https,
    Tcp,
    Tls,
    Udp,
 }

/*fn read_u8(buf: &Vec<u8>, pos: usize) -> u8 {
    return buf[pos];
}*/

fn read_u16(buf: &[u8], pos: usize) -> u16 {
    u16::from(buf[pos]) * 0x100 + u16::from(buf[pos+ 1])
}

fn read_u32(buf: &[u8], pos: usize) -> u32 {
        u32::from(buf[pos]) * 0x1000000 + 
        u32::from(buf[pos + 1]) * 0x10000 +
        u32::from(buf[pos + 2]) *0x100 + 
        u32::from(buf[pos + 3])
}

fn read_header(buf: &[u8]) -> Header {
    let flags = read_u16(buf, 2);

    Header{
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
    
    max_pos + 1
}

fn read_question(buf: &[u8], pos: usize) -> (Question, usize) {
    let mut qname = String::new();
    let current_pos = read_qname(buf, pos, &mut qname);

    let q = Question{
        qname,
        qtype: read_u16(buf, current_pos),
        qclass: read_u16(buf, current_pos+ 2),
    };

    (q, pos + 4)
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


fn read_resource_record(buf: &[u8], pos: usize) -> (ResourceRecord, usize) {
    let mut qname = String::new();
    let mut tmp_current_pos = read_qname(buf, pos, &mut qname);

    let resource_record_type = read_u16(buf, tmp_current_pos + 1);
    println!("tmp_current_pos = {}", tmp_current_pos);
    println!("type = {}", resource_record_type);
    println!("qname = '{}'", qname);
    let mut rdata: String = String::from("");
    match resource_record_type {
        DNS_TYPE_A => { rdata = read_ipv4(buf, tmp_current_pos + 11);},
        /*DNS_TYPE_AAAA => { rdata = read_ipv6(buf, tmp_current_pos + 11);},*/
        _ =>  { read_qname(buf, tmp_current_pos + 11, &mut rdata); },
    }

    let resource_record = ResourceRecord{ 
        name: qname, 
        _type: resource_record_type, 
        class: read_u16(buf, tmp_current_pos + 3), 
        ttl: read_u32(buf, tmp_current_pos + 5), 
        rdlength: read_u16(buf, tmp_current_pos + 9), 
        rdata, 
    };

    let length = resource_record.rdlength;
    println!("length = {}", length);
    tmp_current_pos += 11 + usize::from(resource_record.rdlength);
    (resource_record, tmp_current_pos)
}

pub fn get_message(buf: &[u8]) -> Message {
    let header = read_header(buf);

    let mut pos = 12;

    let mut questions: Vec<Question> = Vec::new();
    for _ in 0 .. header.qdcount {
        let question: Question;
        (question, pos) = read_question(buf, pos);
        questions.push(question)
    }

    let mut answers: Vec<ResourceRecord> = Vec::new();
    for _ in 0 .. header.ancount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(buf, pos);
        answers.push(resource_record)
    }

    let mut authority_records: Vec<ResourceRecord> = Vec::new();
    for _ in 0 .. header.nscount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(buf, pos);
        authority_records.push(resource_record)
    }

    let mut additional_records: Vec<ResourceRecord> = Vec::new();
    for _ in 0 .. header.arcount {
        let resource_record: ResourceRecord;
        (resource_record, pos) = read_resource_record(buf, pos);
        additional_records.push(resource_record)
    }

    Message { 
        header, 
        questions, 
        answers, 
        authority_records, 
        additional_records,
    }
}

pub struct Request {
    pub server: String,
    pub port: u16,
    pub protocol: Protocol,
    pub qname: String,
    pub type_: u16,
    pub class: String 
}

fn create_header() -> Header {
    let mut rnd = rand::thread_rng();
    Header{
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

fn write_qname(buf: &mut [u8], qname: String) -> usize {
    let s = qname.as_bytes();
    let mut length = s.len();
    if char::from(s[length - 1]) == '.' {
        length -=1;
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
    return pos;
}

fn write_question(buf: &mut [u8], question: Question) ->usize {
    let length = write_qname(buf, question.qname);
    write_u16(&mut buf[length + 1 ..], question.qtype);
    write_u16(&mut buf[length + 3 ..], question.qclass);

    length + 5
}

fn create_request_buf(buf: &mut [u8], question: Question) -> usize {
    let header = create_header();
    let header_length = write_header(buf, &header);
    let length = write_question(&mut buf[header_length ..], question);

    return header_length +  length;
}

pub fn resolv(request: Request) -> Result<Message, Error> {
    let question = Question{
        qname: request.qname, 
        qtype: request.type_,
        qclass: dns_class_to_u16(request.class.as_str()),
    };
    let mut buf = [0; 2048];
    let length =  create_request_buf(buf.as_mut_slice(), question);

    let result = UdpSocket::bind("[::]:0");
    let socket: UdpSocket;
    match result {
        Ok(s) => {
            socket = s;
        },
        Err(e) => {
            return Err(e)
        }
    }

    let send_buf: &[u8] = &buf[0 .. length];
    socket.send_to(send_buf, request.server + ":" + request.port.to_string().as_str());
    let mut buf2 = [0; 2048];
	let (amt, _src) = socket.recv_from(&mut buf2)?;

	let message = get_message(&buf2);
    Ok(message)
}

fn get_flags(header:&Header) -> u16 {
    (u16::from(header.qr & 1) << 15) + (u16::from(header.rd & 1) << 8)
}

fn write_u16(buf: &mut [u8], value: u16) {
    buf[0] = ((value & 0xFF00) >> 8) as u8;
    buf[1] = (value & 0xFF) as u8;
} 

pub fn print_message(message: Message) {
    {
        println!("header:");
        print_header(message.header);
    
        println!("\nquestions:");
        for question in message.questions.iter() {
            println!(
                "{}\t\t{}\t{}",
                question.qname,
                u16_to_dns_class(question.qclass),
                u16_to_dns_type(question.qtype));
        }
        
        println!("\nanswers:");
        for answer in message.answers.iter() {
            print_resource_record(answer);
        }

        println!("\nauthority records:");
        for authority_record in message.authority_records.iter() {
            print_resource_record(authority_record);
        }

        println!("\nadditional records:");
        for additional_record in message.additional_records.iter() {
            print_resource_record(additional_record);
        }
    }
}

fn print_header(header: Header)
{
    println!(
        "id: {}\nresponse: {}\nopcode: {}\nauthoritative: {}\ntruncated: {}\nrecursion desired: {}\nrecursion available: {}\nreserved: {}\nrcode: {}\nquestion: {}\nanswer: {}\nauthority: {}\nadditional: {}\n",
        header.id,
        header.qr,
        header.opcode,
        header.aa,
        header.tc,
        header.rd,
        header.ra,
        header.z,
        header.rcode,
        header.qdcount,
        header.ancount,
        header.nscount,
        header.arcount);
}

fn print_resource_record(resource_record: &ResourceRecord) {
    println!(
        "{}\t{}\t{}\t{}\t{}",
        resource_record.name,
        resource_record.ttl,
        u16_to_dns_class(resource_record.class),
        u16_to_dns_type(resource_record._type),
        resource_record.rdata,
    );
}