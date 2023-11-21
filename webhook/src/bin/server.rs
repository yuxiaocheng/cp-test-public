//! Run with
//!
//! ```not_rust
//! cd examples && cargo run -p example-hello-world
//! ```

#![allow(non_snake_case)]

use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
    routing::{get, post},
    Json, Router,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/webhook", post(handle_data));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 9000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

#[derive(Deserialize, Serialize, Debug)]
struct PayLoad {
    #[serde(default)]
    ID: String,
    #[serde(default)]
    Link: String,
    #[serde(default)]
    Operator: String,
    #[serde(default)]
    StartAt: String,
    #[serde(default)]
    CompletedAt: String,
    #[serde(default)]
    Status: String,
    #[serde(default)]
    WorkspaceID: String,
    #[serde(default)]
    ErrorInfo: String,
}

impl PayLoad {
    fn new() -> Self {
        PayLoad {
            ID: String::new(),
            Link: String::new(),
            Operator: String::new(),
            StartAt: String::new(),
            CompletedAt: String::new(),
            Status: String::new(),
            WorkspaceID: String::new(),
            ErrorInfo: String::new(),
        }
    }
}

async fn handle_data(Json(payload): Json<PayLoad>, headers: HeaderMap) -> impl IntoResponse {
    let auth_key = "Authorization";
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("response.log");
    let time = Local::now();
    let code: StatusCode;
    if !headers.contains_key(auth_key) {
        code = StatusCode::BAD_REQUEST;
    } else if headers[auth_key] != "Bearer chaos-test" {
        code = StatusCode::UNAUTHORIZED;
    } else {
        code = StatusCode::OK;
    }

    println!("Time: {time}, Status: {code}, PayLoad: {payload:#?}");
    if file.is_ok() {
        _ = file
            .unwrap()
            .write(format!("Time: {time}, Status: {code}, PayLoad: {payload:#?}").as_bytes())
            .err();
    }

    (code, Json(payload))
}
