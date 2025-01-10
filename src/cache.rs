use crate::online_judges::{codeforces, dmoj, ojuz, OnlineJudge, OnlineJudgeParser, OnlineJudges};
use anyhow::Context;
use reqwest::{cookie::CookieStore, Client};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Default, Serialize, Deserialize)]
pub struct Cache {
    dmoj: dmoj::Parser,
    ojuz: ojuz::Parser,
    codeforces: codeforces::Parser,
}

impl Cache {
    pub async fn parse(
        &mut self,
        dmoj: Option<dmoj::Config>,
        ojuz: Option<ojuz::Config>,
        codeforces: Option<codeforces::Config>,
        cookie_store: impl CookieStore + 'static,
    ) -> anyhow::Result<OnlineJudges> {
        let client = Client::builder()
            .cookie_provider(Arc::new(cookie_store))
            .build()
            .context("failed to build HTTP client")?;

        tokio::try_join!(
            async {
                match dmoj {
                    Some(dmoj) => self
                        .dmoj
                        .parse(&client, dmoj)
                        .await
                        .context("Parsing DMOJ submissions"),
                    None => Ok(OnlineJudge::default()),
                }
            },
            async {
                match ojuz {
                    Some(ojuz) => self
                        .ojuz
                        .parse(&client, ojuz)
                        .await
                        .context("Parsing oj.uz submissions"),
                    None => Ok(OnlineJudge::default()),
                }
            },
            async {
                match codeforces {
                    Some(codeforces) => self
                        .codeforces
                        .parse(&client, codeforces)
                        .await
                        .context("Parsing CodeForces submissions"),
                    None => Ok(OnlineJudge::default()),
                }
            },
        )
        .map(|(dmoj, ojuz, codeforces)| OnlineJudges {
            dmoj,
            ojuz,
            codeforces,
        })
    }
}
