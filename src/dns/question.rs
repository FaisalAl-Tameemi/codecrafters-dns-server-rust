use std::str;

use bytes::{BufMut, BytesMut};

use super::common::{DnsName, DnsType, DnsClass};

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: DnsName, // encoded as \x07example\x02com\x00
    pub qtype: DnsType,
    pub qclass: DnsClass,
    pub _length: usize,
}

impl DnsQuestion {
    pub fn new(name: &str, qtype: DnsType, qclass: DnsClass) -> Self {
        Self { name: DnsName(name.to_string()), qtype, qclass, _length: 0 }
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

impl From<&[u8]> for DnsQuestion {
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
            _length: to_skip,
        }
    }
}
