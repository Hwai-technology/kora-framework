//! High-Performance HTTP Engine for Kora built atop Axum, Tower, and Hyper.
//! Supports Zero-Config routing, route groups, validation, and automatic OpenAPI 3.1 generation.

use std::net::SocketAddr;

use axum::Router;
use axum::routing::{delete, get, post, put};
use serde::{Deserialize, Serialize};

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
        self.router.route(
            "/openapi.json",
            get(|| async move {
                axum::Json(serde_json::from_str::<serde_json::Value>(&openapi_json).unwrap())
            }),
        )
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
                    "200": { "description": "Successful operation" }
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
                "version": "1.0.0"
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
}
