use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{Filter, http::StatusCode, Rejection, reject, reply::json as warp_json};
use std::{env};


#[derive(Clone, Debug, Deserialize, Serialize)]
struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> Self {
        Self { store: HashMap::new() }
    }

    async fn get_value(&self, key: &str) -> Option<String> {
        self.store.get(key).cloned()
    }

    async fn insert_value(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct KeyValue {
    key: String,
    value: String,
}

async fn handle_get(key: String, store: Arc<Mutex<KeyValueStore>>) -> Result<impl warp::Reply, Rejection> {
    let locked_store = store.lock().await;
    let value = locked_store.get_value(&key).await;
    match value {
        Some(value) => Ok(warp_json(&value)),
        None => Err(reject::not_found()),
    }
}

async fn handle_insert(kv: KeyValue, store: Arc<Mutex<KeyValueStore>>) -> Result<impl warp::Reply, Rejection> {
    let mut locked_store = store.lock().await;
    locked_store.insert_value(kv.key, kv.value).await;
    Ok(warp::reply::with_status(warp_json(&json!({"message": "Inserted"})), StatusCode::CREATED))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let store = Arc::new(Mutex::new(KeyValueStore::new()));
    let store_get = store.clone(); // Clone for use in the get route
    let store_insert = store.clone(); // Clone for use in the insert route

    let routes = warp::path("api")
        .and(
            warp::path("get")
                .and(warp::path::param())
                .and(warp::any().map(move || store_get.clone()))
                .and_then(handle_get)
            .or(
                warp::path("insert")
                .and(warp::post())
                .and(warp::body::json())
                .and(warp::any().map(move || store_insert.clone()))
                .and_then(handle_insert)
            )
        );

    let use_tls = env::var("USE_TLS").unwrap_or_default() == "true";
    if use_tls {
        // Here you would insert your TLS setup code requires valid certs
        // For example, using hyper and hyper_rustls (commented out because you have no certificates):
        // let cert_bytes = fs::read("path/to/cert.pem").expect("cannot open certificate file");
        // let key_bytes = fs::read("path/to/key.pem").expect("cannot open key file");
        // Setup TLS server here
    } else {
        warp::serve(routes)
            .run(([127, 0, 0, 1], 3030))
            .await;
    }

    Ok(())
}
