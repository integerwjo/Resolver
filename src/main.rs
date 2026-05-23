use std::net::UdpSocket;

#[tokio::main]
async fn main() {
    let socket = create_udp_socket().expect("Failed to bind UDP socket");

    println!("Socket bound to: {:?}", socket.local_addr());

    let domain = "example.com";

    let packet = build_dns_packet().await;
    parse_domain(domain).await;

    println!("Built packet: {:?}", packet);
}

fn create_udp_socket() -> std::io::Result<UdpSocket> {
    UdpSocket::bind("0.0.0.0:0")
}

async fn build_dns_packet() -> Vec<u8> {
    println!("Building DNS packet...");

    // placeholder for real DNS packet data
    vec![0u8; 32]
}

async fn parse_domain(domain: &str) {
    println!("Parsing domain: {}", domain);
}