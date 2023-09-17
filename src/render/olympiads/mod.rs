use super::{OlympiadProps, ProblemLocation, ProblemProps, SeasonProps};
use crate::online_judges::OnlineJudges;
use std::borrow::Cow;

pub mod apio;
pub mod boi;
pub mod ccc;
pub mod cco;
pub mod ceoi;
pub mod cnoi;
pub mod ioi;
pub mod joi_open;
pub mod joisc;

#[derive(Debug, Copy, Clone)]
enum OnlineJudge {
    Dmoj,
    Ojuz,
    Codeforces,
}

#[derive(Debug, Copy, Clone)]
struct OjSource {
    name: &'static str,
    id: &'static str,
    url: &'static str,
    oj: OnlineJudge,
}

trait ToProblemLocation {
    fn to_problem_location(&self, ojs: &OnlineJudges) -> ProblemLocation;
}

impl ToProblemLocation for OjSource {
    fn to_problem_location(&self, ojs: &OnlineJudges) -> ProblemLocation {
        ProblemLocation {
            name: self.name,
            url: self.url,
            best_score: match self.oj {
                OnlineJudge::Dmoj => &ojs.dmoj,
                OnlineJudge::Ojuz => &ojs.ojuz,
                OnlineJudge::Codeforces => &ojs.codeforces,
            }
            .score_for_problem(self.id),
        }
    }
}

type DynLocation = &'static dyn ToProblemLocation;

struct Year {
    year: u32,
    name: &'static str,
    problems: &'static [Problem],
}

struct Problem {
    name: &'static str,
    sources: &'static [DynLocation],
}

fn to_olympiad(
    ojs: &OnlineJudges,
    data: &[Year],
    name: &'static str,
    mut format_long_name: impl FnMut(&Year, &Problem, u32) -> Cow<'static, str>,
) -> OlympiadProps {
    OlympiadProps {
        name,
        seasons: data
            .iter()
            .map(|y| SeasonProps {
                name: y.name,
                problems: y
                    .problems
                    .iter()
                    .zip(1..)
                    .map(|(p, i)| ProblemProps {
                        short_name: p.name.into(),
                        long_name: format_long_name(y, p, i),
                        locations: p
                            .sources
                            .iter()
                            .map(|s| s.to_problem_location(ojs))
                            .collect(),
                    })
                    .collect(),
            })
            .collect(),
    }
}

// adapted from https://www.reddit.com/r/rust/comments/lz0xsl/concatenating_arrays_at_compiletime_in_rust_151/
const fn concat_array_helper<const A: usize, const B: usize, const C: usize>(
    a: [DynLocation; A],
    b: [DynLocation; B],
) -> [DynLocation; C] {
    // assert A + B == C
    {
        struct LengthCheck<const A: usize, const B: usize, const C: usize>;
        impl<const A: usize, const B: usize, const C: usize> LengthCheck<A, B, C> {
            const ASSERT: () = assert!(A + B == C);
        }

        #[allow(path_statements)]
        LengthCheck::<A, B, C>::ASSERT;
    }

    // so I don't deal with MaybeUninit's
    let mut result: [DynLocation; C] = [&OjSource {
        name: "",
        id: "",
        url: "",
        oj: OnlineJudge::Dmoj,
    }; C];

    let mut i = 0;
    while i < A {
        result[i] = a[i];
        i += 1;
    }

    while i < A + B {
        result[i] = b[i - A];
        i += 1;
    }

    result
}

macro_rules! concat_array {
    ($a:expr, $b:expr $(,)?) => {
        $crate::render::olympiads::concat_array_helper::<
            { $a.len() },
            { $b.len() },
            { $a.len() + $b.len() },
        >($a, $b)
    };
}

