use std::time::Duration;

use tokio::{net::UdpSocket, time};
use udp_client::DiscoverClent;

struct DiscoverServer {
    socket: UdpSocket,
}

impl DiscoverServer {
    async fn new(port: u16) -> Self {
        Self {
            socket: UdpSocket::bind(format!("0.0.0.0:{port}")).await.unwrap(),
        }
    }

    async fn serve(self) {
        let mut buf = [0; 1024];

        while let Ok((_, addr)) = self.socket.recv_from(&mut buf).await {
            tokio::spawn(async move {
                let conn = UdpSocket::bind("0.0.0.0:0").await.unwrap();

                conn.connect(addr).await.unwrap();

                conn.send(&buf).await.unwrap();
            });
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    tokio::spawn(async {
        DiscoverServer::new(9000).await.serve().await;
    });

    let client = DiscoverClent::new(9000).await;

    let _ = time::timeout(
        Duration::from_secs(5),
        client.find(|addr| {
            log::info!("find {}", addr);
        }),
    )
    .await;
}
