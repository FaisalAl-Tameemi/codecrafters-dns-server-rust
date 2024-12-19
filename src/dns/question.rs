use std::{ops::BitAnd, str};

use bit::BitIndex;
use bytes::{BufMut, BytesMut};

use super::common::{DnsClass, DnsName, DnsType};

#[derive(Debug, Clone)]
pub struct DnsQuestion {
    pub name: DnsName, // encoded as \x07example\x02com\x00
    pub qtype: DnsType,
    pub qclass: DnsClass,
    pub length: usize,
}

impl DnsQuestion {
    pub fn new(name: &str, qtype: DnsType, qclass: DnsClass) -> Self {
        Self {
            name: DnsName::new(name.to_string()),
            qtype,
            qclass,
            length: 0,
        }
    }

    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        // process name parts
        buf.put(self.name.as_buf());
        buf.put_u16(self.qtype.clone() as u16);
        buf.put_u16(self.qclass.clone() as u16);
        buf
    }

    pub fn from_buf(data: &[u8; 512], start_index: usize) -> Self {
        let name = DnsName::read(&data[start_index..]);
        let name_length = name.length;
        let mut skip = start_index + name_length;

        // there's a name pointer, read it and resolve
        if let Some(offset) = name.offset {
            let name_completion = DnsName::read(&data[offset..]);
            skip += 2;

            return Self {
                name: DnsName::new(format!("{}.{}", name.name, name_completion.name)),
                qtype: DnsType::from(u16::from_be_bytes([data[skip], data[skip + 1]])),
                qclass: DnsClass::from(u16::from_be_bytes([data[skip + 2], data[skip + 3]])),
                length: name_length + 6,
            }
        }

        // no name pointer, just resolve
        Self {
            name,
            qtype: DnsType::from(u16::from_be_bytes([data[skip], data[skip + 1]])),
            qclass: DnsClass::from(u16::from_be_bytes([data[skip + 2], data[skip + 3]])),
            length: name_length + 4,
        }
    }
}
