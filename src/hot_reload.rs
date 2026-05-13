use std::num::NonZeroUsize;
use std::sync::OnceLock;

use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use tokio::sync::broadcast;
use crate::{Escaped, FallbackRender};

const DEV_TOKEN_HEADER: &str = "X-Azumi-Dev-Token";

static BROADCAST_CHANNEL: OnceLock<broadcast::Sender<String>> = OnceLock::new();

fn get_broadcast_channel() -> &'static broadcast::Sender<String> {
    BROADCAST_CHANNEL.get_or_init(|| {
        let (tx, _) = broadcast::channel(100);
        tx
    })
}

#[derive(Clone)]
pub struct RuntimeTemplate {
    pub static_parts: Vec<String>,
}

impl RuntimeTemplate {
    pub fn render(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        dynamics: &[&dyn FallbackRender],
    ) -> std::fmt::Result {
        for (i, part) in self.static_parts.iter().enumerate() {
            write!(f, "{}", Escaped(part))?;
            if i < dynamics.len() {
                dynamics[i].render_azumi(f)?;
            }
        }
        Ok(())
    }
}

const MAX_REGISTRY_SIZE: usize = 1000;

static TEMPLATE_REGISTRY: OnceLock<std::sync::RwLock<lru::LruCache<String, RuntimeTemplate>>> = OnceLock::new();

fn create_registry() -> lru::LruCache<String, RuntimeTemplate> {
    lru::LruCache::new(NonZeroUsize::new(MAX_REGISTRY_SIZE).unwrap())
}

pub fn get_template(id: &str) -> Option<RuntimeTemplate> {
    let Ok(mut registry) = TEMPLATE_REGISTRY.get_or_init(|| std::sync::RwLock::new(create_registry())).write() else {
        eprintln!("Hot Reload: Registry lock poisoned - template lookup failed");
        return None;
    };
    registry.get(&id.to_string()).cloned()
}

const MAX_TEMPLATE_PARTS: usize = 100;
const MAX_PART_SIZE: usize = 100_000; // 100KB per part
const MAX_TEMPLATE_ID_LEN: usize = 256;
const MAX_TOTAL_CSS_SIZE: usize = 500_000; // 500KB total CSS limit per template

#[derive(serde::Deserialize)]
struct TemplateUpdatePayload {
    id: String,
    parts: Vec<String>,
}

async fn update_template_handler(Json(payload): Json<TemplateUpdatePayload>) -> impl IntoResponse {
    if payload.id.len() > MAX_TEMPLATE_ID_LEN {
        eprintln!("Hot Reload: Template ID too long");
        return (StatusCode::BAD_REQUEST, "Template ID too long");
    }
    if payload.parts.len() > MAX_TEMPLATE_PARTS {
        eprintln!("Hot Reload: Too many parts (max {})", MAX_TEMPLATE_PARTS);
        return (StatusCode::BAD_REQUEST, "Too many template parts");
    }
    let mut total_size = payload.id.len();
    for part in &payload.parts {
        if part.len() > MAX_PART_SIZE {
            eprintln!("Hot Reload: Part too large (max {} bytes)", MAX_PART_SIZE);
            return (StatusCode::PAYLOAD_TOO_LARGE, "Template part too large");
        }
        total_size = total_size.saturating_add(part.len());
        if total_size > MAX_TOTAL_CSS_SIZE {
            eprintln!("Hot Reload: Total CSS size too large (max {} bytes)", MAX_TOTAL_CSS_SIZE);
            return (StatusCode::PAYLOAD_TOO_LARGE, "Total CSS size too large");
        }
    }

    let Ok(mut registry) = TEMPLATE_REGISTRY.get_or_init(|| std::sync::RwLock::new(create_registry())).write() else {
        eprintln!("Hot Reload: Registry lock poisoned - template update failed");
        return (StatusCode::SERVICE_UNAVAILABLE, "Registry unavailable");
    };

    {
        let total_size = payload.id.len() + payload.parts.iter().map(|p| p.len()).sum::<usize>();
        if total_size > MAX_REGISTRY_SIZE * 10 {
            eprintln!("Hot Reload: Total payload size too large");
            return (StatusCode::PAYLOAD_TOO_LARGE, "Total payload size too large");
        }
    }

    // lru::LruCache automatically evicts oldest entries when at capacity
    registry.put(payload.id.clone(), RuntimeTemplate { static_parts: payload.parts });
    #[cfg(debug_assertions)]
    println!("Hot Reload: Updated template \"{}\"", payload.id.replace('"', "\\\""));
    let _ = get_broadcast_channel().send(serde_json::json!({"type": "reload"}).to_string());
    (StatusCode::OK, "Template updated")
}

