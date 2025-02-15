use axum::routing::get;
use socketioxide::{extract::SocketRef, SocketIo};
use std::net::{Ipv4Addr, SocketAddrV4};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (layer, io) = SocketIo::new_layer();

    // Register a handler for the default namespace
    io.ns("/", |s: SocketRef| {
        // For each "message" event received, send a "message-back" event with the "Hello World!" event
        s.on("message", |s: SocketRef| {
            s.emit("message-back", "Hello World!").ok();
        });

        s.on("open", |s: SocketRef| {
            let is_connected = s.connected();
            println!(" is connected {is_connected}");

            // s.emit("message-back", "Hello World!").ok();
        });
    });

    log::info!("starting server on http://127.0.0.1:3000");

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .layer(layer)
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let socket_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 3000);
    let listener = tokio::net::TcpListener::bind(socket_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
