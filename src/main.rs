use api_handler::server::app_state::AppState;
use api_handler::server::server::server;
use api_handler::infrastructure::security::bcrypt_hasher::BcryptHasher;

#[tokio::main]
async fn main() {

    let hasher = Box::new(BcryptHasher);

    let state = AppState{
        hasher
    };

    server().await;  
}
