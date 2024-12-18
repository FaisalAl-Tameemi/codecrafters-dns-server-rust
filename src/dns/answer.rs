use bytes::{BufMut, BytesMut};

use super::common::{DnsClass, DnsName, DnsType};

pub struct DnsAnswer {
    name: DnsName,
    qtype: DnsType,
    qclass: DnsClass,
    ttl: u32,
    length: usize,
    data: Vec<u8>,
}

impl DnsAnswer {
    pub fn new(name: &str, qtype: DnsType, qclass: DnsClass, ttl: u32, data: Vec<u8>) -> Self {
        Self { name: DnsName(name.to_string()), qtype, qclass, ttl, length: data.len(), data }
    }

    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        buf.put(self.name.as_buf());
        buf.put_u16(self.qtype as u16);
        buf.put_u16(self.qclass as u16);
        buf.put_u32(self.ttl);
        buf.put_u16(self.length as u16);
        buf.put_slice(self.data.as_slice());
        buf
    }
}
