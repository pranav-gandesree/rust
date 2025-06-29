use axum::{routing::get, Router};

mod vehicle;

use vehicle::{get_vehicle, post_vehicle};

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    //create a router
    let router1 = Router::new()
        .route("/example", get(get_vehicle).post(post_vehicle));

    //define the ip and port listencer(TCP)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:1111").await.unwrap();

    //3. axum serve to launch the web server
    axum::serve(listener, router1).await.unwrap();

}
