use tower_http::cors::CorsLayer;

use axum::http::{
  header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
  HeaderValue, Method,
};

pub fn new_cors() -> CorsLayer{
  CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
}