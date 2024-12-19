use bytes::BytesMut;

use super::{
    additional::DnsAdditional, answer::DnsAnswer, authority::DnsAuthority, header::*,
    question::DnsQuestion,
};

#[derive(Debug)]
pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsAnswer>,
    pub authorities: Vec<DnsAuthority>,
    pub additional: Vec<DnsAdditional>,
}

impl DnsMessage {
    pub fn new(
        header: DnsHeader,
        questions: Vec<DnsQuestion>,
        answers: Vec<DnsAnswer>,
        authorities: Vec<DnsAuthority>,
        additional: Vec<DnsAdditional>,
    ) -> Self {
        Self {
            header,
            questions,
            answers,
            authorities,
            additional,
        }
    }

    pub fn new_from_received_message(received_message: &DnsMessage) -> Self {
        let answers: Vec<DnsAnswer> = received_message
            .questions
            .iter()
            .map(|question| {
                DnsAnswer::new(
                    &question.name.name,
                    question.qtype,
                    question.qclass,
                    60,
                    vec![8, 8, 8, 8],
                )
            })
            .collect();

        let header = DnsHeader {
            id: received_message.header.id,
            query_response: DnsHeaderQR::Reply,
            opcode: received_message.header.opcode,
            authoritative_answer: DnsHeaderAA::NonAuthoritative,
            truncation: DnsHeaderTC::NotTruncated,
            recursion_desired: received_message.header.recursion_desired,
            recursion_available: DnsHeaderRA::RecursionAvailable,
            z: DnsHeaderZ::Reserved,
            rcode: match received_message.header.opcode {
                DnsHeaderOpcode::Query => DnsHeaderRcode::NoError,
                _ => DnsHeaderRcode::NotImplemented,
            },
            question_count: received_message.questions.len() as u16,
            answer_count: answers.len() as u16,
            authority_count: 0,
            additional_count: 0,
        };
        
        Self::new(
            header,
            received_message.questions.clone(),
            answers,
            vec![],
            vec![],
        )
    }

    pub fn response(&self, received_message: &DnsMessage) -> BytesMut {
        let mut response: BytesMut = BytesMut::with_capacity(512);

        let header_buf = self.header.as_buf();
        response.extend(header_buf);

        received_message.questions.iter().for_each(|question| {
            response.extend(question.as_buf());
        });

        self.answers.iter().for_each(|answer| {
            response.extend(answer.as_buf());
        });

        response
    }

    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        buf.extend(self.header.as_buf());
        self.questions.iter().for_each(|question| {
            buf.extend(question.as_buf());
        });
        self.answers.iter().for_each(|answer| {
            buf.extend(answer.as_buf());
        });
        buf
    }

    pub fn merge(dns_messages: Vec<DnsMessage>) -> DnsMessage {
        let mut dns_header = dns_messages[0].header.clone();
        dns_header.question_count = dns_messages.len() as u16;
        dns_header.answer_count = dns_messages.len() as u16;
        let mut dns_questions: Vec<DnsQuestion> = vec![];
        let mut dns_answers: Vec<DnsAnswer> = vec![];
        dns_messages.into_iter().for_each(|dns_message| {
            dns_questions.extend(dns_message.questions);
            dns_answers.extend(dns_message.answers);
        });
        return DnsMessage {
            header: dns_header,
            questions: dns_questions,
            answers: dns_answers,
            authorities: vec![],
            additional: vec![],
        };
    }
}

impl From<[u8; 512]> for DnsMessage {
    fn from(data: [u8; 512]) -> Self {
        let header = DnsHeader::from(&data[0..12]);
        let mut next_section_skip: usize = 12;
        
        let mut questions = vec![];
        let mut answers = vec![];
        let authorities = vec![];
        let additional = vec![];

        for _ in 0..header.question_count {
            let question = DnsQuestion::from_buf(&data, next_section_skip);
            next_section_skip += question.length;
            questions.push(question);
        }

        for _ in 0..header.answer_count {
            let answer = DnsAnswer::from(&data[next_section_skip..]);
            answers.push(answer);
        }

        Self {
            header,
            questions,
            answers,
            authorities,
            additional,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dns::header::*;

    #[test]
    fn test_dns_message_from_bytes() {
        let header = DnsHeader {
            id: 1234,
            query_response: DnsHeaderQR::Reply,
            opcode: DnsHeaderOpcode::Query,
            authoritative_answer: DnsHeaderAA::Authoritative,
            truncation: DnsHeaderTC::NotTruncated,
            recursion_desired: DnsHeaderRD::RecursionNotDesired,
            recursion_available: DnsHeaderRA::RecursionAvailable,
            z: DnsHeaderZ::Reserved,
            rcode: DnsHeaderRcode::Refused,
            question_count: 1,
            answer_count: 1,
            authority_count: 0,
            additional_count: 0,
        };

        let header_buf = header.as_buf();
        let header_parsed = DnsHeader::from(&header_buf[0..12]);

        debug_assert_eq!(header, header_parsed);
    }
}
