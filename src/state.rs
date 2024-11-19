use crate::models::{Challenge, User};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub type SharedData<T> = Arc<Mutex<Vec<T>>>;
pub type SharedMap<T, V> = Arc<Mutex<HashMap<T, V>>>;

#[derive(Clone)]
pub struct AppState {
    pub users: SharedData<User>,
    pub challenges: SharedMap<Uuid, Challenge>,
    pub meta: SharedMap<Uuid, String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            users: Arc::new(Mutex::new(Vec::new())),
            challenges: Arc::new(Mutex::new(HashMap::new())),
            meta: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
