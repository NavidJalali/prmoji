use serde::{self, Deserialize};

use crate::url_extractor::PrUrl;

/*
 * We are only interested in the following events:
 * - A PR is closed -> pull_request
 * - A PR is merged -> pull_request
 * - A comment is added to a PR -> issue_comment
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
  fn from_raw(s: &str) -> Option<Self> {
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
pub struct User {
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
struct Review {
  state: String,
  user: Option<User>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct Comment {
  user: Option<User>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct IssuePullRequest {
  html_url: Option<String>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct Issue {
  pull_request: Option<IssuePullRequest>,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
struct RawGitHubEvent {
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
enum GitHubEvent {
  Closed { pr_url: PrUrl },
  Merged { pr_url: PrUrl },
  Commented { pr_url: PrUrl, commenter: User },
  ChangesRequested { pr_url: PrUrl, reviewer: User },
  Approved { pr_url: PrUrl, approver: User },
}

impl GitHubEvent {
  fn from_raw(event_type: EventTypeHeader, raw_event: RawGitHubEvent) -> Option<Self> {
    let pr_url = raw_event.get_pr_url()?;
    match (event_type, &raw_event.action) {
      (EventTypeHeader::IssueComment, ActionField::Created) => {
        let commenter = raw_event.comment?.user?;
        Some(GitHubEvent::Commented { pr_url, commenter })
      }
      (EventTypeHeader::PullRequest, ActionField::Closed) => {
        let merged_at = raw_event.pull_request?.merged_at;
        match merged_at {
          Some(_) => Some(GitHubEvent::Merged { pr_url }),
          None => Some(GitHubEvent::Closed { pr_url }),
        }
      }
      (EventTypeHeader::PullRequestReview, ActionField::Submitted) => {
        let review = raw_event.review?;
        let user = review.user?;
        let state = review.state;
        match state.as_str() {
          "changes_requested" => Some(GitHubEvent::ChangesRequested {
            pr_url,
            reviewer: user,
          }),
          "approved" => Some(GitHubEvent::Approved {
            pr_url,
            approver: user,
          }),
          _ => None,
        }
      }
      _ => None,
    }
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
    let raw_event: RawGitHubEvent = load_raw("gh-jsons/approved.json");
    let header = EventTypeHeader::from_raw("pull_request_review").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::Approved {
        approver: User {
          login: "rhalm".to_string()
        },
        pr_url: "https://github.com/NavidJalali/prmoji-testing/pull/2".into(),
      }
    );
  }

  #[test]
  fn can_detect_changes_requested() {
    let raw_event: RawGitHubEvent = load_raw("gh-jsons/request-changes.json");
    let header = EventTypeHeader::from_raw("pull_request_review").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::ChangesRequested {
        reviewer: User {
          login: "rhalm".to_string()
        },
        pr_url: "https://github.com/NavidJalali/prmoji-testing/pull/2".into(),
      }
    );
  }

  #[test]
  fn can_detect_commented() {
    let raw_event: RawGitHubEvent = load_raw("gh-jsons/comment.json");
    let header = EventTypeHeader::from_raw("issue_comment").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::Commented {
        commenter: User {
          login: "NavidJalali".to_string()
        },
        pr_url: "https://github.com/NavidJalali/prmoji-testing/pull/1".into(),
      }
    );
  }

  #[test]
  fn can_detect_merged() {
    let raw_event: RawGitHubEvent = load_raw("gh-jsons/merge.json");
    let header = EventTypeHeader::from_raw("pull_request").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::Merged {
        pr_url: "https://github.com/NavidJalali/prmoji-testing/pull/1".into(),
      }
    );
  }

  #[test]
  fn can_detect_closed() {
    let raw_event: RawGitHubEvent = load_raw("gh-jsons/close.json");
    let header = EventTypeHeader::from_raw("pull_request").unwrap();
    let event = GitHubEvent::from_raw(header, raw_event).unwrap();

    assert_eq!(
      event,
      GitHubEvent::Closed {
        pr_url: "https://github.com/NavidJalali/prmoji-testing/pull/1".into(),
      }
    );
  }
}
