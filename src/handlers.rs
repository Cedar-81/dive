use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{store::DiveStore, DiveState, SharedState};

#[derive(Debug, Deserialize, Serialize)]
pub struct KVData {
    pub data: Vec<(String, String)>,
}

pub async fn get_id(Path(user_id): Path<String>) -> String {
    let mut store2 = DiveStore::load_store("hello.csv".to_owned());

    match store2 {
        Ok(data) => {
            let json_data = serde_json::to_string(&data.kv_store).unwrap();
            return "hello World".to_string();
        }
        Err(err) => "An err occured".to_string(),
    }
    // Html("<h1>Hello, World!</h1>")
}

pub async fn insert(
    Path(instance_id): Path<String>,
    Extension(state): Extension<SharedState>,
    Json(payload): Json<KVData>,
) -> impl IntoResponse {
    let _ = payload.data.iter().map(|(key, val)| async {
        let mut store_state = state.write().await;
        let data = store_state.insert_data(instance_id.clone(), key.to_string(), val.to_string());

        match data {
            Some(_) => (StatusCode::OK, format!("Data inserted successfully")),
            None => (
                StatusCode::BAD_REQUEST,
                format!("Couldn't retrieve store instance with id: {}", instance_id),
            ),
        }
    });
}

pub async fn create_store(
    Extension(state): Extension<SharedState>,
    // Json(payload): Json<KVData>,
) -> impl IntoResponse {
    let mut store_state = state.write().await;
    let user_store_id = store_state.create_instance();

    (
        StatusCode::OK,
        format!("here is ur data: {}", user_store_id),
    )
}
