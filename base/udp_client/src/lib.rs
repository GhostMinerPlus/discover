use std::net::SocketAddr;

use tokio::net::{ToSocketAddrs, UdpSocket};

pub struct DiscoverClent {
    socket: UdpSocket,
}

impl DiscoverClent {
    pub async fn new() -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("couldn't bind to address");

        socket
            .set_multicast_ttl_v4(1)
            .expect("couldn't set broadcast");

        Self { socket }
    }

    pub async fn find(self, multiaddr: impl ToSocketAddrs, cb: impl Fn(SocketAddr)) {
        let mut buf = [0; 1024];
        self.socket
            .send_to("hello".as_bytes(), multiaddr)
            .await
            .expect("couldn't send data");

        while let Ok((_, addr)) = self.socket.recv_from(&mut buf).await {
            cb(addr);
        }
    }
}
