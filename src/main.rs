#[allow(unused_imports)]
use std::net::UdpSocket;

use bytes::BytesMut;
use codecrafters_dns_server::dns::{
    answer::*, header::*, message::DnsMessage
};

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}: {:?}", size, source, buf.clone());
                let received_message = DnsMessage::from(buf);

                let answers: Vec<DnsAnswer> = received_message.questions.iter().map(|question| {
                    DnsAnswer::new(
                        &question.name.name,
                        question.qtype,
                        question.qclass,
                        60,
                        vec![8, 8, 8, 8],
                    )
                }).collect();

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
                    question_count: received_message.questions.len() as u16,
                    answer_count: answers.len() as u16,
                    authority_count: 0,
                    additional_count: 0,
                };

                let response = {
                    let mut response: BytesMut = BytesMut::with_capacity(512);

                    let header_buf = header.as_buf();
                    response.extend(header_buf);

                    received_message.questions.iter().for_each(|question| {
                        response.extend(question.as_buf());
                    });

                    answers.iter().for_each(|answer| {
                        response.extend(answer.as_buf());
                    });

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
