use serde::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelType {
  Channel,
  Group,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Channel(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Text(pub String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Timestamp(String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Message {
  pub channel: Channel,
  pub channel_type: ChannelType,
  pub text: Text,
  pub event_ts: Timestamp,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct AlteredMessage {
  pub text: Text,
  pub user: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Event {
  Create(Message),
  Update(MessageUpdate),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "subtype")]
pub enum MessageUpdate {
  MessageChanged {
    channel: Channel,
    channel_type: ChannelType,
    event_ts: Timestamp,
    message: AlteredMessage,
    previous_message: AlteredMessage,
  },
  MessageDeleted {
    channel: Channel,
    channel_type: ChannelType,
    event_ts: Timestamp,
    previous_message: AlteredMessage,
  },
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum WebookCallback {
  UrlVerification { challenge: String },
  EventCallback { event: Event },
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
  Ok,
  ChallengeReply { challenge: String },
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn can_parse_deletes() {
    let delete_json_path = "./test_resources/delete.json";
    let delete_json = std::fs::read_to_string(delete_json_path).unwrap();
    let delete_message: WebookCallback = serde_json::from_str(&delete_json).unwrap();
    assert_eq!(
      delete_message,
      WebookCallback::EventCallback {
        event: Event::Update(MessageUpdate::MessageDeleted {
          channel: Channel("C05UBF6AJH3".to_string()),
          channel_type: ChannelType::Group,
          event_ts: Timestamp("1696364748.000800".to_string()),
          previous_message: AlteredMessage {
            text: Text("Hello world".to_string()),
            user: "U05TYH6U1K9".to_string(),
          }
        })
      }
    );
  }

  #[test]
  fn can_parse_creates() {
    let create_json_path = "./test_resources/create.json";
    let create_json = std::fs::read_to_string(create_json_path).unwrap();
    let create_message: WebookCallback = serde_json::from_str(&create_json).unwrap();
    assert_eq!(
      create_message,
      WebookCallback::EventCallback {
        event: Event::Create(Message {
          channel: Channel("C05UBF6AJH3".to_string()),
          channel_type: ChannelType::Group,
          text: Text("Hello World!".to_string()),
          event_ts: Timestamp("1696367451.886309".to_string()),
        })
      }
    );
  }

  #[test]
  fn can_parse_edits() {
    let edit_json_path = "./test_resources/edit.json";
    let edit_json = std::fs::read_to_string(edit_json_path).unwrap();
    let edit_message: WebookCallback = serde_json::from_str(&edit_json).unwrap();
    assert_eq!(
      edit_message,
      WebookCallback::EventCallback {
        event: Event::Update(MessageUpdate::MessageChanged {
          channel: Channel("C05UBF6AJH3".to_string()),
          channel_type: ChannelType::Group,
          event_ts: Timestamp("1696367654.001100".to_string()),
          message: AlteredMessage {
            text: Text("Scala is awesome!".to_string()),
            user: "U05TYH6U1K9".to_string(),
          },
          previous_message: AlteredMessage {
            text: Text("Hello World!".to_string()),
            user: "U05TYH6U1K9".to_string(),
          }
        })
      }
    );
  }
}