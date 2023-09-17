use async_trait::async_trait;
use reqwest::Client;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};

pub mod codeforces;
pub mod dmoj;
pub mod ojuz;

#[derive(Debug, Clone)]
pub enum ParserEvent {
    Parsing {
        current_page: u32,
        total_pages: Option<u32>,
    },
    Finished {
        parsed_pages: u32,
        cached_pages: u32,
        time: Duration,
    },
    Error,
    Cached,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Submission {
    id: u32,
    username: String,
    problem: String,
    score: f32,
}

#[async_trait]
pub trait OnlineJudgeParser: Clone + Default + Serialize + DeserializeOwned {
    type Config;

    async fn parse(&mut self, client: &Client, config: Self::Config)
        -> anyhow::Result<OnlineJudge>;
}

#[derive(Debug, Clone, Default)]
pub struct OnlineJudge(HashMap<String, f32>);

impl OnlineJudge {
    fn from<'a>(submissions: impl Iterator<Item = &'a Submission>, username: &str) -> OnlineJudge {
        let mut oj = OnlineJudge::default();

        for s in submissions {
            if s.username == username {
                oj.insert(s.problem.clone(), s.score);
            }
        }

        oj
    }

    fn insert(&mut self, problem_id: String, score: f32) {
        let current_score = self.0.entry(problem_id).or_insert(score);
        *current_score = current_score.max(score);
    }

    pub fn score_for_problem(&self, problem_id: &str) -> Option<f32> {
        self.0.get(problem_id).copied()
    }
}

#[derive(Debug, Clone)]
pub struct OnlineJudges {
    pub dmoj: OnlineJudge,
    pub ojuz: OnlineJudge,
    pub codeforces: OnlineJudge,
}
