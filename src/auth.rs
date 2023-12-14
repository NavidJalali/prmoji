use hmac::{Hmac, Mac};
use sha2::Sha256;

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

pub fn verify_slack_signature(
  secret: &[u8],
  timestamp: i64,
  message: Vec<u8>,
  signature: Vec<u8>,
) -> bool {
  let message = String::from_utf8(message).unwrap();
  let payload = format!("v0:{}:{}", timestamp, message);
  verify_signature(secret, payload.as_bytes(), signature.as_slice())
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
