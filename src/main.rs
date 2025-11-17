use api_handler::routes::server::server;

#[tokio::main]
async fn main() {
    server().await;  
}
