#[allow(unused_imports)]
use std::net::UdpSocket;

use codecrafters_dns_server::dns::*;

fn main() {
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let header = DnsHeader {
                    id: 1234,
                    query_response: DnsHeaderQR::Response,
                    opcode: DnsHeaderOpcode::Query,
                    authoritative_answer: DnsHeaderAA::NonAuthoritative,
                    truncation: DnsHeaderTC::NotTruncated,
                    recursion_desired: DnsHeaderRD::RecursionDesired,
                    recursion_available: DnsHeaderRA::RecursionAvailable,
                    z: DnsHeaderZ::Reserved,
                    rcode: DnsHeaderRcode::NoError,
                    question_count: 1,
                    answer_count: 0,
                    authority_count: 0,
                    additional_count: 0,
                };
                let response = header.response();

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
