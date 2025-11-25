use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};

#[derive(Clone)]
struct CacheEntry<T> {
    data: T,
    timestamp: Instant,
}

pub struct Cache<T> {
    entries: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    ttl: Duration,
}

impl<T: Clone> Cache<T> {
    pub fn new(ttl: Duration) -> Self {
        Self {
            entries: Arc::new(RwLock::new(HashMap::new())),
            ttl,
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        let entries = self.entries.read().ok()?;
        let entry = entries.get(key)?;
        
        if entry.timestamp.elapsed() > self.ttl {
            drop(entries);
            let mut entries = self.entries.write().ok()?;
            entries.remove(key);
            return None;
        }
        
        Some(entry.data.clone())
    }

    pub fn set(&self, key: String, value: T) {
        if let Ok(mut entries) = self.entries.write() {
            entries.insert(
                key,
                CacheEntry {
                    data: value,
                    timestamp: Instant::now(),
                },
            );
        }
    }

    pub fn clear(&self) {
        if let Ok(mut entries) = self.entries.write() {
            entries.clear();
        }
    }

    pub fn invalidate(&self, key: &str) {
        if let Ok(mut entries) = self.entries.write() {
            entries.remove(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_set_get() {
        let cache = Cache::new(Duration::from_secs(60));
        cache.set("key1".to_string(), "value1".to_string());
        
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_cache_expiration() {
        let cache = Cache::new(Duration::from_millis(10));
        cache.set("key1".to_string(), "value1".to_string());
        
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        
        std::thread::sleep(Duration::from_millis(20));
        assert_eq!(cache.get("key1"), None);
    }

    #[test]
    fn test_cache_clear() {
        let cache = Cache::new(Duration::from_secs(60));
        cache.set("key1".to_string(), "value1".to_string());
        cache.set("key2".to_string(), "value2".to_string());
        
        cache.clear();
        
        assert_eq!(cache.get("key1"), None);
        assert_eq!(cache.get("key2"), None);
    }
}

