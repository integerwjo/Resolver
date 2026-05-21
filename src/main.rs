
use std::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0")
        .expect("Failed to bind UDP socket");

    println!("Socket bound to: {:?}", socket.local_addr());

    build_packet().await;

    parse("example.com").await;
}

async fn build_packet() {
    // Builds a DNS packet
    println!("Building DNS packet...");
}

async fn parse(domain: &str) {
    println!("Parsing domain: {}", domain);
}