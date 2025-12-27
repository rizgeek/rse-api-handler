use api_handler::server::server::server;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    server().await;
}
