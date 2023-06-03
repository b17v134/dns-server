use serde::{Deserialize, Serialize, Serializer};
use std::fmt;

use crate::classes::u16_to_dns_class;
use crate::types::u16_to_dns_type;

// https://www.rfc-editor.org/rfc/rfc1035 4.1.2
#[derive(Deserialize)]
pub struct Question {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\t\t{}\t{}",
            self.qname,
            u16_to_dns_class(self.qclass),
            u16_to_dns_type(self.qtype),
        )
    }
}

impl serde::Serialize for Question {
    fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        QuestionSerializer::from(self).serialize(s)
    }
}

#[derive(Debug, Serialize)]
struct QuestionSerializer {
    pub name: String,
    pub qtype: String,
    pub qclass: String,
}

impl<'a> From<&'a Question> for QuestionSerializer {
    fn from(question: &'a Question) -> Self {
        Self {
            name: String::from(question.qname.as_str()),
            qtype: u16_to_dns_type(question.qtype).to_string(),
            qclass: u16_to_dns_class(question.qclass).to_string(),
        }
    }
}
