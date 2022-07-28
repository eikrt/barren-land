use journey::client;
use tokio::{main};
#[tokio::main]
async fn main() {
    client::run().await;
}
