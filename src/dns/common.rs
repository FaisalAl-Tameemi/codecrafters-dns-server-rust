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

#[derive(Debug, Clone, Copy)]
pub enum DnsClass {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[derive(Debug, Clone)]
pub struct DnsName(pub String);

impl DnsName {
    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::new();
        // process name parts
        self.0.split('.').for_each(|part| {
            // put the length of the current part
            buf.put_u8(part.len() as u8);
            // put the current part
            buf.put(part.as_bytes());
        });
        // process end of name, 0 byte
        buf.put_u8(0);
        buf
    }
}
