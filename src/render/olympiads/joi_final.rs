use super::{ojuz, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "JOI Final";
    order: ojuz;
    2017 => {"Foehn Phenomena", JOI17_foehn_phenomena}, {"Semiexpress", JOI17_semiexpress}, {"The Kingdom of JOIOI", JOI17_joioi}, {"Soccer", JOI17_soccer}, {"Rope", JOI17_rope};
    2018 => {"Stove", JOI18_stove}, {"Art Exhibition", JOI18_art}, {"Dango Maker", JOI18_dango_maker}, {"Commuter Pass", JOI18_commuter_pass}, {"Snake Escaping", JOI18_snake_escaping};
    2019 => {"Bitaro the Brave", JOI19_ho_t1}, {"Exhibition", JOI19_ho_t2}, {"Growing Vegetable is Fun 3", JOI19_ho_t3}, {"Coin Collecting", JOI19_ho_t4}, {"Unique Cities", JOI19_ho_t5};
    2020 => {"Just Long Neckties", JOI20_ho_t1}, {"JJOOII 2", JOI20_ho_t2}, {"Collecting Stamps 3", JOI20_ho_t3}, {"Olympic Bus", JOI20_ho_t4}, {"Fire", JOI20_ho_t5};
    2021 => {"Growing Vegetables is Fun 4", JOI21_ho_t1}, {"Snowball", JOI21_ho_t2}, {"Group Photo", JOI21_ho_t3}, {"Robot", JOI21_ho_t4}, {"Dungeon 3", JOI21_ho_t5};
    2022 => {"Intercastellar", JOI22_ho_t1}, {"Self Study", JOI22_ho_t2}, {"Let's Win the Election", JOI22_ho_t3}, {"Railway Trip 2", JOI22_ho_t4}, {"Sandcastle 2", JOI22_ho_t5};
    2023 => {"Stone Arranging 2", JOI23_ho_t1}, {"Advertisement 2", JOI23_ho_t2}, {"Maze", JOI23_ho_t3}, {"Cat Exercise", JOI23_ho_t4}, {"Modern Machine", JOI23_ho_t5};
    2024 => {"Room Temperature", JOI24_ho_t1}, {"Construction Project 2", JOI24_ho_t2}, {"Marathon Race 2", JOI24_ho_t3}, {"Gift Exchange", JOI24_ho_t4}, {"Road Service 2", JOI24_ho_t5};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "JOI Final", |y, p, i| {
        format!("JOI Final {} Problem {} {}", y.year, i, p.name).into()
    })
}
