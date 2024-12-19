#[allow(unused_imports)]
use std::net::UdpSocket;

use clap::Parser;
use codecrafters_dns_server::dns::{answer::DnsAnswer, message::DnsMessage};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "")]
    resolver: String,
}

fn resolve_dns_msg(
    udp_socket: &UdpSocket,
    upstream_addr: &String,
    dns_message: DnsMessage,
) -> DnsMessage {
    let upstream_replies = dns_message
        .questions
        .iter()
        .map(|question| {
            let mut header = dns_message.header.clone();
            header.question_count = 1;
            header.answer_count = 0;
            let msg = DnsMessage {
                header,
                questions: vec![question.clone()],
                answers: vec![],
                authorities: vec![],
                additional: vec![],
            };
            
            udp_socket
                .send_to(&msg.as_buf(), upstream_addr)
                .expect("Failed to send request upstream");
            
            let mut forward_buf = [0; 512];
            udp_socket
                .recv_from(&mut forward_buf)
                .expect("Failed to receive response from upstream");

            return DnsMessage::from(forward_buf);
        })
        .collect();

    return DnsMessage::merge(upstream_replies);
}

fn main() {
    let args = Args::parse();
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    let resolver = args.resolver;
    
    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((_, source)) => {
                // parse stuff
                let received_message = DnsMessage::from(buf);
                
                let response =
                    resolve_dns_msg(&udp_socket, &resolver, received_message);
                
                udp_socket
                    .send_to(&response.as_buf(), source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
