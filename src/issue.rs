use reqwest::{Client, header::USER_AGENT};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;
use futures::{stream::iter, Stream, StreamExt};

#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
pub struct GithubIssueMinimal {
    pub html_url: String,
    pub title: String,
    #[serde(skip)]
    _private: ()
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"(?:^|\s)#(\d+)").unwrap();
}

pub fn get_issues<'a>(msg: &'a str, repo: &str) -> impl Stream<Item=GithubIssueMinimal> + 'a + Send {
    let url = format!("https://api.github.com/repos/{}/issues/", repo);
    let matches = RE.captures_iter(msg).collect::<Vec<_>>();
    iter(
        matches.into_iter()
            .map(|c| c.get(1).unwrap().as_str())
            .map(move |id| format!("{}{}", url, id))
    )
        .filter_map(|url| async move {
            let resp = Client::new()
                .get(&url)
                .header(USER_AGENT, "ETLBot")
                .send()
                .await
                .map_err(|e| println!("{}", e)).ok()
                .filter(|resp| resp.status().is_success())?;
            resp.json().await.map_err(|e| println!("{}", e)).ok()
        })
}
