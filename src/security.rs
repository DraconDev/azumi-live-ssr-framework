use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::env;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

type HmacSha256 = Hmac<Sha256>;

#[cfg(debug_assertions)]
const DEFAULT_SECRET: &str = "azumi-dev-secret-do-not-use-in-prod";
const MAX_STATE_AGE_SECS: u64 = 3600; // 1 hour max age for signed state

static SECRET: OnceLock<String> = OnceLock::new();

fn get_secret() -> &'static str {
    SECRET
        .get_or_init(|| {
            // Check if AZUMI_SECRET is explicitly set (even if empty)
            let explicit_empty = matches!(env::var("AZUMI_SECRET"), Ok(s) if s.is_empty());
            
            let env_secret = env::var("AZUMI_SECRET").unwrap_or_else(|_| {
                #[cfg(debug_assertions)]
                {
                    eprintln!(
                    "⚠️  WARNING: AZUMI_SECRET is not set. Set it for production!"
                );
                    DEFAULT_SECRET.to_string()
                }
                #[cfg(not(debug_assertions))]
                {
                    panic!(
                        "FATAL: AZUMI_SECRET environment variable is REQUIRED in release builds.\n\
                      The default dev secret is publicly known and insecure.\n\
                      Set AZUMI_SECRET to a random 64+ character string before deploying."
                    );
                }
            });

            // If env var was explicitly set to empty, require explicit opt-in for dev mode
            if explicit_empty {
                #[cfg(debug_assertions)]
                {
                    let dev_mode = env::var("AZUMI_DEV_MODE").unwrap_or_default();
                    if dev_mode == "1" || dev_mode.to_lowercase() == "true" {
                        eprintln!("⚠️  WARNING: Using default dev HMAC secret (AZUMI_DEV_MODE enabled). Do NOT use in production!");
                        return DEFAULT_SECRET.to_string();
                    }
                    // In debug mode without dev mode, still allow but warn
                    eprintln!("⚠️  WARNING: AZUMI_SECRET is empty. Set a proper secret for production!");
                    DEFAULT_SECRET.to_string()
                }
                #[cfg(not(debug_assertions))]
                {
                    panic!("FATAL: AZUMI_SECRET cannot be empty. Set a non-empty random string.");
                }
            } else {
                let secret = env_secret;
                if secret.len() < 32 {
                    #[cfg(debug_assertions)]
                    eprintln!(
                        "⚠️  WARNING: AZUMI_SECRET is too short ({} bytes). \
                        For HMAC-SHA256 security, use at least 32 random characters (64+ hex chars).",
                        secret.len()
                    );
                    #[cfg(not(debug_assertions))]
                    panic!(
                        "FATAL: AZUMI_SECRET is too short ({} bytes). \
                        For HMAC-SHA256 security in production, use at least 32 random characters (64+ hex chars).",
                        secret.len()
                    );
                }
                secret
            }
        })
        .as_str()
}

fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .expect("System clock is before UNIX_EPOCH or unavailable - this is a fatal error for state signing")
}

/// Signs a state string with HMAC-SHA256 and includes a timestamp for replay protection.
/// Returns format: "{json}|{timestamp}|{signature_base64}"
///
/// # Format Design
///
/// The pipe `|` delimiter is safe despite JSON potentially containing `|` because:
/// 1. Verification uses `rfind('|')` to locate the signature (rightmost pipe)
/// 2. Then `rfind('|')` on the remainder to locate the timestamp
/// 3. HMAC covers the entire payload, so any tampering with pipe placement invalidates the signature
/// 4. A `TooManyPipes` limit (10) prevents pathological inputs from causing excessive parsing
///
/// For user-scoped signing (prevents replay across users), use `sign_state_for_user`.
#[must_use]
pub fn sign_state(state_json: &str) -> String {
    sign_state_internal(None, state_json)
}

