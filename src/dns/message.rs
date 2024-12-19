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