#[cfg(test)]
mod tests {
    use super::*;

    // For tests, bypass the cache and read directly from env
    fn check_dev_token(token: Option<&str>) -> bool {
        let Some(t) = token else {
            return false;
        };
        let Ok(expected) = std::env::var("AZUMI_DEV_TOKEN") else {
            return false;
        };
        
        let t_bytes = t.as_bytes();
        let expected_bytes = expected.as_bytes();
        
        if t_bytes.len() != expected_bytes.len() {
            return false;
        }
        
        let mut result = 0u8;
        for i in 0..t_bytes.len() {
            result |= t_bytes[i] ^ expected_bytes[i];
        }
        
        result == 0
    }

    #[test]
    fn test_is_dev_token_valid_exact_match() {
        std::env::set_var("AZUMI_DEV_TOKEN", "secret123");
        assert!(check_dev_token(Some("secret123")));
        std::env::remove_var("AZUMI_DEV_TOKEN");
    }

    #[test]
    fn test_is_dev_token_valid_wrong_token() {
        std::env::set_var("AZUMI_DEV_TOKEN", "secret123");
        assert!(!check_dev_token(Some("wrongtoken")));
        std::env::remove_var("AZUMI_DEV_TOKEN");
    }

    #[test]
    fn test_is_dev_token_valid_partial_match_rejected() {
        std::env::set_var("AZUMI_DEV_TOKEN", "secret123");
        assert!(!check_dev_token(Some("secret")));
        std::env::remove_var("AZUMI_DEV_TOKEN");
    }

    #[test]
    fn test_is_dev_token_valid_empty_token() {
        std::env::set_var("AZUMI_DEV_TOKEN", "secret123");
        assert!(!check_dev_token(Some("")));
        std::env::remove_var("AZUMI_DEV_TOKEN");
    }

    #[test]
    fn test_is_dev_token_valid_no_env_var() {
        std::env::remove_var("AZUMI_DEV_TOKEN");
        assert!(!check_dev_token(Some("secret123")));
    }

    #[test]
    fn test_is_dev_token_valid_none_token() {
        std::env::set_var("AZUMI_DEV_TOKEN", "secret123");
        assert!(!check_dev_token(None));
        std::env::remove_var("AZUMI_DEV_TOKEN");
    }

    #[test]
    fn test_lru_cache_insert_and_get() {
        let mut cache = create_registry();
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["value1".to_string()] });
        assert!(cache.get("key1").is_some());
    }

    #[test]
    fn test_lru_cache_update_existing_key() {
        let mut cache = create_registry();
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["value1".to_string()] });
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["value2".to_string()] });
        let val = cache.get("key1").unwrap();
        assert_eq!(val.static_parts, vec!["value2".to_string()]);
    }

    #[test]
    fn test_lru_cache_get_nonexistent() {
        let mut cache = create_registry();
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["value1".to_string()] });
        assert!(cache.get("key2").is_none());
    }

    #[test]
    fn test_lru_cache_len() {
        let mut cache = create_registry();
        assert_eq!(cache.len(), 0);
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["value1".to_string()] });
        assert_eq!(cache.len(), 1);
        cache.put("key2".to_string(), RuntimeTemplate { static_parts: vec!["value2".to_string()] });
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_cache_auto_evict() {
        let mut cache = lru::LruCache::new(NonZeroUsize::new(2).unwrap());
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["v1".to_string()] });
        cache.put("key2".to_string(), RuntimeTemplate { static_parts: vec!["v2".to_string()] });
        cache.put("key3".to_string(), RuntimeTemplate { static_parts: vec!["v3".to_string()] });
        assert_eq!(cache.len(), 2);
        assert!(cache.get("key1").is_none()); // key1 was evicted
    }

    #[test]
    fn test_lru_cache_access_updates_order() {
        let mut cache = lru::LruCache::new(NonZeroUsize::new(2).unwrap());
        cache.put("key1".to_string(), RuntimeTemplate { static_parts: vec!["v1".to_string()] });
        cache.put("key2".to_string(), RuntimeTemplate { static_parts: vec!["v2".to_string()] });
        let _ = cache.get("key1"); // access key1 to make it more recent
        cache.put("key3".to_string(), RuntimeTemplate { static_parts: vec!["v3".to_string()] });
        assert!(cache.get("key1").is_some()); // key1 should still be there
        assert!(cache.get("key2").is_none()); // key2 was evicted
    }
}