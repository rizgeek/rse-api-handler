// use std::net::SocketAddr;


// use axum::{routing::get, Router};

use std::{thread, time::Duration};



async fn handler_hello()  {
    thread::sleep(Duration::from_secs(5));
    print!("Hello MF");
}

fn main() {
    // let app: Router = Router::new()
    //     .route("/", get(handler_hello));

    // let addr = SocketAddr::from(([0,0,0,0], 3000));

    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    // axum::serve(listener,app)
    //     .await
    //     .unwrap()
    handler_hello().await;
    println!("Hello world");

}