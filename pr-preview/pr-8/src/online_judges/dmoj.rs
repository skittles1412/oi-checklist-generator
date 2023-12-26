use super::{OnlineJudge, OnlineJudgeParser, ParserEvent};
use anyhow::Context;
use async_trait::async_trait;
use reqwest::{Client, IntoUrl, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
struct ApiError {
    code: u32,
    message: String,
}

impl Display for ApiError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "DMOJ API ERROR ({}): {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ApiResult<T> {
    Data(T),
    Error(ApiError),
}

impl<T> From<ApiResult<T>> for anyhow::Result<T> {
    fn from(value: ApiResult<T>) -> Self {
        match value {
            ApiResult::Data(x) => Ok(x),
            ApiResult::Error(e) => Err(e).context("DMOJ api error response")?,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize)]
struct ApiResponse<T> {
    api_version: String,
    method: String,
    fetched: String,
    #[serde(flatten)]
    result: ApiResult<T>,
}

impl<T> From<ApiResponse<T>> for anyhow::Result<T> {
    fn from(value: ApiResponse<T>) -> Self {
        value.result.into()
    }
}

impl<T> ApiResponse<T> {
    fn into_result(self) -> anyhow::Result<T> {
        self.into()
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Problem {
    code: String,
    points: f32,
    partial: bool,
    name: String,
    group: String,
}

#[allow(unused)]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Submission {
    id: u32,
    problem: String,
    user: String,
    date: String,
    language: String,
    time: Option<f32>,
    memory: Option<f32>,
    points: Option<f32>,
    result: Option<String>,
}

async fn get_paginated<T: DeserializeOwned>(
    client: &Client,
    channel: &mpsc::UnboundedSender<ParserEvent>,
    url: impl IntoUrl,
    starting_page: u32,
) -> anyhow::Result<(u32, Vec<T>)> {
    async fn inner<T: DeserializeOwned>(
        client: &Client,
        channel: &mpsc::UnboundedSender<ParserEvent>,
        url: impl IntoUrl,
        starting_page: u32,
    ) -> anyhow::Result<(u32, Vec<T>)> {
        #[derive(Debug, Deserialize)]
        struct Paginated<T> {
            has_more: bool,
            total_pages: u32,
            objects: Vec<T>,
        }

        let url = url.into_url().unwrap();

        let mut res = vec![];

        let mut page = starting_page;
        let mut total_pages = None;

        loop {
            channel
                .send(ParserEvent::Parsing {
                    current_page: page,
                    total_pages,
                })
                .ok();

            let url = {
                let mut url = url.clone();
                url.query_pairs_mut().append_pair("page", &page.to_string());
                url
            };

            let Paginated::<T> {
                has_more,
                total_pages: res_total_pages,
                objects,
            } = client
                .get(url.clone())
                .send()
                .await
                .with_context(|| format!("GET {url}"))?
                .error_for_status()
                .with_context(|| format!("GET {url}"))?
                .json::<ApiResponse<Paginated<T>>>()
                .await
                .with_context(|| format!("GET {url} as json"))?
                .into_result()?;

            total_pages = Some(res_total_pages);

            res.extend(objects.into_iter());
            if !has_more {
                break;
            }

            page += 1;
        }

        Ok((page, res))
    }

    assert!(starting_page >= 1);

    let instant = Instant::now();

    match inner::<T>(client, channel, url, starting_page).await {
        Ok((parsed_pages, t)) => {
            channel
                .send(ParserEvent::Finished {
                    parsed_pages,
                    cached_pages: starting_page - 1,
                    time: instant.elapsed(),
                })
                .ok();
            Ok((parsed_pages, t))
        }
        Err(e) => {
            channel.send(ParserEvent::Error).ok();
            Err(e)
        }
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct ProblemsParser {
    /// time since unix epoch
    last_updated: Duration,
    problems: Vec<Problem>,
}

impl ProblemsParser {
    // 1 week
    const CACHE_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

    async fn parse(
        &mut self,
        client: &Client,
        channel: &mpsc::UnboundedSender<ParserEvent>,
    ) -> anyhow::Result<Vec<Problem>> {
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards... fix your clock");

        if time_now
            .checked_sub(self.last_updated)
            .map(|d| d < ProblemsParser::CACHE_TTL)
            .unwrap_or_default()
        {
            channel.send(ParserEvent::Cached).ok();
            return Ok(self.problems.clone());
        }

        self.problems =
            get_paginated::<Problem>(client, channel, "https://dmoj.ca/api/v2/problems", 1)
                .await
                .context("failed to fetch DMOJ problems")?
                .1;
        self.last_updated = time_now;

        Ok(self.problems.clone())
    }
}

#[derive(Clone, Default, Serialize, Deserialize)]
struct SubmissionsParser {
    parsed_pages: HashMap<String, u32>,
    submissions: HashMap<u32, Submission>,
}

impl SubmissionsParser {
    async fn parse(
        &mut self,
        client: &Client,
        channel: &mpsc::UnboundedSender<ParserEvent>,
        username: &str,
    ) -> anyhow::Result<Vec<Submission>> {
        let starting_page = self.parsed_pages.entry(username.to_string()).or_insert(1);

        let (parsed_pages, new_submissions) = get_paginated::<Submission>(
            client,
            channel,
            Url::parse_with_params("https://dmoj.ca/api/v2/submissions", [("user", username)])
                .unwrap(),
            *starting_page,
        )
        .await?;
        *starting_page = parsed_pages;

        self.submissions
            .extend(new_submissions.into_iter().map(|s| (s.id, s)));

        Ok(self
            .submissions
            .values()
            .filter(|s| s.user == username)
            .cloned()
            .collect())
    }
}

pub struct Config {
    pub username: String,
    pub problems_channel: mpsc::UnboundedSender<ParserEvent>,
    pub submissions_channel: mpsc::UnboundedSender<ParserEvent>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Parser {
    problems: ProblemsParser,
    submissions: SubmissionsParser,
}

#[async_trait]
impl OnlineJudgeParser for Parser {
    type Config = Config;

    async fn parse(
        &mut self,
        client: &Client,
        config: Self::Config,
    ) -> anyhow::Result<OnlineJudge> {
        let Config {
            username,
            problems_channel,
            submissions_channel,
        } = config;

        let (problems, submissions) = tokio::try_join!(
            async {
                self.problems
                    .parse(client, &problems_channel)
                    .await
                    .context("failed to parse DMOJ problems")
            },
            async {
                self.submissions
                    .parse(client, &submissions_channel, &username)
                    .await
                    .context("failed to parse DMOJ submissions")
            }
        )?;

        let problems = problems
            .into_iter()
            .map(|p| (p.code.clone(), p))
            .collect::<HashMap<_, _>>();

        let mut oj = OnlineJudge::default();

        for s in submissions {
            if let Some(points) = s.points {
                let score = 100. * points
                    / problems
                        .get(&s.problem)
                        .with_context(|| format!("no such problem when parsing submission {s:?}"))?
                        .points;
                oj.insert(s.problem, score);
            }
        }

        Ok(oj)
    }
}
