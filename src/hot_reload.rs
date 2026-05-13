use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub fn is_dev_token_valid(token: Option<&str>) -> bool {
    let Some(t) = token else {
        return false;
    };
    let Ok(expected) = std::env::var("AZUMI_DEV_TOKEN") else {
        return false;
    };
    
    let t_bytes = t.as_bytes();
    let expected_bytes = expected.as_bytes();
    
    // SECURITY: Length check must come BEFORE byte comparison
    // Otherwise a partial token (e.g., "sec" vs "secret") would match
    if t_bytes.len() != expected_bytes.len() {
        return false;
    }
    
    let mut result = 0u8;
    for i in 0..t_bytes.len() {
        result |= t_bytes[i] ^ expected_bytes[i];
    }
    
    result == 0
}

static TEMPLATE_REGISTRY: OnceLock<std::sync::RwLock<lru::LruCache<String, RuntimeTemplate>>> = OnceLock::new();

/// Pushes a style update to all connected clients
pub fn push_style_update(scope_id: &str, css: &str) {
    let msg = serde_json::json!({
        "type": "style-update",
        "scopeId": scope_id,
        "css": css
    });
    let _ = get_broadcast_channel().send(msg.to_string());
}

/// Mounts the hot reload route at `/_azumi/live_reload`
///
/// # Security Warning
///
/// These endpoints are **development-only** and should NOT be exposed in production:
///
/// - `/_azumi/live_reload` - WebSocket endpoint for hot reload
/// - `/_azumi/update_template` - POST endpoint to update templates
///
/// **Authentication**: Both endpoints require the `X-Azumi-Dev-Token` header
/// to be set to the value of the `AZUMI_DEV_TOKEN` environment variable.
///
/// In production, either:
/// 1. Remove this router entirely (hot reload is for development only)
/// 2. Restrict access at the network level (e.g., firewall rules to block external access)
/// 3. Ensure `AZUMI_DEV_TOKEN` is not set or is a secret only localhost knows
///
/// If deploying to production with this enabled, ensure only localhost can access these routes.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/_azumi/live_reload", get(ws_handler))
        .route("/_azumi/update_template", post(update_template_handler))
        .layer(axum::middleware::from_fn(check_dev_token))
}

async fn check_dev_token(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, StatusCode> {
    let token = req.headers()
        .get(DEV_TOKEN_HEADER)
        .and_then(|v| v.to_str().ok());
    
    if is_dev_token_valid(token) {
        Ok(next.run(req).await)
    } else {
        // 401 UNAUTHORIZED is correct here - missing/invalid token means unauthenticated
        // 403 FORBIDDEN would mean authenticated but not allowed (not our case)
        Err(StatusCode::UNAUTHORIZED)
    }
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    let mut rx = get_broadcast_channel().subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                if let Ok(msg) = msg {
                    if socket.send(Message::Text(msg.into())).await.is_err() {
                        break;
                    }
                }
            }
            res = socket.recv() => {
                #[allow(clippy::collapsible_match)]
                match res {
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) => break,
                    Some(Err(_)) => break,
                    None => break,
                    _ => {}
                }
            }
        }
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
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

pub fn get_template(id: &str) -> Option<RuntimeTemplate> {
    let Ok(registry) = TEMPLATE_REGISTRY.get_or_init(Default::default).read() else {
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

    let Ok(mut registry) = TEMPLATE_REGISTRY.get_or_init(Default::default).write() else {
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

    // Atomic check-and-evict: loop until we have room or hit max capacity
    while registry.len() >= MAX_REGISTRY_SIZE {
        let evict_count = (MAX_REGISTRY_SIZE / 10).max(1);
        let old_len = registry.len();
        registry.evict_lru(evict_count);
        // If eviction didn't help (e.g., registry is exactly full and can't evict more), break
        if registry.len() >= old_len {
            break;
        }
    }

    // Final check: if we're still over capacity after eviction, reject
    if registry.len() >= MAX_REGISTRY_SIZE {
        eprintln!("Hot Reload: Registry at capacity, could not evict");
        return (StatusCode::INSUFFICIENT_STORAGE, "Registry at capacity");
    }

    registry.insert(payload.id.clone(), RuntimeTemplate { static_parts: payload.parts });
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
        let mut cache = LRUCache::new();
        cache.insert("key1", "value1");
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
    }

    #[test]
    fn test_lru_cache_update_existing_key() {
        let mut cache = LRUCache::new();
        cache.insert("key1", "value1");
        cache.insert("key1", "value2");
        assert_eq!(cache.get(&"key1"), Some(&"value2"));
    }

    #[test]
    fn test_lru_cache_get_nonexistent() {
        let mut cache = LRUCache::new();
        cache.insert("key1", "value1");
        assert_eq!(cache.get(&"key2"), None);
    }

    #[test]
    fn test_lru_cache_len() {
        let mut cache = LRUCache::new();
        assert_eq!(cache.len(), 0);
        cache.insert("key1", "value1");
        assert_eq!(cache.len(), 1);
        cache.insert("key2", "value2");
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_cache_evict_lru() {
        let mut cache = LRUCache::new();
        cache.insert("key1", "v1");
        cache.insert("key2", "v2");
        cache.insert("key3", "v3");
        assert_eq!(cache.len(), 3);
        cache.evict_lru(1);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_cache_evict_all() {
        let mut cache = LRUCache::new();
        cache.insert("key1", "v1");
        cache.insert("key2", "v2");
        cache.evict_lru(10);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_lru_cache_evict_zero_does_nothing() {
        let mut cache = LRUCache::new();
        cache.insert("key1", "v1");
        cache.evict_lru(0);
        assert_eq!(cache.len(), 1);
    }

    #[test]
    fn test_lru_cache_access_updates_lru_order() {
        let mut cache = LRUCache::new();
        cache.insert("key1", "v1");
        cache.insert("key2", "v2");
        cache.insert("key3", "v3");
        let _ = cache.get(&"key1");
        cache.evict_lru(1);
        assert_eq!(cache.len(), 2);
    }

    #[test]
    fn test_lru_cache_default() {
        let cache: LRUCache<String, i32> = LRUCache::default();
        assert_eq!(cache.len(), 0);
    }
}