macro_rules! dmoj {
    (no_dmoj) => {
        [] as [$crate::render::olympiads::DynLocation; 0]
    };
    (@from_id $name:expr, $id:expr) => {
        [&$crate::render::olympiads::OjSource {
            name: $name,
            id: $id,
            url: concat!("https://dmoj.ca/problem/", $id),
            oj: $crate::render::olympiads::OnlineJudge::Dmoj,
        }]
    };
    (@io $id:expr) => {
        $crate::render::olympiads::dmoj!(@from_id "DMOJ (io)", concat!(stringify!($id), "io"))
    };
    ($id:ident) => {
        $crate::render::olympiads::dmoj!(@from_id "DMOJ", stringify!($id))
    };
    ($id:ident+io) => {
        $crate::render::olympiads::concat_array!(
            $crate::render::olympiads::dmoj!($id),
            $crate::render::olympiads::dmoj!(@from_id "DMOJ (io)", concat!(stringify!($id), "io")),
        )
    };
    ($id:ident+io_only) => {
        $crate::render::olympiads::dmoj!(@io $id)
    }
}

macro_rules! ojuz {
    (no_ojuz) => {
        [] as [$crate::render::olympiads::DynLocation; 0]
    };
    ($x:ident) => {
        [&$crate::render::olympiads::OjSource {
            name: "ojuz",
            id: stringify!($x),
            url: concat!("https://oj.uz/problem/view/", stringify!($x)),
            oj: $crate::render::olympiads::OnlineJudge::Ojuz,
        }]
    };
}

// arcane macro magic
// basically this whole module is an over engineered mess to be able to store everything at compile time
macro_rules! olympiad {
    (@chain {$($head:tt)*} $($tail:tt)*) => {
        $crate::render::olympiads::concat_array!(
            $($head)*,
            $crate::render::olympiads::olympiad!(@chain $($tail)*)
        )
    };
    (@chain) => {
        [] as [$crate::render::olympiads::DynLocation; 0]
    };
    (@problem (
        $($cb:ident),*;
        {$name:expr, $($id:tt $(+$s1:ident)? $(/$s2:ident)?),*};
        $({$($tail:tt)*};)*
    ) -> [$($body:tt)*]) => {
        $crate::render::olympiads::olympiad!(@problem (
            $($cb),*;
            $({$($tail)*};)*
        ) -> [
            $($body)*
            $crate::render::olympiads::Problem {
                name: $name,
                sources: &$crate::render::olympiads::olympiad!(@chain $({
                    $cb!($id $(+$s1)? $(/$s2)?)
                })*)
            },
        ])
    };
    (@problem ($($cb:ident),*;) -> [$($body:tt)*]) => {
        [$($body)*]
    };
    (@year (
        $year_prefix:expr;
        $($cb:ident),*;
        $year:expr => $({ $($problem:tt)* }),*;
        $($tail:tt)*
    ) -> [$($body:tt)*]) => {
        $crate::render::olympiads::olympiad!(@year (
            $year_prefix;
            $($cb),*;
            $($tail)*
        ) -> [
            $($body)*
            $crate::render::olympiads::Year {
                year: $year,
                name: concat!($year_prefix, " ", $year),
                problems: &$crate::render::olympiads::olympiad!(@problem (
                    $($cb),*;
                    $({ $($problem)* };)*
                ) -> [])
            },
        ])
    };
    (@year ($year_prefix:expr; $($cb:ident),*;) -> [$($body:tt)*]) => {
        [$($body)*]
    };
    (
        $year_prefix:expr;
        order: $($cb:ident),*;
        $($years:tt)*
    ) => {
        &$crate::render::olympiads::olympiad!(@year (
            $year_prefix;
            $($cb),*;
            $($years)*
        ) -> [])
    };
}

use concat_array;
use dmoj;
use ojuz;
use olympiad;

#[cfg(test)]
mod tests {
    use super::{dmoj, ojuz, *};

    const DATA: &[Year] = olympiad!(
        "FOO";
        order: dmoj, ojuz;
        2021 => {"A", no_dmoj, no_ojuz}, {"B", bbb, abc}, {"C", xxx, def};
    );

    #[test]
    fn test() {
        for y in DATA {
            println!("Year {} {}", y.year, y.name);
            for p in y.problems {
                println!("{}", p.name);
            }
        }
    }
}
