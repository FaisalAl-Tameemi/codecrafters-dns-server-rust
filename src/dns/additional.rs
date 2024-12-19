
#[derive(Debug)]
pub struct DnsAdditional {
    name: String,
    qtype: u16,
    qclass: u16,
    ttl: u32,
    data: Vec<u8>,
}