/// Signs a state string scoped to a specific user.
/// Returns format: "azu:{user_id}:{json}|{timestamp}|{signature}"
///
/// The `azu:` prefix is an explicit marker that unambiguously identifies
/// user-scoped state during verification, preventing misidentification
/// of non-user-scoped JSON that happens to contain a colon.
///
/// This prevents replay attacks where User A's state is replayed by User B.
/// User B's verification will fail because the user_id won't match.
#[must_use]
pub fn sign_state_for_user(user_id: &str, state_json: &str) -> String {
    sign_state_internal(Some(user_id), state_json)
}

fn sign_state_internal(user_id: Option<&str>, state_json: &str) -> String {
    let secret = get_secret();
    let timestamp = get_current_timestamp();

    let payload = match user_id {
        Some(uid) => format!("azu:{}:{}", uid, state_json),
        None => state_json.to_string(),
    };

    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");

    mac.update(payload.as_bytes());
    mac.update(&timestamp.to_be_bytes());
    let result = mac.finalize();
    let signature = BASE64.encode(result.into_bytes());

    format!("{}|{}|{}", payload, timestamp, signature)
}

/// Verifies a signed state string and checks timestamp for replay protection.
/// Returns the original JSON if valid, or an Err if invalid or expired.
///
/// # Security Properties
///
/// **WHAT HMAC VERIFIES:**
/// - State integrity: Data was not tampered with after signing
/// - Timestamp freshness: State is within allowed age (1 hour max)
/// - (For user-scoped) User identity: State was signed for a specific user
///
/// **WHAT HMAC DOES NOT VERIFY:**
/// - **User authorization**: A valid signed state does NOT prove the user is allowed to submit this action
/// - **Business logic**: The framework cannot enforce "is this user allowed to modify this data?"
///
/// # Authorization Responsibility
///
/// Action handlers verify that the state was signed by the server, but Azumi does NOT
/// enforce authorization. Your application must verify that the authenticated user
/// is permitted to perform the requested action on the given state.
///
/// Example attack scenario WITHOUT proper authorization:
/// ```text
/// // User A's state contains {role: "user", id: 123}
/// // User A sends this state to the set_admin action
/// // HMAC verifies ✓, state is valid
/// // But User A should NOT be able to make themselves admin!
/// ```
///
/// Your application should validate authorization AFTER verifying state:
/// ```ignore
/// async fn set_admin_handler(signed_state: String) -> Response {
///     let json = verify_state(&signed_state)?; // 1. Verify HMAC
///     let state: UserState = serde_json::from_str(&json)?;
///
///     // 2. Check authorization (YOUR responsibility)
///     let current_user = get_authenticated_user()?;
///     if !current_user.is_admin() && state.id != current_user.id {
///         return Err(Unauthorized); // Can't modify other users!
///     }
///
///     // 3. Perform action
///     state.role = "admin".into();
///     Ok(Json(state))
/// }
/// ```
///
/// # Security Note
///
/// This function uses constant-time HMAC comparison via `verify_slice`,
/// but different validation failures may return at slightly different times.
/// This is considered acceptable as all failures result in rejection,
/// and distinguishing between specific error types provides minimal
/// additional information to an attacker.
pub fn verify_state(signed_state: &str) -> Result<String, VerifyStateError> {
    verify_state_internal_detailed(None, signed_state)
}

/// Verifies a user-scoped signed state.
/// Returns the original JSON if valid, or an Err if invalid/expired/user mismatch.
///
/// Use this when the state was signed with `sign_state_for_user`.
pub fn verify_state_for_user(expected_user_id: &str, signed_state: &str) -> Result<String, VerifyStateError> {
    verify_state_internal_detailed(Some(expected_user_id), signed_state)
}

