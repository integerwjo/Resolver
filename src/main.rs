
use std::net::udpSocket;
#[tokio::main]

async fn main() {

    let socket = udpSocket::bind("0.0.0.0:0").unwrap()
}


async fn build_packet() {
    /// This fn builds a dns packet


}

async fn parse(domain: &str) {

}