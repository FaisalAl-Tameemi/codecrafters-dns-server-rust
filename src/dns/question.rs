use bytes::{BufMut, BytesMut};


#[derive(Debug, Clone)]
pub enum DnsQuestionType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    WKS = 11,
    PTR = 12,
}

#[derive(Debug, Clone)]
pub enum DnsQuestionClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: String, // encoded as \x07example\x02com\x00
    pub qtype: DnsQuestionType,
    pub qclass: DnsQuestionClass,
}

impl DnsQuestion {
    pub fn new(name: &str, qtype: DnsQuestionType, qclass: DnsQuestionClass) -> Self {
        Self { name: name.to_string(), qtype, qclass }
    }

    pub fn as_buf(&self) -> Vec<u8> {
        let mut buf = BytesMut::new();
        // process name parts
        self.name.split('.').for_each(|part| {
            // put the length of the current part
            buf.put_u8(part.len() as u8);
            // put the current part
            buf.put(part.as_bytes());
        });
        // process qtype and qclass
        buf.put_u8(0);
        buf.put_u16(self.qtype.clone() as u16);
        buf.put_u16(self.qclass.clone() as u16);
        // process end of question, 0 byte
        buf.put_u8(0);
        
        buf.to_vec()
    }
}
