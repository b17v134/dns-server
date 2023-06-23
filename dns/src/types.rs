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

pub const DNS_STR_TYPE_ERROR: &str = "ERROR";
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

#[must_use]
pub fn dns_type_to_u16(v: &str) -> u16 {
    match v.to_uppercase().as_str() {
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

#[must_use]
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
        _ => DNS_STR_TYPE_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        dns_type_to_u16, u16_to_dns_type, DNS_STR_TYPE_A, DNS_STR_TYPE_AAAA, DNS_STR_TYPE_ERROR,
        DNS_STR_TYPE_MX, DNS_TYPE_A, DNS_TYPE_AAAA, DNS_TYPE_ERROR, DNS_TYPE_MX,
    };

    #[test]
    fn test_dns_type_to_u16() {
        assert_eq!(dns_type_to_u16(DNS_STR_TYPE_ERROR), DNS_TYPE_ERROR);
        assert_eq!(dns_type_to_u16(DNS_STR_TYPE_A), DNS_TYPE_A);
        assert_eq!(dns_type_to_u16(DNS_STR_TYPE_AAAA), DNS_TYPE_AAAA);
        assert_eq!(dns_type_to_u16("foo"), DNS_TYPE_ERROR);
    }

    #[test]
    fn test_u16_to_dns_type() {
        assert_eq!(u16_to_dns_type(DNS_TYPE_ERROR), DNS_STR_TYPE_ERROR);
        assert_eq!(u16_to_dns_type(DNS_TYPE_MX), DNS_STR_TYPE_MX);
        assert_eq!(u16_to_dns_type(DNS_TYPE_AAAA), DNS_STR_TYPE_AAAA);
        assert_eq!(u16_to_dns_type(34324), DNS_STR_TYPE_ERROR);
    }
}
