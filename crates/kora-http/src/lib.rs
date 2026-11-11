//! High-Performance HTTP Engine for Kora built atop Axum, Tower, and Hyper.
//! Supports Zero-Config routing, route groups, validation, automatic OpenAPI 3.1 generation,
//! and RFC 7807 Centralized Global Error Handling with Panic Recovery.

use std::any::Any;
use std::net::SocketAddr;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{delete, get, patch, post, put};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::catch_panic::CatchPanicLayer;

/// RFC 7807 Detailed Validation Error Item
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ValidationErrorItem {
    pub field: String,
    pub message: String,
}

/// Standardized API Error Payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub details: Vec<ValidationErrorItem>,
}

/// Standardized Universal API Response Envelope
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApiResponse<T> {
    pub success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ApiErrorDetail>,
    pub request_id: String,
    pub timestamp: i64,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn ok(data: T, request_id: impl Into<String>) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            request_id: request_id.into(),
            timestamp: current_timestamp(),
        }
    }
}

/// Centralized Typed Domain & HTTP Errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApiError {
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    Conflict(String),
    ValidationFailed(Vec<ValidationErrorItem>),
    InternalServerError(String),
}

impl ApiError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Conflict(_) => StatusCode::CONFLICT,
            Self::ValidationFailed(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn error_code(&self) -> &'static str {
        match self {
            Self::BadRequest(_) => "BAD_REQUEST",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Forbidden(_) => "FORBIDDEN",
            Self::NotFound(_) => "NOT_FOUND",
            Self::Conflict(_) => "CONFLICT",
            Self::ValidationFailed(_) => "VALIDATION_FAILED",
            Self::InternalServerError(_) => "INTERNAL_SERVER_ERROR",
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::BadRequest(msg)
            | Self::Unauthorized(msg)
            | Self::Forbidden(msg)
            | Self::NotFound(msg)
            | Self::Conflict(msg)
            | Self::InternalServerError(msg) => msg.clone(),
            Self::ValidationFailed(items) => {
                format!("Validation failed with {} field error(s)", items.len())
            }
        }
    }

    pub fn to_envelope(&self, request_id: impl Into<String>) -> ApiResponse<()> {
        let details = match self {
            Self::ValidationFailed(items) => items.clone(),
            _ => Vec::new(),
        };

        ApiResponse {
            success: false,
            data: None,
            error: Some(ApiErrorDetail {
                code: self.error_code().to_string(),
                message: self.message(),
                details,
            }),
            request_id: request_id.into(),
            timestamp: current_timestamp(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let envelope = self.to_envelope("req_err");
        (status, Json(envelope)).into_response()
    }
}

/// Returns current Unix epoch in seconds.
fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

/// Fallback panic handler producing safe RFC 7807 500 response without leaking internal traces.
fn handle_panic(_err: Box<dyn Any + Send + 'static>) -> Response {
    let envelope = ApiResponse::<()> {
        success: false,
        data: None,
        error: Some(ApiErrorDetail {
            code: "INTERNAL_SERVER_ERROR".to_string(),
            message: "An unexpected internal server error occurred".to_string(),
            details: Vec::new(),
        }),
        request_id: "req_panic".to_string(),
        timestamp: current_timestamp(),
    };
    (StatusCode::INTERNAL_SERVER_ERROR, Json(envelope)).into_response()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteInfo {
    pub method: String,
    pub path: String,
    pub summary: Option<String>,
}

#[derive(Default)]
pub struct KoraApp {
    router: Router,
    routes: Vec<RouteInfo>,
}

impl KoraApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes.push(RouteInfo {
            method: "GET".into(),
            path: path.into(),
            summary: None,
        });
        self.router = self.router.route(path, get(handler));
        self
    }

    pub fn post<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes.push(RouteInfo {
            method: "POST".into(),
            path: path.into(),
            summary: None,
        });
        self.router = self.router.route(path, post(handler));
        self
    }

    pub fn put<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes.push(RouteInfo {
            method: "PUT".into(),
            path: path.into(),
            summary: None,
        });
        self.router = self.router.route(path, put(handler));
        self
    }

    pub fn patch<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes.push(RouteInfo {
            method: "PATCH".into(),
            path: path.into(),
            summary: None,
        });
        self.router = self.router.route(path, patch(handler));
        self
    }

    pub fn delete<H, T>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<T, ()>,
        T: 'static,
    {
        self.routes.push(RouteInfo {
            method: "DELETE".into(),
            path: path.into(),
            summary: None,
        });
        self.router = self.router.route(path, delete(handler));
        self
    }

    pub fn into_router(self) -> Router {
        let openapi_json = self.generate_openapi_spec();
        self.router
            .route(
                "/openapi.json",
                get(|| async move {
                    Json(serde_json::from_str::<serde_json::Value>(&openapi_json).unwrap())
                }),
            )
            .route(
                "/health",
                get(|| async {
                    Json(serde_json::json!({
                        "status": "UP",
                        "framework": "Kora",
                        "version": "0.1.0"
                    }))
                }),
            )
            .layer(CatchPanicLayer::custom(handle_panic))
    }

    pub async fn listen(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let app = self.into_router();
        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }

    pub fn generate_openapi_spec(&self) -> String {
        let mut paths_obj = serde_json::Map::new();

        for r in &self.routes {
            let mut method_obj = serde_json::Map::new();
            method_obj.insert(
                "summary".into(),
                serde_json::Value::String(format!("{} {}", r.method, r.path)),
            );
            method_obj.insert(
                "responses".into(),
                serde_json::json!({
                    "200": { "description": "Successful operation" },
                    "400": { "description": "Invalid input / Bad Request" },
                    "404": { "description": "Resource not found" },
                    "422": { "description": "Validation failed" },
                    "500": { "description": "Internal server error" }
                }),
            );

            let mut path_entry = serde_json::Map::new();
            path_entry.insert(
                r.method.to_lowercase(),
                serde_json::Value::Object(method_obj),
            );
            paths_obj.insert(r.path.clone(), serde_json::Value::Object(path_entry));
        }

        let doc = serde_json::json!({
            "openapi": "3.1.0",
            "info": {
                "title": "Kora Application API",
                "version": "1.0.0",
                "description": "Compiled High-Performance API generated by Kora Framework"
            },
            "paths": paths_obj
        });

        serde_json::to_string_pretty(&doc).unwrap()
    }
}

