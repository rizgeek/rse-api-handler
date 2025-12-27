use api_handler::server::server::server;

#[tokio::main]
async fn main() {
    server().await;
}
