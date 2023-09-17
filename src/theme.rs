use crate::online_judges::ParserEvent;
use console::{style, Style};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{fmt::Display, sync::Arc, time::Duration};
use tokio::{sync::mpsc, task::JoinHandle};

pub struct DmojProgress {
    pub problems_channel: mpsc::UnboundedReceiver<ParserEvent>,
    pub submissions_channel: mpsc::UnboundedReceiver<ParserEvent>,
}

pub struct OjuzProgress {
    pub channel: mpsc::UnboundedReceiver<ParserEvent>,
}

pub struct Theme {
    info_style: Style,
    warn_style: Style,
    error_style: Style,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            info_style: Style::new().dim(),
            warn_style: Style::new().yellow(),
            error_style: Style::new().red(),
        }
    }
}

impl Theme {
    const SPINNER_STEADY_TICK: Duration = Duration::from_millis(75);

    const SPINNER_TICK_CHARS: &'static str = "⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏ ";

    fn spinner_style() -> ProgressStyle {
        ProgressStyle::default_spinner().tick_chars(Self::SPINNER_TICK_CHARS)
    }
}

pub struct ParserProgress<'a> {
    // ensure we can't use the Theme while this progress bar is running
    _theme: &'a mut Theme,
    _mp: MultiProgress,
    drawers: Vec<(JoinHandle<()>, Arc<ProgressBar>)>,
}

impl<'a> ParserProgress<'a> {
    fn new(theme: &'a mut Theme, dmoj: Option<DmojProgress>, ojuz: Option<OjuzProgress>) -> Self {
        let mp = MultiProgress::new();

        let mut drawers = vec![];

        let mut push_progress_bar = |pref, mut rx: mpsc::UnboundedReceiver<ParserEvent>| {
            let pb = ProgressBar::new_spinner().with_style(
                Theme::spinner_style()
                    .template(&format!("{{spinner}} {:<11} {{msg}}", format!("{pref}:")))
                    .expect("failed to create pb style"),
            );

            let pb = Arc::new(mp.add(pb));
            pb.enable_steady_tick(Theme::SPINNER_STEADY_TICK);

            let task = {
                let pb = pb.clone();

                tokio::spawn(async move {
                    while let Some(evt) = rx.recv().await {
                        match evt {
                            ParserEvent::Finished {
                                parsed_pages,
                                cached_pages,
                                time,
                            } => {
                                pb.finish_with_message(
                                    style(format!(
                                        "finished ({parsed_pages} {}, {}{:.2}s)",
                                        if parsed_pages == 1 { "page" } else { "pages" },
                                        if cached_pages > 0 {
                                            format!("{cached_pages} cached, ")
                                        } else {
                                            "".to_string()
                                        },
                                        time.as_secs_f32()
                                    ))
                                    .green()
                                    .to_string(),
                                );
                                break;
                            }
                            ParserEvent::Error => {
                                pb.finish_with_message(style("error occurred").red().to_string());
                                break;
                            }
                            ParserEvent::Cached => {
                                pb.finish_with_message(style("cached").dim().to_string());
                                break;
                            }
                            ParserEvent::Parsing {
                                current_page,
                                total_pages,
                            } => {
                                pb.set_message(format!(
                                    "parsing page {current_page}/{}",
                                    match total_pages {
                                        Some(p) => p.to_string(),
                                        None => "?".to_string(),
                                    }
                                ));
                            }
                        }
                    }

                    if !pb.is_finished() {
                        pb.set_message(style("??? lost contact with worker").yellow().to_string());
                    }
                })
            };

            drawers.push((task, pb));
        };

        if let Some(dmoj) = dmoj {
            push_progress_bar("DMOJ probs", dmoj.problems_channel);
            push_progress_bar("DMOJ subs", dmoj.submissions_channel);
        }
        if let Some(ojuz) = ojuz {
            push_progress_bar("ojuz subs", ojuz.channel);
        }

        Self {
            _theme: theme,
            _mp: mp,
            drawers,
        }
    }

    pub async fn finish(self) -> anyhow::Result<()> {
        for (handle, pb) in self.drawers {
            handle.await?;

            if !pb.is_finished() {
                pb.finish_with_message("canceled");
            }
        }

        Ok(())
    }
}

impl Theme {
    pub fn log_info(&self, d: impl Display) {
        eprintln!("{:>4} {d}", self.info_style.apply_to("INFO"));
    }

    pub fn log_warn(&self, d: impl Display) {
        eprintln!("{:>4} {d}", self.warn_style.apply_to("WARN"));
    }

    pub fn log_error(&self, d: impl Display) {
        eprintln!("{:>4} {d}", self.error_style.apply_to("ERROR"));
    }

    pub fn println(&self, d: impl Display) {
        eprintln!("{d}");
    }

    pub fn display_parser_progress(
        &mut self,
        dmoj: Option<DmojProgress>,
        ojuz: Option<OjuzProgress>,
    ) -> ParserProgress {
        ParserProgress::new(self, dmoj, ojuz)
    }
}
