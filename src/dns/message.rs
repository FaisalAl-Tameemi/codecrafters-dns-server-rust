use super::{
    additional::DnsAdditional,
    answer::DnsAnswer,
    authority::DnsAuthority,
    header::DnsHeader,
    question::DnsQuestion,
};

pub struct DnsMessage {
    header: DnsHeader,
    questions: Vec<DnsQuestion>,
    answers: Vec<DnsAnswer>,
    authorities: Vec<DnsAuthority>,
    additional: Vec<DnsAdditional>,
}

impl DnsMessage {
    pub fn from(data: [u8; 512]) -> u16 {
        u16::from_be_bytes([data[0], data[1]])
    }
}