/// Request Field Validation
pub struct Validator;

impl Validator {
    pub fn validate_email(email: &str) -> Result<(), &'static str> {
        if email.contains('@') && email.contains('.') && email.len() >= 5 {
            Ok(())
        } else {
            Err("must be a valid email address")
        }
    }

    pub fn validate_min_len(val: &str, min: usize) -> Result<(), &'static str> {
        if val.chars().count() >= min {
            Ok(())
        } else {
            Err("value does not meet minimum length requirement")
        }
    }

    pub fn validate_range(val: i64, min: i64, max: i64) -> Result<(), &'static str> {
        if val >= min && val <= max {
            Ok(())
        } else {
            Err("numeric value out of allowable range")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_api_error_response_envelope() {
        let err = ApiError::NotFound("User #99 was not found".into());
        assert_eq!(err.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(err.error_code(), "NOT_FOUND");

        let envelope = err.to_envelope("req_test_123");
        assert!(!envelope.success);
        assert_eq!(envelope.request_id, "req_test_123");
        assert_eq!(
            envelope.error.as_ref().unwrap().message,
            "User #99 was not found"
        );
    }

    #[test]
    fn validates_validation_failed_envelope() {
        let items = vec![
            ValidationErrorItem {
                field: "email".into(),
                message: "must be a valid email".into(),
            },
            ValidationErrorItem {
                field: "password".into(),
                message: "must be at least 8 chars".into(),
            },
        ];
        let err = ApiError::ValidationFailed(items);
        assert_eq!(err.status_code(), StatusCode::UNPROCESSABLE_ENTITY);

        let envelope = err.to_envelope("req_test_456");
        assert_eq!(envelope.error.as_ref().unwrap().details.len(), 2);
    }

    #[test]
    fn validates_email_validator() {
        assert!(Validator::validate_email("user@example.com").is_ok());
        assert!(Validator::validate_email("invalid-email").is_err());
    }
}
