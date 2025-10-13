use std::time::Duration;

use tokio::time;
use udp_client::DiscoverClent;

#[tokio::main]
async fn main() {
    env_logger::init();

    let client = DiscoverClent::new().await;

    let _ = time::timeout(
        Duration::from_secs(5),
        client.find("239.0.0.1:9000", |addr| {
            println!("find {}", addr);
        }),
    )
    .await;
}
