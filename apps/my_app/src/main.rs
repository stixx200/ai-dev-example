use api::{create_router, AppState};
use core::PetStore;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing for logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "my_app=debug,api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Create the pet store
    let pet_store = PetStore::new();

    // Create application state
    let state = AppState { pet_store };

    // Build the router
    let app = create_router(state);

    // Bind to address
    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    tracing::info!("🚀 Pet Management API Server starting on {}", addr);
    tracing::info!("📖 API Endpoints:");
    tracing::info!("   GET    /health          - Health check");
    tracing::info!("   GET    /pets            - List all pets");
    tracing::info!("   POST   /pets            - Create a new pet");
    tracing::info!("   GET    /pets/:id        - Get a specific pet");
    tracing::info!("   PUT    /pets/:id        - Update a pet");
    tracing::info!("   DELETE /pets/:id        - Delete a pet");
    tracing::info!("\n✨ Server is ready to accept connections!");

    // Start the server
    axum::serve(listener, app)
        .await
        .expect("Failed to start server");
}


