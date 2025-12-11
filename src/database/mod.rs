use std::{collections::HashMap, sync::Arc};

use tokio::{
    sync::RwLock,
    time::{Duration, Instant},
};

#[derive(Clone)]
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

    pub async fn set(&self, key: String, value: String, ttl: Option<u64>) {
        let mut db_lock = self.db.write().await;
        db_lock.insert(key.clone(), value);

        if let Some(ttl_secs) = ttl {
            let mut expire_lock = self.expire.write().await;
            let expire_time = Instant::now() + Duration::from_secs(ttl_secs);
            expire_lock.insert(key, expire_time);
        }
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        if self.is_expired(key).await {
            self.del(key).await;
            return None;
        }
        let db_lock = self.db.read().await;
        db_lock.get(key).cloned()
    }

    pub async fn del(&self, key: &str) -> bool {
        let mut db_lock = self.db.write().await;
        let mut expire_lock = self.expire.write().await;
        expire_lock.remove(key);
        db_lock.remove(key).is_some()
    }

    pub async fn is_expired(&self, key: &str) -> bool {
        let expire_lock = self.expire.read().await;
        if let Some(&expiry_time) = expire_lock.get(key) {
            Instant::now() > expiry_time
        } else {
            false
        }
    }
}
