use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // 1. Initialize Tracing (Logging)
    // Professional apps always log what they are doing
    println!("Initializing axum-production-blueprint...");

    // 2. Build our application with routes
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/health", get(health_check_handler));

    // 3. Define the address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    // 4. Create a Networking Listener
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("🚀 Server active at http://{}", addr);

    // 5. Start the server
    axum::serve(listener, app).await.unwrap();
}

// --- Handlers ---

async fn hello_handler() -> &'static str {
    "Hello! This is the Axum Production Blueprint."
}

async fn health_check_handler() -> &'static str {
    "OK"
}