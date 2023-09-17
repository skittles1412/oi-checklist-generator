use super::{OnlineJudge, OnlineJudgeParser, Submission};
use async_trait::async_trait;
use nipper::Document;
use reqwest::{multipart::Form, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Parser {
    submissions: HashMap<u32, Submission>,
}

pub struct Config {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl OnlineJudgeParser for Parser {
    type Config = Config;

    async fn parse(
        &mut self,
        client: &Client,
        config: Self::Config,
    ) -> anyhow::Result<OnlineJudge> {
        let mut new_submissions = vec![];
        'label: for i in 1.. {
            #[allow(clippy::overly_complex_bool_expr)]
            let res = client
                .post(format!(
                    "https://codeforces.com/enter?back=%2Fsubmissions%2F{}%2Fpage%2F{i}",
                    config.username
                ))
                .multipart(
                    Form::new()
                        .text("action", "enter")
                        .text("handleOrEmail", config.username.clone())
                        .text("password", config.password.clone()),
                )
                .send()
                .await?
                .error_for_status()?;

            if res.url().path() != format!("/submissions/{}/page/{i}", config.username) {
                anyhow::bail!("response landed on {} instead of /submisssions/{}/page/{i}, maybe credentials are invalid?", res.url().path(), config.username);
            }
            let html = res.text().await?;

            let document = Document::from(&html);

            if document
                .select(&format!("a[href='/profile/{}']", config.username))
                .length()
                == 0
            {
                anyhow::bail!("response is not signed in");
            }

            let last_page: u32 = document
                .select("span.page-index")
                .last()
                .text()
                .parse()
                .expect("failed to parse last page as an integer");
            if i > last_page {
                break;
            }

            for submission in document.select("tr[data-submission-id]").iter() {
                let id = submission
                    .attr("data-submission-id")
                    .expect("selector guarantees attr")
                    .parse()
                    .expect("submission id is not an integer");
                let problem = submission
                    .select("td[data-problemid] > a[href]")
                    .attr("href")
                    .expect("failed to find problem url");
                let score = if let Some(score) = submission
                    .select(".submissionVerdictWrapper .verdict-format-points")
                    .get(0)
                {
                    score.text().parse().expect("score is not a float")
                } else if submission
                    .select(".submissionVerdictWrapper .verdict-accepted")
                    .length()
                    > 0
                {
                    100.
                } else {
                    0.
                };

                if self.submissions.contains_key(&id) {
                    break 'label;
                }
                new_submissions.push((
                    id,
                    Submission {
                        id,
                        username: config.username.to_string(),
                        problem: problem.to_string(),
                        score,
                    },
                ));
            }
        }
        self.submissions.extend(new_submissions);

        Ok(OnlineJudge::from(
            self.submissions.values(),
            &config.username,
        ))
    }
}
