use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
    Extension, RequestPartsExt,
};
use axum_extra::extract::cookie::CookieJar;

// -----------------------------------------------------------------------------
// 1. THE USER MODEL
// -----------------------------------------------------------------------------
#[derive(Clone, Debug)]
pub struct User {
    pub username: String,
}

// -----------------------------------------------------------------------------
// 2. THE MIDDLEWARE (The Gatekeeper)
// -----------------------------------------------------------------------------
pub async fn auth_middleware(
    mut request: axum::extract::Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let jar = CookieJar::from_headers(headers);

    if let Some(user_cookie) = jar.get("azumi_user") {
        // Logged in: Insert User into Extensions
        request.extensions_mut().insert(User {
            username: user_cookie.value().to_string(),
        });
    }
    // Always pass through (for this demo, we allow guests)
    Ok(next.run(request).await)
}

// -----------------------------------------------------------------------------
// 3. THE EXTRACTOR (The Helper)
// -----------------------------------------------------------------------------
// This allows handlers to just ask for `CurrentUser` without worrying about Extensions.
pub struct CurrentUser(pub Option<User>);

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let Extension(user) = parts
            .extract::<Extension<Option<User>>>()
            .await
            .unwrap_or(Extension(None));
        Ok(CurrentUser(user))
    }
}
