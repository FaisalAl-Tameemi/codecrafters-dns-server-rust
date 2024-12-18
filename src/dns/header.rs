
// QR - Query/Response
#[derive(Copy, Clone)]
pub enum DnsHeaderQR {
    Question = 0,
    Reply = 1,
}

// OPCODE - Operation Code
#[derive(Copy, Clone)]
pub enum DnsHeaderOpcode {
    Query = 0,
    B = 1,
    C = 2,
    D = 3,
}

// AA - Authoritative Answer
#[derive(Copy, Clone)]
pub enum DnsHeaderAA {
    NonAuthoritative = 0,
    Authoritative = 1,
}

// TC - Truncated
#[derive(Copy, Clone)]
pub enum DnsHeaderTC {
    NotTruncated = 0,
    Truncated = 1,
}

// RD - Recursion Desired
#[derive(Copy, Clone)]
pub enum DnsHeaderRD {
    RecursionDesired = 0,
    RecursionNotDesired = 1,
}

// RA - Recursion Available
#[derive(Copy, Clone)]
pub enum DnsHeaderRA {
    RecursionAvailable = 0,
    RecursionNotAvailable = 1,
}

// Z - Reserved
#[derive(Copy, Clone)]
pub enum DnsHeaderZ {
    Reserved = 0,
}

// RCODE - Response Code
#[derive(Copy, Clone)]
pub enum DnsHeaderRcode {
    NoError = 0,
    FormatError = 1,
}

// DNS Header
pub struct DnsHeader {
    // 16-bit identifier assigned by the program that generates the query
    pub id: u16,
    // 1-bit query/response flag
    pub query_response: DnsHeaderQR,
    // 4-bit opcode
    pub opcode: DnsHeaderOpcode,
    // 1-bit authoritative answer flag
    pub authoritative_answer: DnsHeaderAA,
    // 1-bit truncated flag
    pub truncation: DnsHeaderTC,
    // 1-bit recursion desired flag
    pub recursion_desired: DnsHeaderRD,
    // 1-bit recursion available flag
    pub recursion_available: DnsHeaderRA,
    // 3-bit reserved
    pub z: DnsHeaderZ,
    // 4-bit response code
    pub rcode: DnsHeaderRcode,
    // 16-bit question count
    pub question_count: u16,
    // 16-bit answer count
    pub answer_count: u16,
    // 16-bit authority count
    pub authority_count: u16,
    // 16-bit additional count
    pub additional_count: u16,
}

impl DnsHeader {
    pub fn as_buf(&self) -> [u8; 12] {
        let mut buffer = [0u8; 12];

        // First 2 bytes: ID (16 bits)
        buffer[0] = (self.id >> 8) as u8;
        buffer[1] = self.id as u8;

        // Byte 2: QR (1 bit) | OPCODE (4 bits) | AA (1 bit) | TC (1 bit) | RD (1 bit)
        buffer[2] = (self.query_response as u8) << 7 |
                    (self.opcode as u8) << 3 |
                    (self.authoritative_answer as u8) << 2 |
                    (self.truncation as u8) << 1 |
                    (self.recursion_desired as u8);

        // Byte 3: RA (1 bit) | Z (3 bits) | RCODE (4 bits)
        buffer[3] = (self.recursion_available as u8) << 7 |
                    (self.z as u8) << 4 |
                    (self.rcode as u8);

        // Bytes 4-5: QDCOUNT (16 bits)
        buffer[4] = (self.question_count >> 8) as u8;
        buffer[5] = self.question_count as u8;

        // Bytes 6-7: ANCOUNT (16 bits)
        buffer[6] = (self.answer_count >> 8) as u8;
        buffer[7] = self.answer_count as u8;

        // Bytes 8-9: NSCOUNT (16 bits)
        buffer[8] = (self.authority_count >> 8) as u8;
        buffer[9] = self.authority_count as u8;

        // Bytes 10-11: ARCOUNT (16 bits)
        buffer[10] = (self.additional_count >> 8) as u8;
        buffer[11] = self.additional_count as u8;

        buffer
    }
}