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

#[cfg(test)]
mod tests {
    use crate::{Question, DNS_CLASS_IN, DNS_TYPE_A, DNS_TYPE_AAAA};

    use super::QuestionSerializer;

    #[test]
    fn test_question_display() {
        let qname = "example.test-bind.".to_string();

        let question = Question {
            qname: qname.to_string(),
            qclass: DNS_CLASS_IN,
            qtype: DNS_TYPE_A,
        };

        assert_eq!(question.to_string(), "example.test-bind.\t\tIN\tA");

        let question = Question {
            qname: qname.to_string(),
            qclass: DNS_CLASS_IN,
            qtype: 8888,
        };

        assert_eq!(question.to_string(), "example.test-bind.\t\tIN\tERROR");

        let question = Question {
            qname,
            qclass: 234,
            qtype: DNS_TYPE_AAAA,
        };

        assert_eq!(question.to_string(), "example.test-bind.\t\tERROR\tAAAA");
    }

    #[test]
    fn test_question_to_serialize() {
        let qname = "example.test-bind.".to_string();

        let question = Question {
            qname,
            qclass: DNS_CLASS_IN,
            qtype: DNS_TYPE_A,
        };

        let result = serde_json::to_string_pretty(&question);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "{\n  \"name\": \"example.test-bind.\",\n  \"qtype\": \"A\",\n  \"qclass\": \"IN\"\n}"
        );

        let result = serde_yaml::to_string(&question);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "name: example.test-bind.\nqtype: A\nqclass: IN\n"
        );
    }

    #[test]
    fn test_questionserializer_from() {
        let qname = "example.test-bind.".to_string();

        let question = Question {
            qname: qname.to_string(),
            qtype: DNS_TYPE_A,
            qclass: DNS_CLASS_IN,
        };

        let question_serializer = QuestionSerializer::from(&question);
        assert_eq!(question_serializer.name, question.qname);
        assert_eq!(question_serializer.qtype, "A");
        assert_eq!(question_serializer.qclass, "IN");

        let question = Question {
            qname: qname.to_string(),
            qtype: DNS_TYPE_A,
            qclass: 234,
        };

        let question_serializer = QuestionSerializer::from(&question);
        assert_eq!(question_serializer.name, question.qname);
        assert_eq!(question_serializer.qtype, "A");
        assert_eq!(question_serializer.qclass, "ERROR");

        let question = Question {
            qname,
            qtype: 8888,
            qclass: DNS_CLASS_IN,
        };

        let question_serializer = QuestionSerializer::from(&question);
        assert_eq!(question_serializer.name, question.qname);
        assert_eq!(question_serializer.qtype, "ERROR");
        assert_eq!(question_serializer.qclass, "IN");
    }
}
