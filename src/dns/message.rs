use super::{
    additional::DnsAdditional,
    answer::DnsAnswer,
    authority::DnsAuthority,
    header::{DnsHeader, DnsHeaderAA, DnsHeaderOpcode, DnsHeaderQR, DnsHeaderRA, DnsHeaderRD, DnsHeaderRcode, DnsHeaderTC, DnsHeaderZ},
    question::DnsQuestion,
};

pub struct DnsMessage {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
    authorities: Vec<DnsAuthority>,
    additional: Vec<DnsAdditional>,
}

impl From<[u8; 512]> for DnsMessage {
    fn from(data: [u8; 512]) -> Self {
        unimplemented!("Not implemented");

        // Self {
        //     header: DnsHeader {
        //         id: u16::from_be_bytes([data[0], data[1]]),
        //         query_response: DnsHeaderQR::Reply,
        //         opcode: DnsHeaderOpcode::Query,
        //         authoritative_answer: DnsHeaderAA::NonAuthoritative,
        //         truncation: DnsHeaderTC::NotTruncated,
        //         recursion_desired: DnsHeaderRD::RecursionDesired,
        //         recursion_available: DnsHeaderRA::RecursionAvailable,
        //         z: DnsHeaderZ::Reserved,
        //         rcode: DnsHeaderRcode::NoError,
        //         question_count: 1,
        //         answer_count: 0,
        //         authority_count: 0,
        //         additional_count: 0,
        //     },
        //     questions: Vec::new(),
        //     answers: Vec::new(),
        //     authorities: Vec::new(),
        //     additional: Vec::new(),
        // }
    }
}
