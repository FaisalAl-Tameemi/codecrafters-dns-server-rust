#[allow(unused_imports)]
use std::net::UdpSocket;

use bytes::BytesMut;
use codecrafters_dns_server::dns::{
    answer::*, common::*, header::*, message::DnsMessage, question::{self, *}
};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}: {:?}", size, source, buf.clone());
                let received_message = DnsMessage::from(buf);

                let header = DnsHeader {
                    id: received_message.header.id,
                    query_response: DnsHeaderQR::Reply,
                    opcode: received_message.header.opcode,
                    authoritative_answer: DnsHeaderAA::NonAuthoritative,
                    truncation: DnsHeaderTC::NotTruncated,
                    recursion_desired: received_message.header.recursion_desired,
                    recursion_available: DnsHeaderRA::RecursionAvailable,
                    z: DnsHeaderZ::Reserved,
                    rcode: match received_message.header.opcode {
                        DnsHeaderOpcode::Query => DnsHeaderRcode::NoError,
                        _ => DnsHeaderRcode::NotImplemented,
                    },
                    question_count: 1,
                    answer_count: 1,
                    authority_count: 0,
                    additional_count: 0,
                };

                let question = DnsQuestion::new(
                    received_message.questions[0].name.0.as_str(),
                    DnsType::A,
                    DnsClass::IN,
                );

                let answer = DnsAnswer::new(
                    received_message.questions[0].name.0.as_str(),
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
