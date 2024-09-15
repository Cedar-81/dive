mod handlers;
mod store;

use std::{collections::HashMap, sync::Arc};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use handlers::{create_store, get_id, insert};
use serde::{Deserialize, Serialize};
use store::DiveStore;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
pub struct DiveState {
    instances: HashMap<String, DiveStore>,
}

// impl Clone for DiveState {
//     fn clone(&self) -> Self {
//         Self {
//             instances: self.instances.clone(),
//         }
//     }
// }

impl DiveState {
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
        }
    }

    pub fn create_instance(&mut self) -> String {
        let store = DiveStore::new();
        let id = Uuid::new_v4().to_string();
        self.instances.insert(id.clone(), store);

        id
    }

    pub fn insert_data(&mut self, id: String, key: String, val: String) -> Option<()> {
        let mut user_store = self.instances.get_mut(&id);

        match user_store {
            Some(store) => {
                store.set(key, val);
                Some(())
            }
            None => None,
        }
    }
}

pub type SharedState = Arc<RwLock<DiveState>>;

#[tokio::main]
async fn main() {
    let shared_state = SharedState::default();

    // build our application with a route
    let app = Router::new()
        .route("/create-store", post(create_store))
        .route("/get/:id", get(get_id))
        .route("/insert/:instance_id", post(insert))
        .layer(Extension(shared_state));
    // .with_state(DiveState::new().into());

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
    // let mut store: HashMap<String, String> = HashMap::new();

    // let a: Vec<i32> = vec![];

    // let value = store.get("divine");

    // println!("vec len: {}", a.len());

    // let mut store2 = DiveStore::load_store("hello.csv".to_owned());

    // store2
    //     .as_mut()
    //     .unwrap()
    //     .set("1".to_owned(), "Divine".to_owned());
    // store2
    //     .as_mut()
    //     .unwrap()
    //     .set("6".to_owned(), "Yin".to_owned());
    // store2
    //     .as_mut()
    //     .unwrap()
    //     .set("7".to_owned(), "Yang".to_owned());

    // let val1 = store2.as_mut().unwrap().get("7").unwrap();
    // let val2 = store2.as_mut().unwrap().get("7").unwrap();
    // let val3 = store2.as_mut().unwrap().get("6").unwrap();
    // // let val4 = store2.as_mut().unwrap().get("6").unwrap();

    // println!("Recoil map:\n {:?}", store2.as_ref().unwrap().recoil);
    // store2.as_mut().unwrap().lfu_evict();
    // println!("KVstore map:\n {:?}", store2.as_ref().unwrap().kv_store);
    // println!("Recoil map:\n {:?}", store2.as_ref().unwrap().recoil);

    // let _ = store2.unwrap().persist("hello.csv".to_owned());
}
