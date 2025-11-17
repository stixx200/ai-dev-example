use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Json, Router,
};
use core::PetStore;
use data::{CreatePetRequest, Pet, UpdatePetRequest};
use serde_json::json;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub pet_store: PetStore,
}

/// Creates the main API router with all endpoints
pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/", get(health_check))
        .route("/health", get(health_check))
        .route("/pets", get(list_pets).post(create_pet))
        .route(
            "/pets/{id}",
            get(get_pet).put(update_pet).delete(delete_pet),
        )
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(Arc::new(state))
}

/// Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "healthy",
        "service": "Pet Management API",
        "version": "1.0.0"
    }))
}

/// Create a new pet (POST /pets)
async fn create_pet(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePetRequest>,
) -> Result<(StatusCode, Json<Pet>), (StatusCode, Json<serde_json::Value>)> {
    match state.pet_store.create_pet(payload) {
        Ok(pet) => Ok((StatusCode::CREATED, Json(pet))),
        Err(e) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": e })),
        )),
    }
}

/// Get a pet by ID (GET /pets/{id})
async fn get_pet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Pet>, (StatusCode, Json<serde_json::Value>)> {
    match state.pet_store.get_pet(&id) {
        Some(pet) => Ok(Json(pet)),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": format!("Pet with ID {} not found", id) })),
        )),
    }
}

/// List all pets (GET /pets)
async fn list_pets(
    State(state): State<Arc<AppState>>,
) -> Json<serde_json::Value> {
    let pets = state.pet_store.list_pets();
    Json(json!({
        "pets": pets,
        "count": pets.len()
    }))
}

/// Update a pet (PUT /pets/{id})
async fn update_pet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePetRequest>,
) -> Result<Json<Pet>, (StatusCode, Json<serde_json::Value>)> {
    match state.pet_store.update_pet(&id, payload) {
        Ok(pet) => Ok(Json(pet)),
        Err(e) => {
            let status = if e.contains("not found") {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::BAD_REQUEST
            };
            Err((status, Json(json!({ "error": e }))))
        }
    }
}

/// Delete a pet (DELETE /pets/{id})
async fn delete_pet(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Pet>, (StatusCode, Json<serde_json::Value>)> {
    match state.pet_store.delete_pet(&id) {
        Ok(pet) => Ok(Json(pet)),
        Err(e) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": e })),
        )),
    }
}

