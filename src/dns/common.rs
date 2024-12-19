use bytes::{BufMut, BytesMut};

#[derive(Debug, Clone, Copy)]
pub enum DnsType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    WKS = 11,
    PTR = 12,
}

impl From<u16> for DnsType {
    fn from(value: u16) -> Self {
        match value {
            1 => DnsType::A,
            2 => DnsType::NS,
            5 => DnsType::CNAME,
            6 => DnsType::SOA,
            11 => DnsType::WKS,
            12 => DnsType::PTR,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DnsClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl From<u16> for DnsClass {
    fn from(value: u16) -> Self {
        match value {
            1 => DnsClass::IN,
            2 => DnsClass::CS,
            3 => DnsClass::CH,
            4 => DnsClass::HS,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DnsName {
    pub name: String,
    pub offset: Option<usize>,
    pub length: usize,
}

impl DnsName {
    pub fn new(name: String) -> Self {
        // total length is always 1 byte for each part + 1 byte for the end of the name + characters length in each part
        let length = name.split('.').fold(1, |acc, part| acc + part.len() + 1);

        Self {
            name,
            offset: None,
            length,
        }
    }

    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        // process name parts
        self.name.split('.').for_each(|part| {
            // put the length of the current part
            buf.put_u8(part.len() as u8);
            // put the current part
            buf.put(part.as_bytes());
        });
        // process end of name, 0 byte
        buf.put_u8(0);
        buf
    }

    /// Reads a DNS name from a buffer and returns the name when reading is done along with an optional offset pointer if encountered.
    /// This method assumes the name starts at the beginning of the buffer.
    pub fn read(data: &[u8]) -> Self {
        let mut name_parts: Vec<String> = vec![];
        let mut to_skip = 0;
        let mut has_next_part = true;

        while has_next_part {
            let part_length = data[to_skip];

            if part_length & 0b11000000 == 0b11000000 {
                // Handle compression pointer
                let offset =
                    ((part_length & 0b00111111) as usize) << 8 | data[to_skip + 1] as usize;
                
                return DnsName {
                    name: name_parts.join("."), 
                    offset: Some(offset), 
                    length: to_skip 
                }
            } else if part_length == 0 {
                // End of name
                has_next_part = false;
                to_skip += 1;
            } else {
                // Regular label
                let part = &data[to_skip + 1..to_skip + 1 + part_length as usize];
                name_parts.push(String::from_utf8(part.to_vec()).unwrap());
                to_skip += part_length as usize + 1;
            }
        }

        DnsName { name: name_parts.join("."), offset: None, length: to_skip }
    }
}
