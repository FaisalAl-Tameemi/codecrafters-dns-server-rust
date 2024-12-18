use bytes::{BufMut, BytesMut};

use super::common::{DnsName, DnsType, DnsClass};

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: DnsName, // encoded as \x07example\x02com\x00
    pub qtype: DnsType,
    pub qclass: DnsClass,
}

impl DnsQuestion {
    pub fn new(name: &str, qtype: DnsType, qclass: DnsClass) -> Self {
        Self { name: DnsName(name.to_string()), qtype, qclass }
    }

    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        // process name parts
        buf.put(self.name.as_buf());
        buf.put_u16(self.qtype.clone() as u16);
        buf.put_u16(self.qclass.clone() as u16);
        buf
    }
}
