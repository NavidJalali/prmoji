use once_cell::sync::Lazy;
use regex::Regex;

use crate::models::PrUrl;

static PR_REGEX: Lazy<Regex> = Lazy::new(|| {
  Regex::new(r"https:\/\/github\.com\/[A-Za-z0-9_.-]+\/[A-Za-z0-9_.-]+\/pull\/\d+").unwrap()
});

pub fn extract_pr_urls(message: &String) -> Vec<PrUrl> {
  PR_REGEX
    .captures_iter(message)
    .map(|cap| PrUrl(cap[0].to_string()))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  fn pr(n: i32) -> String {
    format!("https://github.com/fancy-org/cool-repo/pull/{n}")
  }

  #[test]
  fn find_single_pr_url() {
    let message = format!("Hello please take a look at this pr {}", pr(69).to_string());
    let urls = extract_pr_urls(&message);
    assert_eq!(urls, vec![PrUrl(pr(69))]);
  }

  #[test]
  fn find_multiple_pr_urls() {
    let message = format!(
      "Please take a look at these prs
            {}
            {}
            {}
            ",
      pr(267),
      pr(268),
      pr(269)
    );
    let urls = extract_pr_urls(&message.to_string());
    assert_eq!(
      urls,
      vec![pr(267), pr(268), pr(269)]
        .iter()
        .map(|s| PrUrl(s.to_string()))
        .collect::<Vec<PrUrl>>()
    );
  }

  #[test]
  fn find_no_pr_urls() {
    let message = "Hello please take a look at these nuts lmao";
    let urls = extract_pr_urls(&message.to_string());
    assert_eq!(urls, Vec::<PrUrl>::new());
  }
}