fn verify_state_internal_detailed(
    expected_user_id: Option<&str>,
    signed_state: &str,
) -> Result<String, VerifyStateError> {
    if signed_state.len() > 100_000 {
        return Err(VerifyStateError::StateTooLarge {
            len: signed_state.len(),
        });
    }

    let pipe_count = signed_state.matches('|').count();
    if pipe_count > 10 {
        return Err(VerifyStateError::TooManyPipes { count: pipe_count });
    }

    let last_pipe = match signed_state.rfind('|') {
        Some(idx) => idx,
        None => return Err(VerifyStateError::MissingPipe),
    };
    let second_last_pipe = match signed_state[..last_pipe].rfind('|') {
        Some(idx) => idx,
        None => return Err(VerifyStateError::MissingPipe),
    };

    let payload_with_ts = &signed_state[..last_pipe];
    let signature_b64 = &signed_state[last_pipe + 1..];

    let timestamp_str = &payload_with_ts[second_last_pipe + 1..];
    let timestamp: u64 = match timestamp_str.parse() {
        Ok(t) => t,
        Err(_) => {
            return Err(VerifyStateError::TimestampParseFailed {
                raw: timestamp_str.to_string(),
            })
        }
    };

    let current_time = get_current_timestamp();

    if timestamp == u64::MAX {
        return Err(VerifyStateError::TimestampMaxValue);
    }

    if current_time.saturating_sub(timestamp) > MAX_STATE_AGE_SECS {
        return Err(VerifyStateError::TimestampExpired {
            ts: timestamp,
            now: current_time,
            max_age: MAX_STATE_AGE_SECS,
        });
    }

    const ALLOWED_CLOCK_SKEW: u64 = 60;
    if timestamp > current_time && timestamp - current_time > ALLOWED_CLOCK_SKEW {
        return Err(VerifyStateError::TimestampFuture {
            ts: timestamp,
            now: current_time,
            skew: ALLOWED_CLOCK_SKEW,
        });
    }

    let payload = &payload_with_ts[..second_last_pipe];
    let (actual_user_id, state_json) = if let Some(rest) = payload.strip_prefix("azu:") {
        match rest.find(':') {
            Some(idx) => {
                let uid = &rest[..idx];
                let json = &rest[idx + 1..];
                if !uid.is_empty() && !json.is_empty() {
                    (Some(uid), json)
                } else {
                    (None, payload)
                }
            }
            None => (None, payload),
        }
    } else {
        (None, payload)
    };

    if let Some(expected) = expected_user_id {
        match actual_user_id {
            Some(actual) if actual == expected => {}
            _ => {
                return Err(VerifyStateError::UserIdMismatch {
                    expected: expected.to_string(),
                    actual: actual_user_id.unwrap_or("").to_string(),
                })
            }
        }
    } else if actual_user_id.is_some() {
        return Err(VerifyStateError::UnexpectedUserId {
            actual: actual_user_id.unwrap_or("").to_string(),
        });
    }

    let secret = get_secret();
    let mut mac =
        HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take any size key");

    mac.update(payload.as_bytes());
    mac.update(&timestamp.to_be_bytes());

    let signature_bytes = match BASE64.decode(signature_b64) {
        Ok(s) => s,
        Err(_) => return Err(VerifyStateError::SignatureDecodeFailed),
    };

    match mac.verify_slice(&signature_bytes) {
        Ok(()) => Ok(state_json.to_string()),
        Err(_) => Err(VerifyStateError::HmacVerificationFailed),
    }
}

/// Error type for state verification failures.
///
/// Provides detailed information about why a signed state failed verification.
/// Use `.to_string()` or `{:#}` for a human-readable description.
///
/// # Security Note
///
/// The `Display` implementation returns a generic "Invalid state" message
/// to avoid leaking internal details to clients. The `Debug` implementation
/// (`{:#?}`) provides detailed diagnostic information for troubleshooting
/// during development.
#[derive(Error)]
pub enum VerifyStateError {
    #[error("Invalid state")]
    StateTooLarge {
        /// The length of the signed state in bytes
        len: usize,
    },
    #[error("Invalid state")]
    TooManyPipes {
        /// Number of '|' separators found
        count: usize,
    },
    #[error("Invalid state")]
    MissingPipe,
    #[error("Invalid state")]
    TimestampParseFailed {
        /// The raw timestamp string that couldn't be parsed
        raw: String,
    },
    #[error("Invalid state")]
    TimestampFuture {
        /// The timestamp in the state
        ts: u64,
        /// Current system time
        now: u64,
        /// Allowed clock skew in seconds
        skew: u64,
    },
    #[error("Invalid state")]
    TimestampExpired {
        /// The timestamp in the state
        ts: u64,
        /// Current system time
        now: u64,
        /// Maximum allowed age in seconds
        max_age: u64,
    },
    #[error("Invalid state")]
    TimestampMaxValue,
    #[error("Invalid state")]
    UserIdMismatch {
        /// The expected user ID
        expected: String,
        /// The actual user ID found in the state
        actual: String,
    },
    #[error("Invalid state")]
    UnexpectedUserId {
        /// The user ID that was unexpectedly present
        actual: String,
    },
    #[error("Invalid state")]
    SignatureDecodeFailed,
    #[error("Invalid state")]
    HmacVerificationFailed,
}

