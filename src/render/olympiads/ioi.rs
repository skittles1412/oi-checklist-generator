use super::{
    dmoj, ojuz, olympiad, DynLocation, OjSource, OlympiadProps, OnlineJudge, ProblemLocation,
    ToProblemLocation, Year,
};
use crate::online_judges::OnlineJudges;

struct IoiPebblingOdometer;

impl ToProblemLocation for IoiPebblingOdometer {
    fn to_problem_location(&self, ojs: &OnlineJudges) -> ProblemLocation {
        ProblemLocation {
            name: "Codeforces",
            best_score: (1..=5)
                .filter_map(|i| {
                    ojs.codeforces
                        .score_for_problem(&format!("/gym/103760/problem/A{i}"))
                })
                .reduce(|a, b| a + b),
            url: "https://ioi.contest.codeforces.com/group/32KGsXgiKA/contest/103760/problem/A1",
        }
    }
}

macro_rules! codeforces {
    (no_codeforces) => {
        [] as [DynLocation; 0]
    };
    (103760/A) => {
        [&IoiPebblingOdometer]
    };
    ($contest_id:tt/$problem_id:ident) => {
        [&OjSource {
            name: "Codeforces",
            id: concat!(
                "/gym/",
                stringify!($contest_id),
                "/problem/",
                stringify!($problem_id)
            ),
            url: concat!(
                "https://ioi.contest.codeforces.com/group/32KGsXgiKA/contest/",
                stringify!($contest_id),
                "/problem/",
                stringify!($problem_id)
            ),
            oj: OnlineJudge::Codeforces,
        }]
    };
}

