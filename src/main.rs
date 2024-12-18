#[allow(unused_imports)]
use std::net::UdpSocket;

use bytes::{BufMut, BytesMut};
use codecrafters_dns_server::dns::{
    answer::*, common::*, header::*, message::DnsMessage, question::*
};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}: {:?}", size, source, buf.clone());

                let received_message = DnsMessage::from(buf);
                println!("Received message: {}", received_message);

                let header = DnsHeader {
                    id: 1234,
                    query_response: DnsHeaderQR::Reply,
                    opcode: DnsHeaderOpcode::Query,
                    authoritative_answer: DnsHeaderAA::NonAuthoritative,
                    truncation: DnsHeaderTC::NotTruncated,
                    recursion_desired: DnsHeaderRD::RecursionDesired,
                    recursion_available: DnsHeaderRA::RecursionAvailable,
                    z: DnsHeaderZ::Reserved,
                    rcode: DnsHeaderRcode::NoError,
                    question_count: 1,
                    answer_count: 1,
                    authority_count: 0,
                    additional_count: 0,
                };

                let question = DnsQuestion::new(
                    "codecrafters.io",
                    DnsType::A,
                    DnsClass::IN,
                );

                let answer = DnsAnswer::new(
                    "codecrafters.io",
                    DnsType::A,
                    DnsClass::IN,
                    60,
                    vec![8, 8, 8, 8],
                );

                let response = {
                    let mut response: BytesMut = BytesMut::with_capacity(512);

                    let header_buf = header.as_buf();
                    response.extend(header_buf);

                    let question_buf = question.as_buf();
                    response.extend(question_buf);

                    let answer_buf = answer.as_buf();
                    response.extend(answer_buf);

                    response
                };

                println!("Response: {:?}", response);

                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
