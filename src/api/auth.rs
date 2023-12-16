use axum::{
  body::Body,
  extract::{Request, State},
  middleware::Next,
  response::Response,
};
use hmac::{Hmac, Mac};
use http_body_util::BodyExt;
use sha2::Sha256;
use tracing::error;

use crate::{app_state::AppState, clock::Clock};

use super::models::ApiError;

pub fn hmac<'a>(secret: &[u8], message: &[u8]) -> Vec<u8> {
  let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("HMAC can take key of any size");
  mac.update(message);
  let result = mac.finalize();
  result.into_bytes().to_vec()
}

pub fn verify_signature<'a>(secret: &[u8], message: &[u8], signature: &[u8]) -> bool {
  let expected = hmac(secret, message);
  consistenttime::ct_u8_slice_eq(expected.as_slice(), signature)
}

pub async fn authenticate_github_webhook<S: AppState>(
  State(state): State<S>,
  request: Request,
  next: Next,
) -> Result<Response, ApiError> {
  let headers = request.headers();

  let x_hub_signature = headers
    .get("x-hub-signature-256")
    .ok_or(ApiError::new("Missing X-Hub-Signature-256 header", 401))?
    .to_str()
    .map_err(|_| ApiError::new("Invalid X-Hub-Signature-256 header", 400))?
    .strip_prefix("sha256=")
    .ok_or(ApiError::new("Invalid X-Hub-Signature-256 header", 400))?;

  let x_hub_signature = hex::decode(x_hub_signature).map_err(|err| {
    error!("Failed to decode X-Hub-Signature-256 header: {:?}", err);
    ApiError::new("Failed to decode X-Hub-Signature-256 header", 400)
  })?;

  let (parts, body) = request.into_parts();

  let payload = body
    .collect()
    .await
    .map_err(|err| {
      error!("Failed to read request body: {:?}", err);
      ApiError::new("Failed to read request body", 400)
    })?
    .to_bytes();

  let signature = verify_signature(
    &state.config().github.secret(),
    &payload.to_vec().as_slice(),
    &x_hub_signature.as_slice(),
  );

  if signature {
    let response = next
      .run(Request::from_parts(parts, Body::from(payload)))
      .await;
    Ok(response)
  } else {
    error!("Signature mismatch");
    return Err(ApiError::new("Invalid signature", 401));
  }
}

pub async fn authenticate_slack_webhook<S: AppState>(
  State(state): State<S>,
  request: Request,
  next: Next,
) -> Result<Response, ApiError> {
  let headers = request.headers();

  let x_slack_signature = headers
    .get("x-slack-signature")
    .ok_or(ApiError::new("Missing X-Slack-Signature header", 401))?
    .to_str()
    .map_err(|_| ApiError::new("Invalid X-Slack-Signature header", 400))?
    .strip_prefix("v0=")
    .ok_or(ApiError::new("Invalid X-Slack-Signature header", 400))?;

  let x_slack_signature = hex::decode(x_slack_signature).map_err(|_| {
    error!("Failed to decode X-Slack-Signature header");
    ApiError::new("Failed to decode X-Slack-Signature header", 400)
  })?;

  let x_slack_request_timestamp: i64 = headers
    .get("x-slack-request-timestamp")
    .ok_or(ApiError::new(
      "Missing X-Slack-Request-Timestamp header",
      401,
    ))?
    .to_str()
    .map_err(|_| ApiError::new("Invalid X-Slack-Request-Timestamp header", 400))?
    .parse()
    .map_err(|_| ApiError::new("Invalid X-Slack-Request-Timestamp header", 400))?;

  let now = state.clock().now().timestamp();

  if now - x_slack_request_timestamp > 60 * 5 {
    error!("X-Slack-Request-Timestamp header is too old");
    return Err(ApiError::new(
      "X-Slack-Request-Timestamp header is too old",
      401,
    ));
  }

  let (parts, body) = request.into_parts();

  let payload = body
    .collect()
    .await
    .map_err(|err| {
      error!("Failed to read request body: {:?}", err);
      ApiError::new("Failed to read request body", 400)
    })?
    .to_bytes();

  let message = String::from_utf8(payload.to_vec()).unwrap();

  let body = format!("v0:{}:{}", x_slack_request_timestamp, message);

  let signature = verify_signature(
    &state.config().slack.signing_secret(),
    body.as_bytes(),
    x_slack_signature.as_slice(),
  );
  if signature {
    let response = next
      .run(Request::from_parts(parts, Body::from(payload)))
      .await;
    Ok(response)
  } else {
    error!("Signature mismatch");
    return Err(ApiError::new("Invalid signature", 401));
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn hmac_correct() {
    let secret = "It's a Secret to Everybody".as_bytes();
    let message = "Hello, World!".as_bytes();
    let signature = hmac(secret, message);
    let signature = hex::encode(signature);
    let expected = "757107ea0eb2509fc211221cce984b8a37570b6d7586c22c46f4379c8b043e17";

    assert_eq!(signature, expected.to_string());

    assert!(verify_signature(
      secret,
      message,
      hex::decode(expected).unwrap().as_slice()
    ));
  }
}
