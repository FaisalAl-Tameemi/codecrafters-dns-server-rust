use super::{
    additional::DnsAdditional,
    answer::DnsAnswer,
    authority::DnsAuthority,
    header::DnsHeader,
    question::DnsQuestion,
};

pub struct DnsMessage {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsAnswer>,
    pub authorities: Vec<DnsAuthority>,
    pub additional: Vec<DnsAdditional>,
}

impl From<[u8; 512]> for DnsMessage {
    fn from(data: [u8; 512]) -> Self {
        Self {
            header: DnsHeader::from(&data[0..12]),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            additional: Vec::new(),
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
