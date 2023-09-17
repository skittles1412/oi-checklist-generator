use super::{OnlineJudge, OnlineJudgeParser, ParserEvent, Submission};
use anyhow::Context;
use async_trait::async_trait;
use nipper::Document;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Instant};
use tokio::sync::mpsc;

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Parser {
    submissions: HashMap<u32, Submission>,
}

pub struct Config {
    pub username: String,
    pub channel: mpsc::UnboundedSender<ParserEvent>,
}

impl Parser {
    async fn parse(
        &mut self,
        client: &Client,
        username: &str,
        channel: &mpsc::UnboundedSender<ParserEvent>,
    ) -> anyhow::Result<(u32, OnlineJudge)> {
        // TODO: multithread this part
        let mut new_submissions = vec![];

        let mut page = 1;
        'label: loop {
            channel
                .send(ParserEvent::Parsing {
                    current_page: page,
                    total_pages: None,
                })
                .ok();

            let url = format!("https://oj.uz/submissions?handle={}&page={page}", username);

            let html = client
                .get(&url)
                .send()
                .await
                .with_context(|| format!("GET {url}"))?
                .error_for_status()
                .with_context(|| format!("GET {url}"))?
                .text()
                .await
                .with_context(|| format!("GET {url} as text"))?;

            let document = Document::from(&html);

            let mut found_new_submission = false;

            for submission in document.select("tr").iter() {
                let [Some(id), Some(problem), Some(score)] = [
                    "a[href^='/submission/']",
                    "a[href^='/problem/view/']",
                    ".progressbar > .text",
                ]
                .map(|s| {
                    let selection = submission.select(s);
                    if selection.length() != 1 {
                        None
                    } else {
                        Some(selection)
                    }
                }) else {
                    continue;
                };

                let id = id
                    .attr("href")
                    .expect("selector guarantees href attr")
                    .trim_start_matches("/submission/")
                    .parse()
                    .expect("submission id is not an integer");
                let problem = problem.attr("href").expect("selector guarantees href attr");
                let problem = problem.trim_start_matches("/problem/view/");
                let score = score.text();
                let score = if let Some((num, den)) = score.split_once(" / ") {
                    100. * num.parse::<f32>().expect("failed to parse score")
                        / den.parse::<f32>().expect("failed to parse score")
                } else {
                    assert_eq!(score.as_ref(), "Compilation error", "unrecognized score");
                    0.0
                };

                if self.submissions.contains_key(&id) {
                    break 'label;
                }

                new_submissions.push((
                    id,
                    Submission {
                        id,
                        username: username.to_string(),
                        problem: problem.to_string(),
                        score,
                    },
                ));
                found_new_submission = true;
            }

            if !found_new_submission {
                break;
            }

            page += 1;
        }

        self.submissions.extend(new_submissions);

        Ok((page, OnlineJudge::from(self.submissions.values(), username)))
    }
}

#[async_trait]
impl OnlineJudgeParser for Parser {
    type Config = Config;

    async fn parse(
        &mut self,
        client: &Client,
        config: Self::Config,
    ) -> anyhow::Result<OnlineJudge> {
        let Config { username, channel } = config;

        let instant = Instant::now();

        match self.parse(client, &username, &channel).await {
            Ok((parsed_pages, o)) => {
                channel
                    .send(ParserEvent::Finished {
                        parsed_pages,
                        cached_pages: 0,
                        time: instant.elapsed(),
                    })
                    .ok();
                Ok(o)
            }
            Err(e) => {
                channel.send(ParserEvent::Error).ok();
                Err(e)
            }
        }
    }
}
