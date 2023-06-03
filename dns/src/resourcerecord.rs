use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

use crate::classes::u16_to_dns_class;
use crate::types::u16_to_dns_type;

// https://www.rfc-editor.org/rfc/rfc1035 4.1.3
#[derive(Deserialize)]
pub struct ResourceRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: u16,
    pub class: u16,
    pub ttl: u32,
    #[serde(skip)]
    pub rdlength: u16,
    pub rdata: String,
}

impl fmt::Display for ResourceRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t{}\t{}\t{}\t{}",
            self.name,
            self.ttl,
            u16_to_dns_class(self.class),
            u16_to_dns_type(self.type_),
            self.rdata,
        )
    }
}

impl serde::Serialize for ResourceRecord {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        SerdeResourceRecord::from(self).serialize(s)
    }
}
#[derive(Debug, Serialize)]
pub struct SerdeResourceRecord {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub class: String,
    pub ttl: u32,
    pub rdata: String,
}

impl<'a> From<&'a ResourceRecord> for SerdeResourceRecord {
    fn from(resource_record: &'a ResourceRecord) -> Self {
        Self {
            name: String::from(resource_record.name.as_str()),
            type_: u16_to_dns_type(resource_record.type_).to_string(),
            class: u16_to_dns_class(resource_record.class).to_string(),
            ttl: resource_record.ttl,
            rdata: String::from(resource_record.rdata.as_str()),
        }
    }
}
