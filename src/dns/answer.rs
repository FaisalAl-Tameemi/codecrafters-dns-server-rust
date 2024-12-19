use std::str;

use bytes::{BufMut, BytesMut};

use super::common::{DnsClass, DnsName, DnsType};

#[derive(Debug)]
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

impl From<&[u8]> for DnsAnswer {
    fn from(data: &[u8]) -> Self {
        let mut name_parts: Vec<&str> = vec![];
        let mut to_skip = 0;
        let mut has_next_part = true;
        
        while has_next_part {
            let part_length = data[to_skip];
            let part = &data[to_skip + 1..to_skip + 1 + part_length as usize];
            let name_part = str::from_utf8(part).unwrap();

            name_parts.push(name_part);
            
            match data[to_skip + part_length as usize + 1] {
                0 => {
                    has_next_part = false;
                    to_skip += part_length as usize + 2;
                },
                _ => {
                    to_skip += part_length as usize + 1;
                }
            }
        }

        Self {
            name: DnsName(name_parts.join(".")),
            qtype: DnsType::from(u16::from_be_bytes([data[to_skip], data[to_skip + 1]])),
            qclass: DnsClass::from(u16::from_be_bytes([data[to_skip + 2], data[to_skip + 3]])),
            ttl: 60,
            length: 4,
            data: vec![8,8,8,8],
        }
    }
}
