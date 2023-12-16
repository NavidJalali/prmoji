use axum::response::{IntoResponse, Response};
use hyper::StatusCode;

#[derive(serde::Serialize, Clone, Copy)]
pub struct ApiError {
  pub message: &'static str,
  pub status_code: u16,
}

impl ApiError {
  pub fn new(message: &'static str, status_code: u16) -> Self {
    Self {
      message,
      status_code,
    }
  }
}

impl IntoResponse for ApiError {
  fn into_response(self) -> axum::response::Response {
    let status_code = StatusCode::from_u16(self.status_code).unwrap();
    let body = serde_json::to_string(&self).unwrap();
    Response::builder()
      .status(status_code)
      .header("Content-Type", "application/json")
      .body(body.into())
      .unwrap()
  }
}
