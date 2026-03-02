use axum::Json;
use serde_json::{json, Value};

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "success": true,
        "data": {
            "status": "ok",
            "service": "Airi Press",
            "version": env!("CARGO_PKG_VERSION")
        }
    }))
}
