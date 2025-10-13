use std::net::SocketAddr;

use tokio::net::UdpSocket;

pub struct DiscoverClent {
    socket: UdpSocket,
    port: u16,
}

impl DiscoverClent {
    pub async fn new(port: u16) -> Self {
        let socket = UdpSocket::bind("0.0.0.0:0")
            .await
            .expect("couldn't bind to address");

        socket.set_broadcast(true).expect("couldn't set broadcast");

        Self { socket, port }
    }

    pub async fn find(self, cb: impl Fn(SocketAddr)) {
        let mut buf = [0; 1024];
        self.socket
            .send_to("hello".as_bytes(), format!("255.255.255.255:{}", self.port))
            .await
            .expect("couldn't send data");

        while let Ok((_, addr)) = self.socket.recv_from(&mut buf).await {
            cb(addr);
        }
    }
}
