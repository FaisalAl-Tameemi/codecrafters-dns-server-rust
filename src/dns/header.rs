use bytes::{BufMut, BytesMut};

use super::common::get_bits_value;

// QR - Query/Response
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderQR {
    Question = 0,
    Reply = 1,
}

impl From<u8> for DnsHeaderQR {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[7]) {
            0 => DnsHeaderQR::Question,
            _ => DnsHeaderQR::Reply,
        }
    }
}

// OPCODE - Operation Code
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderOpcode {
    Query = 0,
    IQuery = 1,
    Status = 2,
    Notify = 4,
    Update = 5,
}

impl From<u8> for DnsHeaderOpcode {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[6, 5, 4, 3]) {
            0 => DnsHeaderOpcode::Query,
            1 => DnsHeaderOpcode::IQuery,
            2 => DnsHeaderOpcode::Status,
            4 => DnsHeaderOpcode::Notify,
            5 => DnsHeaderOpcode::Update,
            _ => panic!("Invalid OPCODE value"),
        }
    }
}

// AA - Authoritative Answer
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderAA {
    NonAuthoritative = 0,
    Authoritative = 1,
}

impl From<u8> for DnsHeaderAA {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[2]) {
            0 => DnsHeaderAA::NonAuthoritative,
            1 => DnsHeaderAA::Authoritative,
            _ => panic!("Invalid AA value"),
        }
    }
}

// TC - Truncated
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderTC {
    NotTruncated = 0,
    Truncated = 1,
}

impl From<u8> for DnsHeaderTC {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[1]) {
            0 => DnsHeaderTC::NotTruncated,
            1 => DnsHeaderTC::Truncated,
            _ => panic!("Invalid TC value"),
        }
    }
}

// RD - Recursion Desired
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderRD {
    RecursionNotDesired = 0,
    RecursionDesired = 1,
}

impl From<u8> for DnsHeaderRD {
    fn from(data: u8) -> Self {
        println!("RD: {:?}", get_bits_value(data, &[0]));

        match get_bits_value(data, &[0]) {
            0 => DnsHeaderRD::RecursionNotDesired,
            1 => DnsHeaderRD::RecursionDesired,
            _ => panic!("Invalid RD value"),
        }
    }
}

// RA - Recursion Available
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderRA {
    RecursionAvailable = 0,
    RecursionNotAvailable = 1,
}

impl From<u8> for DnsHeaderRA {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[7]) {
            0 => DnsHeaderRA::RecursionAvailable,
            1 => DnsHeaderRA::RecursionNotAvailable,
            _ => panic!("Invalid RA value"),
        }
    }
}

// Z - Reserved
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderZ {
    Reserved = 0,
}

impl From<u8> for DnsHeaderZ {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[6, 5, 4]) {
            0 => DnsHeaderZ::Reserved,
            value => panic!("Invalid Z value: {}", value),
        }
    }
}

// RCODE - Response Code
#[derive(Copy, Clone, Debug)]
pub enum DnsHeaderRcode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5,
}

impl From<u8> for DnsHeaderRcode {
    fn from(data: u8) -> Self {
        match get_bits_value(data, &[3, 2, 1, 0]) {
            0 => DnsHeaderRcode::NoError,
            1 => DnsHeaderRcode::FormatError,
            2 => DnsHeaderRcode::ServerFailure,
            3 => DnsHeaderRcode::NameError,
            4 => DnsHeaderRcode::NotImplemented,
            5 => DnsHeaderRcode::Refused,
            _ => panic!("Invalid RCODE value"),
        }
    }
}

// DNS Header
#[derive(Debug)]
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
    pub fn as_buf(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(12);

        // First 2 bytes: ID (16 bits)
        buf.put_u16(self.id);
        // Byte 2: QR (1 bit) | OPCODE (4 bits) | AA (1 bit) | TC (1 bit) | RD (1 bit)
        buf.put_u8(
            (self.query_response as u8) << 7 |
            (self.opcode as u8) << 3 |
            (self.authoritative_answer as u8) << 2 |
            (self.truncation as u8) << 1 |
            (self.recursion_desired as u8)
        );
        // Byte 3: RA (1 bit) | Z (3 bits) | RCODE (4 bits)
        buf.put_u8(
            (self.recursion_available as u8) << 7 |
            (self.z as u8) << 4 |
            (self.rcode as u8)
        );
        // Bytes 4-5: QDCOUNT (16 bits)
        buf.put_u16(self.question_count);
        // Bytes 6-7: ANCOUNT (16 bits)
        buf.put_u16(self.answer_count);
        // Bytes 8-9: NSCOUNT (16 bits)
        buf.put_u16(self.authority_count);
        // Bytes 10-11: ARCOUNT (16 bits)
        buf.put_u16(self.additional_count);

        buf
    }
}

impl From<&[u8]> for DnsHeader {
    fn from(data: &[u8]) -> Self {
        Self {
            id: u16::from_be_bytes([data[0], data[1]]),
            query_response: DnsHeaderQR::from(data[2]),
            opcode: DnsHeaderOpcode::from(data[2]),
            authoritative_answer: DnsHeaderAA::from(data[2]),
            truncation: DnsHeaderTC::from(data[2]),
            recursion_desired: DnsHeaderRD::from(data[2]),
            recursion_available: DnsHeaderRA::from(data[3]),
            z: DnsHeaderZ::from(data[3]),
            rcode: DnsHeaderRcode::from(data[3]),
            question_count: u16::from_be_bytes([data[4], data[5]]),
            answer_count: u16::from_be_bytes([data[6], data[7]]),
            authority_count: u16::from_be_bytes([data[8], data[9]]),
            additional_count: u16::from_be_bytes([data[10], data[11]]),
        }
    }
}