const DATA: &[Year] = olympiad!(
    "IOI";
    order: ojuz, dmoj, codeforces;
    2000 => {"Palindrome", no_ojuz, ioi00p1, no_codeforces}, {"Car Parking", no_ojuz, ioi00p2, no_codeforces}, {"Median Strength", no_ojuz, no_dmoj, no_codeforces}, {"Walls", no_ojuz, ioi00p4, no_codeforces}, {"Post Office", no_ojuz, ioi00p5, no_codeforces}, {"Building with Blocks", no_ojuz, ioi00p6, no_codeforces};
    2001 => {"Mobile Phones", no_ojuz, ioi01p1, no_codeforces}, {"Ioiwari Game", no_ojuz, ioi01p2, no_codeforces}, {"Twofive", no_ojuz, ioi01p3, no_codeforces}, {"Score", no_ojuz, no_dmoj, no_codeforces}, {"Double Crypt", no_ojuz, no_dmoj, no_codeforces}, {"Depot", no_ojuz, ioi01p6, no_codeforces};
    2002 => {"The Troublesome Frog", no_ojuz, ioi02p1, 103699/A}, {"Utopia Divided", no_ojuz, ioi02p2, 103699/B}, {"XOR", no_ojuz, no_dmoj, 103699/C}, {"Batch Scheduling", no_ojuz, ioi02p4, 103700/D}, {"Bus Terminals", no_ojuz, ioi02p5, 103700/E}, {"Two Rods", no_ojuz, no_dmoj, 103700/F};
    2003 => {"Trail Maintenance", no_ojuz, no_dmoj, 103701/A}, {"Comparing Code", no_ojuz, ioi03p2, 103701/B}, {"Reverse", IOI03_reverse, no_dmoj, 103701/C}, {"Guess Which Cow", no_ojuz, no_dmoj, 103702/D}, {"Amazing Robots", no_ojuz, ioi03p5, 103702/E}, {"Seeing the Boundary", no_ojuz, ioi03p6, 103702/F};
    2004 => {"Artemis", no_ojuz, ioi04p1, 103744/A}, {"Hermes", no_ojuz, ioi04p2, 103744/B}, {"Polygon", no_ojuz, ioi04p3, 103744/C}, {"Empodia", no_ojuz, ioi04p4, 103745/D}, {"Phidias", no_ojuz, ioi04p5, 103745/E}, {"Farmer", no_ojuz, ioi04p6, 103745/F};
    2005 => {"Garden", no_ojuz, ioi05p1, 103746/A}, {"Mountain", no_ojuz, ioi05p2, 103746/B}, {"Mean Sequence", no_ojuz, ioi05p3, 103746/C}, {"Birthday", no_ojuz, ioi05p4, 103747/D}, {"Rectangle Game", no_ojuz, no_dmoj, 103747/E}, {"Rivers", no_ojuz, ioi05p6, 103747/F};
    2006 => {"Deciphering the Mayan Writing", no_ojuz, ioi06p1, 103748/A}, {"Pyramid", no_ojuz, ioi06p2, 103748/B}, {"Forbidden Subgraph", no_ojuz, no_dmoj, 103748/C}, {"The Valley of Mexico", no_ojuz, ioi06p4, 103749/D}, {"Joining Points", no_ojuz, no_dmoj, 103749/E}, {"A Black Box Game", no_ojuz, no_dmoj, 103749/F};
    2007 => {"Aliens", IOI07_aliens, ioi07p1, 103750/A}, {"Flood", IOI07_flood, ioi07p2, 103750/B}, {"Sails", IOI07_sails, ioi07p3, 103750/C}, {"Miners", IOI07_miners, ioi07p4, 103751/D}, {"Pairs", IOI07_pairs, ioi07p5, 103751/E}, {"Training", IOI07_training, ioi07p6, 103751/F};
    2008 => {"Type Printer", IOI08_printer, ioi08p1, 103752/A}, {"Fish", IOI08_fish, ioi08p2, 103752/B}, {"Islands", IOI08_islands, ioi08p3, 103752/C}, {"Teleporters", IOI08_teleporters, ioi08p4, 103753/D}, {"Linear Garden", IOI08_linear_garden, no_dmoj, 103753/E}, {"Pyramid Base", IOI08_pyramid_base, ioi08p6, 103753/F};
    2009 => {"Archery", IOI09_archery, ioi09p1, 103754/A}, {"Hiring", IOI09_hiring, ioi09p2, 103754/B}, {"Poi", IOI09_poi, ioi09p3, 103754/C}, {"Raisins", IOI09_raisins, ioi09p4, 103754/D}, {"Garage", IOI09_garage, ioi09p5, 103755/E}, {"Mecho", IOI09_mecho, ioi09p6, 103755/F}, {"Regions", IOI09_regions, ioi09p7, 103755/G}, {"Salesman", IOI09_salesman, ioi09p8, 103755/H};
    2010 => {"Cluedo", IOI10_cluedo, ioi10p1, 103756/A}, {"Hotter Colder", IOI10_hottercolder, ioi10p2, 103756/B}, {"Quality of Living", IOI10_quality, ioi10p3+io, 103756/C}, {"Languages", IOI10_languages, no_dmoj, 103756/D}, {"Memory", IOI10_memory, ioi10p5, 103757/E}, {"Traffic Congestion", IOI10_traffic, ioi10p6+io, 103757/F}, {"Maze", IOI10_maze, ioi10p7, 103757/G}, {"Saveit", IOI10_saveit, no_dmoj, 103757/H};
    2011 => {"Tropical Garden", IOI11_garden, ioi11p1+io, 103758/A}, {"Race", IOI11_race, ioi11p2+io, 103758/B}, {"Rice Hub", IOI11_ricehub, ioi11p3+io, 103758/C}, {"Crocodile's Underground City", IOI11_crocodile, ioi11p4+io, 103759/D}, {"Dancing Elephants", IOI11_elephants, ioi11p5+io, 103759/E}, {"Parrots", IOI11_parrots, ioi11p6, 103759/F};
    2012 => {"Pebbling Odometer", no_ojuz, ioi12p1, 103760/A}, {"Parachute Rings", IOI12_rings, ioi12p2+io, 103760/B}, {"Crayfish Scrivener", IOI12_scrivener, ioi12p3+io, 103760/C}, {"Ideal City", IOI12_city, ioi12p4+io, 103761/D}, {"Last Supper", IOI12_supper, ioi12p5, 103761/E}, {"Jousting Tournament", IOI12_tournament, ioi12p6+io, 103761/F};
    2013 => {"Dreaming", IOI13_dreaming, ioi13p1+io, 103762/A}, {"Art Class", IOI13_artclass, ioi13p2+io_only, 103762/B}, {"Wombats", IOI13_wombats, ioi13p3+io, 103762/C}, {"Cave", IOI13_cave, ioi13p4, 103763/D}, {"Robots", IOI13_robots, ioi13p5+io, 103763/E}, {"Game", IOI13_game, ioi13p6+io, 103763/F};
    2014 => {"Rail", IOI14_rail, ioi14p1, 103767/A}, {"Wall", IOI14_wall, ioi14p2+io, 103767/B}, {"Game", IOI14_game, ioi14p3, 103767/C}, {"Gondola", IOI14_gondola, ioi14p4+io, 103768/D}, {"Friend", IOI14_friend, ioi14p5+io, 103768/E}, {"Holiday", IOI14_holiday, ioi14p6+io, 103768/F};
    2015 => {"Boxes with Souvenirs", IOI15_boxes, ioi15p1+io, 103769/A}, {"Scales", IOI15_scales, ioi15p2, 103769/B}, {"Teams", IOI15_teams, ioi15p3+io, 103769/C}, {"Horses", IOI15_horses, ioi15p4+io, 103770/D}, {"Sorting", IOI15_sorting, ioi15p5+io, 103770/E}, {"Towns", IOI15_towns, ioi15p6, 103770/F};
    2016 => {"Detecting Molecules", IOI16_molecules, ioi16p1+io, 103772/A}, {"Roller Coaster Railroad", IOI16_railroad, ioi16p2+io, 103772/B}, {"Shortcut", IOI16_shortcut, ioi16p3+io, 103772/C}, {"Paint By Numbers", IOI16_paint, ioi16p4+io, 103773/D}, {"Unscrambling a Messy Bug", IOI16_messy, ioi16p5, 103773/E}, {"Aliens", IOI16_aliens, ioi16p6+io, 103773/F};
    2017 => {"Nowruz", IOI17_nowruz, ioi17p1, 103774/A}, {"Wiring", IOI17_wiring, ioi17p2, 103774/B}, {"Toy Train", IOI17_train, ioi17p3, 103774/C}, {"The Big Prize", IOI17_prize, ioi17p4, 103775/D}, {"Simurgh", IOI17_simurgh, ioi17p5, 103775/E}, {"Ancient Books", IOI17_books, ioi17p6, 103775/F};
    2018 => {"Combo", IOI18_combo, ioi18p1, 103776/A}, {"Seats", IOI18_seats, ioi18p2, 103776/B}, {"Werewolf", IOI18_werewolf, ioi18p3, 103776/C}, {"Mechanical Doll", IOI18_doll, ioi18p4, 103777/D}, {"Highway Tolls", IOI18_highway, ioi18p5, 103777/E}, {"Meetings", IOI18_meetings, ioi18p6, 103777/F};
    2019 => {"Arranging Shoes", IOI19_shoes, ioi19p1, 103778/A}, {"Split the Attractions", IOI19_split, ioi19p2, 103778/B}, {"Rectangles", IOI19_rect, ioi19p3, 103778/C}, {"Broken Line", IOI19_line, ioi19p4, 103779/D}, {"Vision Program", IOI19_vision, ioi19p5, 103779/E}, {"Sky Walking", IOI19_walk, ioi19p6, 103779/F};
    2020 => {"Comparing Plants", IOI20_plants, ioi20p1, 103780/A}, {"Connecting Supertrees", IOI20_supertrees, ioi20p2, 103780/B}, {"Carnival Tickets", IOI20_tickets, ioi20p3, 103780/C}, {"Packing Biscuits", IOI20_biscuits, ioi20p4, 103781/D}, {"Counting Mushrooms", IOI20_mushrooms, ioi20p5, 103781/E}, {"Stations", IOI20_stations, ioi20p6, 103781/F};
    2021 => {"Distributing Candies", IOI21_candies, ioi21p1, 103782/A}, {"Keys", IOI21_keys, ioi21p2, 103782/B}, {"Fountain Parks", IOI21_parks, ioi21p3, 103782/C}, {"Mutating DNA", IOI21_dna, ioi21p4, 103784/D}, {"Dungeons Game", IOI21_dungeons, ioi21p5, 103784/E}, {"Bit Shift Registers", IOI21_registers, ioi21p6, 103784/F};
    2022 => {"Catfish Farm", IOI22_fish, ioi22p1, 103877/A}, {"Prisoner Challenge", IOI22_prison, ioi22p2, 103877/B}, {"Radio Towers", IOI22_towers, ioi22p3, 103877/C}, {"Digital Circuit", IOI22_circuit, ioi22p4, 103880/D}, {"Rarest Insects", IOI22_insects, ioi22p5, 103880/E}, {"Thousands Islands", IOI22_islands, ioi22p6, 103880/F};
    2023 => {"Closing Time", IOI23_closing, ioi23p1, 104548/A}, {"Longest Trip", IOI23_longesttrip, ioi23p2, 104548/B}, {"Soccer Stadium", IOI23_soccer, ioi23p3, 104548/C}, {"Beech Tree", IOI23_beechtree, ioi23p4, 104552/D}, {"Overtaking", IOI23_overtaking, ioi23p5, 104552/E}, {"Robot Contest", IOI23_robot, ioi23p6, 104552/F};
    2024 => {"Nile", IOI24_nile, ioi24p1, 105328/A}, {"Message", IOI24_message, ioi24p2, 105328/B}, {"Tree", IOI24_tree, ioi24p3, 105328/C}, {"Hieroglyphs", IOI24_hieroglyphs, ioi24p4, 105330/D}, {"Mosaic", IOI24_mosaic, ioi24p5, 105330/E}, {"Sphinx's Riddle", IOI24_sphinx, ioi24p6, 105330/F};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "IOI", |y, p, i| {
        format!("IOI {} Problem {} {}", y.year, i, p.name).into()
    })
}
