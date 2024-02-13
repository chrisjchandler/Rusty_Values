use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{http::StatusCode, Filter, Rejection, reject, reply::json as warp_json};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct KeyValueStore {
    store: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> Self {
        Self {
            store: HashMap::new(),
        }
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

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let store = Arc::new(Mutex::new(KeyValueStore::new()));

    // Clone `store` for `get_route`
    let store_for_get = store.clone();
    let get_route = warp::path("get")
        .and(warp::path::param())
        .and(warp::any().map(move || store_for_get.clone()))
        .and_then(|key: String, store: Arc<Mutex<KeyValueStore>>| async move {
            let locked_store = store.lock().await;
            let value = locked_store.get_value(&key).await;
            match value {
                Some(value) => Ok::<_, Rejection>(warp_json(&value)),
                None => Err(reject::not_found()),
            }
        });

    // `store` is already cloned for `post_route`
    let post_route = warp::post()
        .and(warp::path("insert"))
        .and(warp::body::json())
        .and(warp::any().map(move || store.clone()))
        .and_then(|kv: KeyValue, store: Arc<Mutex<KeyValueStore>>| async move {
            let mut locked_store = store.lock().await;
            locked_store.insert_value(kv.key, kv.value).await;
            let reply = warp::reply::json(&json!({"message": "Inserted"}));
            Ok::<_, Rejection>(warp::reply::with_status(reply, StatusCode::CREATED))
        });

    let routes = get_route.or(post_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;

    Ok(())
}
