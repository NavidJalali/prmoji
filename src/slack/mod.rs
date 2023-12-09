pub mod models;

use models::*;

enum SlackClientError {
  ClientSendError(reqwest::Error),
  CannotReadBody(reqwest::Error),
  UnexpectedResponse(reqwest::StatusCode, SlackResponse),
}

#[async_trait::async_trait]
trait SlackClient {
  async fn add_reaction(
    &self,
    payload: AddReactionRequest,
  ) -> Result<SlackResponse, SlackClientError>;
  //async fn send_message(&self, channel: &Channel, text: &Text) -> ();
}

pub struct LiveSlackClient {
  credentials: Credentials,
  http_client: reqwest::Client,
}

#[async_trait::async_trait]
impl SlackClient for LiveSlackClient {
  async fn add_reaction(
    &self,
    payload: AddReactionRequest,
  ) -> Result<SlackResponse, SlackClientError> {
    let response = self
      .http_client
      .post("https://slack.com/api/reactions.add")
      .json(&payload)
      .bearer_auth(&self.credentials.bot_token)
      .send()
      .await
      .map_err(SlackClientError::ClientSendError)?;

    let status = response.status();

    let body = response
      .json::<SlackResponse>()
      .await
      .map_err(SlackClientError::CannotReadBody)?;

    if status.is_success() && body.ok {
      Ok(body)
    } else {
      Err(SlackClientError::UnexpectedResponse(status, body))
    }
  }
}