impl std::fmt::Debug for VerifyStateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateTooLarge { .. } => f.debug_struct("StateTooLarge").finish_non_exhaustive(),
            Self::TooManyPipes { .. } => f.debug_struct("TooManyPipes").finish_non_exhaustive(),
            Self::MissingPipe => f.debug_struct("MissingPipe").finish(),
            Self::TimestampParseFailed { .. } => {
                f.debug_struct("TimestampParseFailed").finish_non_exhaustive()
            }
            Self::TimestampFuture { .. } => {
                f.debug_struct("TimestampFuture").finish_non_exhaustive()
            }
            Self::TimestampExpired { .. } => {
                f.debug_struct("TimestampExpired").finish_non_exhaustive()
            }
            Self::TimestampMaxValue => f.debug_struct("TimestampMaxValue").finish(),
            Self::UserIdMismatch { .. } => {
                f.debug_struct("UserIdMismatch").finish_non_exhaustive()
            }
            Self::UnexpectedUserId { .. } => {
                f.debug_struct("UnexpectedUserId").finish_non_exhaustive()
            }
            Self::SignatureDecodeFailed => f.debug_struct("SignatureDecodeFailed").finish(),
            Self::HmacVerificationFailed => f.debug_struct("HmacVerificationFailed").finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_verify() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state(json);
        assert_eq!(signed.matches('|').count(), 2); // json|timestamp|signature

        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_tamper_fails() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state(json);

        // Tamper with the JSON part
        let tampered = signed.replace("10", "99");
        let result = verify_state(&tampered);
        assert!(result.is_err());
    }

    #[test]
    fn test_default_secret_is_obviously_dev() {
        // The default secret must be obviously a dev placeholder
        assert!(
            DEFAULT_SECRET.contains("dev"),
            "Default secret should contain 'dev'"
        );
        assert!(
            DEFAULT_SECRET.contains("do-not-use"),
            "Default secret should contain 'do-not-use'"
        );
        assert!(
            DEFAULT_SECRET.len() < 50,
            "Default secret should be short enough to not look like a real key"
        );
    }

    #[test]
    fn test_sign_verify_empty_string() {
        let json = "";
        let signed = sign_state(json);
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_sign_verify_with_pipes_in_json() {
        // JSON containing '|' should work since we split and take first element
        let json = r#"{"msg": "a|b|c"}"#;
        let signed = sign_state(json);
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_sign_verify_with_many_pipes_in_json() {
        let json = r#"{"data": "||||||||"}"#;
        let signed = sign_state(json);
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_sign_verify_user_scoped_with_pipes_in_json() {
        let json = r#"{"msg": "a|b|c"}"#;
        let signed = sign_state_for_user("user1", json);
        let verified = verify_state_for_user("user1", &signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_invalid_base64_signature() {
        // Use a recent timestamp so it doesn't expire
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let result = verify_state(&format!(r#"{{"count": 10}}|{}|not-valid-base64!!!"#, now));
        assert!(matches!(result, Err(VerifyStateError::SignatureDecodeFailed)));
    }

    #[test]
    fn test_missing_separator() {
        let result = verify_state(r#"{"count": 10}"#);
        assert!(matches!(result, Err(VerifyStateError::MissingPipe)));
    }

    #[test]
    fn test_expired_state_fails() {
        let json = r#"{"count": 10}"#;
        let expired = format!("{}|0|invalid", json);
        let result = verify_state(&expired);
        assert!(matches!(result, Err(VerifyStateError::TimestampExpired { .. })));
    }

    #[test]
    fn test_state_too_large_fails() {
        let json = "x".repeat(100_001);
        let result = verify_state(&json);
        assert!(matches!(result, Err(VerifyStateError::StateTooLarge { .. })));
    }

    #[test]
    fn test_future_timestamp_rejected() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let current = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
        let future_timestamp = current + 120;

        let json = r#"{"count": 10}"#;
        let mut mac = HmacSha256::new_from_slice(b"azumi-dev-secret-do-not-use-in-prod").unwrap();
        mac.update(json.as_bytes());
        mac.update(&future_timestamp.to_be_bytes());
        let sig = base64::engine::general_purpose::STANDARD.encode(mac.finalize().into_bytes());
        let future_state = format!("{}|{}|{}", json, future_timestamp, sig);

        let result = verify_state(&future_state);
        assert!(
            matches!(result, Err(VerifyStateError::TimestampFuture { .. })),
            "Future timestamp beyond clock skew should be rejected"
        );
    }

    #[test]
    fn test_user_scoped_sign_and_verify() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state_for_user("user123", json);
        assert!(signed.starts_with("azu:user123:"));

        let verified = verify_state_for_user("user123", &signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_user_scoped_replay_attack_prevented() {
        let json = r#"{"role": "user"}"#;
        let signed = sign_state_for_user("attacker", json);

        // Victim tries to use attacker's signed state - should fail
        let result = verify_state_for_user("victim", &signed);
        assert!(result.is_err(), "Replay attack should be prevented");
    }

    #[test]
    fn test_user_scoped_without_user_fails() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state_for_user("user123", json);

        // Using verify_state (no user) on user-scoped state should fail
        let result = verify_state(&signed);
        assert!(
            result.is_err(),
            "User-scoped state should not verify without user context"
        );
    }

    #[test]
    fn test_non_user_scoped_state_has_no_prefix() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state(json);

        // Should not start with "azu:" prefix
        assert!(!signed.starts_with("azu:"));

        // verify_state should work fine
        let verified = verify_state(&signed).unwrap();
        assert_eq!(verified, json);
    }

    #[test]
    fn test_json_with_colon_prefix_not_misidentified_as_user_scoped() {
        let json = r#"{"key:value": 1}"#;
        let signed = sign_state(json);
        let result = verify_state(&signed);
        assert!(result.is_ok(), "JSON with colon in key should verify as non-user-scoped");
        assert_eq!(result.unwrap(), json);
    }

    #[test]
    fn test_json_starting_with_alphanumeric_colon_not_misidentified() {
        let json = r#"{"x": true}"#;
        let signed = sign_state(json);
        let result = verify_state(&signed);
        assert!(result.is_ok(), "JSON starting with open-brace should never be misidentified as user-scoped");
    }

    #[test]
    fn test_user_scoped_with_colon_in_json_verifies_correctly() {
        let json = r#"{"msg": "a:b:c"}"#;
        let signed = sign_state_for_user("user1", json);
        let result = verify_state_for_user("user1", &signed);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), json);
    }

    #[test]
    fn test_non_user_scoped_state_not_rejected_by_verify_state() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state(json);
        let result = verify_state(&signed);
        assert!(result.is_ok(), "Non-user-scoped state should verify with verify_state()");
    }

    #[test]
    fn test_user_scoped_state_rejected_by_verify_state_no_user() {
        let json = r#"{"count": 10}"#;
        let signed = sign_state_for_user("alice", json);
        let result = verify_state(&signed);
        assert!(result.is_err(), "User-scoped state must be rejected by verify_state() (no user context)");
    }
}
