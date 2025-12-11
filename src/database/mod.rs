use std::{collections::HashMap, sync::Arc};

use tokio::{sync::RwLock, time::Instant};

pub struct Database {
    pub db: Arc<RwLock<HashMap<String, String>>>,
    pub expire: Arc<RwLock<HashMap<String, Instant>>>,
}

impl Database {
    pub fn new() -> Self {
        Database {
            db: Arc::new(RwLock::new(HashMap::new())),
            expire: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn set(&self, key: String, value: String) {
        let mut db_lock = self.db.write().await;
        db_lock.insert(key, value);
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.del(key).await;
            return None;
        }
        let db_lock = self.db.read().await;
        db_lock.get(key).cloned()
    }

    pub async fn del(&self, key: &str) -> bool{
        let mut db_lock = self.db.write().await;
        db_lock.remove(key).is_some()
    }

    pub async fn set_with_expiry(&self, key: String, duration_secs: u64, value: String) {
        self.set(key.clone(), value).await;
        let mut expire_lock = self.expire.write().await;
        let expire_time = Instant::now() + tokio::time::Duration::from_secs(duration_secs);
        expire_lock.insert(key, expire_time);
    }

    pub async fn is_expired(&self, key: &str) -> bool {
        let expire_lock = self.expire.read().await;
        if let Some(&expiry_time) = expire_lock.get(key) {
            Instant::now() >= expiry_time
        } else {
            false
        }
    }
}