use bevy::prelude::*;
use sender_plugin::res::AsyncPool;
use tokio::net::UdpSocket;

pub(super) fn start_server(pool: Res<AsyncPool>) {
    pool.0.spawn(async move {
        let port = 9000;

        let listener = UdpSocket::bind(format!("0.0.0.0:{port}")).await.unwrap();

        let mut buf = [0; 1024];

        while let Ok((_, addr)) = listener.recv_from(&mut buf).await {
            tokio::spawn(async move {
                let conn = UdpSocket::bind("0.0.0.0:0").await.unwrap();

                conn.connect(addr).await.unwrap();

                conn.send(format!("Discover:///:{port}").as_bytes())
                    .await
                    .unwrap();
            });
        }
    });
}
