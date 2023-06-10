pub const DNS_CLASS_ERROR: u16 = 0;
pub const DNS_CLASS_IN: u16 = 1;
pub const DNS_CLASS_CH: u16 = 3;
pub const DNS_CLASS_HS: u16 = 4;

pub const DNS_STR_CLASS_ERROR: &str = "ERROR";
pub const DNS_STR_CLASS_IN: &str = "IN";
pub const DNS_STR_CLASS_CH: &str = "CH";
pub const DNS_STR_CLASS_HS: &str = "HS";

#[must_use]
pub fn dns_class_to_u16(v: &str) -> u16 {
    match v.to_uppercase().as_str() {
        DNS_STR_CLASS_IN => DNS_CLASS_IN,
        DNS_STR_CLASS_CH => DNS_CLASS_CH,
        DNS_STR_CLASS_HS => DNS_CLASS_HS,
        _ => DNS_CLASS_ERROR,
    }
}

#[must_use]
pub fn u16_to_dns_class(v: u16) -> &'static str {
    match v {
        DNS_CLASS_IN => DNS_STR_CLASS_IN,
        DNS_CLASS_CH => DNS_STR_CLASS_CH,
        DNS_CLASS_HS => DNS_STR_CLASS_HS,
        _ => DNS_STR_CLASS_ERROR,
    }
}

#[cfg(test)]
mod tests {
    use crate::{dns_class_to_u16, u16_to_dns_class};

    #[test]
    fn test_dns_class_to_u16() {
        assert_eq!(dns_class_to_u16("ERROR"), 0);
        assert_eq!(dns_class_to_u16("IN"), 1);
        assert_eq!(dns_class_to_u16("CH"), 3);
        assert_eq!(dns_class_to_u16("HS"), 4);
        assert_eq!(dns_class_to_u16("foo"), 0);
    }

    #[test]
    fn test_u16_to_dns_class() {
        assert_eq!(u16_to_dns_class(0), "ERROR");
        assert_eq!(u16_to_dns_class(1), "IN");
        assert_eq!(u16_to_dns_class(3), "CH");
        assert_eq!(u16_to_dns_class(4), "HS");
        assert_eq!(u16_to_dns_class(100), "ERROR");
    }
}
