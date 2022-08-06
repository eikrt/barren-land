use journey::client::Client;
use tokio::{main};
#[tokio::main]
async fn main() {
    let mut client = Client::default();
    client.run().await;
}
