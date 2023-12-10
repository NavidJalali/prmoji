use serde::{self, Deserialize};

use crate::models::PrUrl;
/*
 * We are only interested in the following events:
 * - A PR is closed -> pull_request
 * - A PR is merged -> pull_request
 * - A comment is added to a PR -> issue_comment
 * - A review is added to a PR -> pull_request_review
 * - A review is approved -> pull_request_review
 */

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventTypeHeader {
  PullRequest,
  IssueComment,
  PullRequestReview,
}

impl EventTypeHeader {
  pub fn from_raw(s: &str) -> Option<Self> {
    match s {
      "pull_request" => Some(EventTypeHeader::PullRequest),
      "issue_comment" => Some(EventTypeHeader::IssueComment),
      "pull_request_review" => Some(EventTypeHeader::PullRequestReview),
      _ => None,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionField {
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
pub struct Href {
  href: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PullRequestLinks {
  html: Href,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct User {
  login: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct PullRequest {
  _links: PullRequestLinks,
  number: u32,
  merged_at: Option<String>,
  user: Option<User>,
  title: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Repository {
  full_name: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Review {
  state: String,
  user: Option<User>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Comment {
  user: Option<User>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct IssuePullRequest {
  html_url: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Issue {
  pull_request: Option<IssuePullRequest>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct RawGitHubEvent {
  action: ActionField,
  pull_request: Option<PullRequest>,
  repository: Repository,
  review: Option<Review>,
  comment: Option<Comment>,
  issue: Option<Issue>,
}

impl RawGitHubEvent {
  fn get_pr_url(&self) -> Option<PrUrl> {
    self
      .pull_request
      .as_ref()
      .map(|pr| &pr._links.html.href)
      .or(
        self
          .issue
          .as_ref()
          .and_then(|issue| issue.pull_request.as_ref())
          .and_then(|pr| pr.html_url.as_ref()),
      )
      .map(|href| PrUrl(href.to_string()))
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GitHubEvent {
  pub pr_url: PrUrl,
  pub event_type: GitHubEventType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GitHubEventType {
  Closed,
  Merged,
  Commented { commenter: User },
  ChangesRequested { reviewer: User },
  Approved { approver: User },
}

impl GitHubEvent {
  pub fn new(pr_url: PrUrl, event_type: GitHubEventType) -> Self {
    Self { pr_url, event_type }
  }

  pub fn from_raw(event_type: EventTypeHeader, raw_event: RawGitHubEvent) -> Option<Self> {
    let pr_url = raw_event.get_pr_url()?;
    let event_type = match (event_type, &raw_event.action) {
      (EventTypeHeader::IssueComment, ActionField::Created) => {
        let commenter = raw_event.comment?.user?;
        Some(GitHubEventType::Commented { commenter })
      }
      (EventTypeHeader::PullRequest, ActionField::Closed) => {
        let merged_at = raw_event.pull_request?.merged_at;
        match merged_at {
          Some(_) => Some(GitHubEventType::Merged),
          None => Some(GitHubEventType::Closed),
        }
      }
      (EventTypeHeader::PullRequestReview, ActionField::Submitted) => {
        let review = raw_event.review?;
        let user = review.user?;
        let state = review.state;
        match state.as_str() {
          "changes_requested" => Some(GitHubEventType::ChangesRequested { reviewer: user }),
          "approved" => Some(GitHubEventType::Approved { approver: user }),
          _ => None,
        }
      }
      _ => None,
    };

    event_type.map(|event_type| Self::new(pr_url, event_type))
  }
}

#[cfg(test)]
mod tests {

  fn load_raw(file_location: &str) -> RawGitHubEvent {
    let json = std::fs::read_to_string(file_location).unwrap();
    serde_json::from_str(&json).unwrap()
  }

  use super::*;
  #[test]
  fn can_detect_approved() {
    let raw_event: RawGitHubEvent = load_raw("test_resources/github/approved.json");
    let header = EventTypeHeader::from_raw("pull_request_review").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::new(
        "https://github.com/NavidJalali/prmoji-testing/pull/2".into(),
        GitHubEventType::Approved {
          approver: User {
            login: "rhalm".to_string()
          },
        }
      )
    );
  }

  #[test]
  fn can_detect_changes_requested() {
    let raw_event: RawGitHubEvent = load_raw("test_resources/github/request-changes.json");
    let header = EventTypeHeader::from_raw("pull_request_review").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::new(
        "https://github.com/NavidJalali/prmoji-testing/pull/2".into(),
        GitHubEventType::ChangesRequested {
          reviewer: User {
            login: "rhalm".to_string()
          },
        }
      )
    );
  }

  #[test]
  fn can_detect_commented() {
    let raw_event: RawGitHubEvent = load_raw("test_resources/github/comment.json");
    let header = EventTypeHeader::from_raw("issue_comment").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::new(
        "https://github.com/NavidJalali/prmoji-testing/pull/1".into(),
        GitHubEventType::Commented {
          commenter: User {
            login: "NavidJalali".to_string()
          },
        }
      )
    );
  }

  #[test]
  fn can_detect_merged() {
    let raw_event: RawGitHubEvent = load_raw("test_resources/github/merge.json");
    let header = EventTypeHeader::from_raw("pull_request").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::new(
        "https://github.com/NavidJalali/prmoji-testing/pull/1".into(),
        GitHubEventType::Merged
      )
    );
  }

  #[test]
  fn can_detect_closed() {
    let raw_event: RawGitHubEvent = load_raw("test_resources/github/close.json");
    let header = EventTypeHeader::from_raw("pull_request").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::new(
        "https://github.com/NavidJalali/prmoji-testing/pull/1".into(),
        GitHubEventType::Closed
      )
    );
  }
}
