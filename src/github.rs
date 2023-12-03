use serde::{self, Deserialize, Serialize};

/*
 * We are only interested in the following events:
 * - A PR is closed -> pull_request
 * - A PR is merged -> pull_request
 * - A comment is added to a PR -> issue_comment (potentially pull_request_review_comment?? Gotta check)
 * - A review is added to a PR -> pull_request_review
 * - A review is approved -> pull_request_review
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EventTypeHeader {
  PullRequest,
  IssueComment,
  PullRequestReview,
}

impl EventTypeHeader {
  fn from_str(s: &str) -> Option<Self> {
    match s {
      "pull_request" => Some(EventTypeHeader::PullRequest),
      "issue_comment" => Some(EventTypeHeader::IssueComment),
      "pull_request_review" => Some(EventTypeHeader::PullRequestReview),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ActionField {
  Opened,    // PR opened
  Created,   // PR comment created
  Closed,    // PR closed, PR merged
  Submitted, // PR review submitted, PR approved
  Other,     // Other action. We don't care about it.
}

impl<'de> Deserialize<'de> for ActionField {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    let option_string = Option::<String>::deserialize(deserializer)?;
    match option_string {
      Some(string) => match string.as_str() {
        "opened" => Ok(ActionField::Opened),
        "created" => Ok(ActionField::Created),
        "closed" => Ok(ActionField::Closed),
        "submitted" => Ok(ActionField::Submitted),
        _ => Ok(ActionField::Other),
      },
      None => Ok(ActionField::Other),
    }
  }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct Href {
  href: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct PullRequestLinks {
  html: Href,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct User {
  login: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct PullRequest {
  _links: PullRequestLinks,
  number: u32,
  merged_at: Option<String>,
  user: Option<User>,
  title: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct Repository {
  full_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct RawGitHubEvent {
  action: ActionField,
  pull_request: Option<PullRequest>,
  repository: Repository,
}
