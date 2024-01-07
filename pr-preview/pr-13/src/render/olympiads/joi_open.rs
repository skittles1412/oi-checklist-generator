use super::{ojuz, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "JOI Open";
    order: ojuz;
    2013 => {"Vote-Value Disparity", JOI13_disparity}, {"Synchronization", JOI13_synchronization}, {"Watching", JOI13_watching};
    2014 => {"Factories", JOI14_factories}, {"Fortune Telling 2", JOI14_fortune_telling2}, {"Space Pirate", JOI14_space_pirate}, {"Project of Migration", JOI14_migration}, {"Pinball", JOI14_pinball}, {"Secret", JOI14_secret};
    2015 => {"Colored Tiles", JOI15_colored_tiles}, {"Election Campaign", JOI15_election_campaign}, {"Sterilizing Spray", JOI15_sterilizing};
    2016 => {"JOIRIS", JOI16_joiris}, {"Selling RNA Strands", JOI16_selling_rna}, {"Skyscraper", JOI16_skyscraper};
    2017 => {"Amusement Park", JOI17_amusement_park}, {"Bulldozer", JOI17_bulldozer}, {"Golf", JOI17_golf};
    2018 => {"Bubble Sort 2", JOI18_bubblesort2}, {"Cats or Dogs", JOI18_catdog}, {"Collapse", JOI18_collapse}, {"Xylophone", JOI18_xylophone};
    2019 => {"Triple Jump", JOI19_jumps}, {"Remittance", JOI19_remittance}, {"Virus Experiment", JOI19_virus};
    2020 => {"Furniture", JOI20_furniture}, {"Monochrome Points", JOI20_monochrome}, {"Power Plant", JOI20_power};
    2021 => {"Crossing", JOI21_crossing}, {"Financial Report", JOI21_financial}, {"Monster Game", JOI21_monster};
    2022 => {"Seesaw", JOI22_seesaw}, {"Giraffes", JOI22_giraffes}, {"School Road", JOI22_schoolroad};
    2023 => {"Ancient Machine 2", JOI23_ancient2}, {"Cell Automaton", JOI23_cell}, {"Garden", JOI23_garden};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "JOI Open", |y, p, i| {
        format!("JOI Open {} Problem {} {}", y.year, i, p.name).into()
    })
}
