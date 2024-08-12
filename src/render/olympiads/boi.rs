use super::{dmoj, olympiad, OlympiadProps, ToProblemLocation, Year};
use crate::{online_judges::OnlineJudges, render::ProblemLocation};

struct BoiNecklace;

impl ToProblemLocation for BoiNecklace {
    fn to_problem_location(&self, ojs: &OnlineJudges) -> ProblemLocation {
        ProblemLocation {
            name: "ojuz",
            best_score: match (
                ojs.ojuz
                    .score_for_problem("BOI19_necklace1")
                    .map(|score| score / 100. * 85.),
                ojs.ojuz.score_for_problem("BOI19_necklace4"),
            ) {
                (Some(s1), Some(s2)) => Some(s1.max(s2)),
                (Some(s1), None) => Some(s1),
                (None, Some(s2)) => Some(s2),
                (None, None) => None,
            },
            url: "https://oj.uz/problem/view/BOI19_necklace1",
        }
    }
}

macro_rules! ojuz {
    (BOI19_necklace) => {
        [&BoiNecklace]
    };
    ($ojuz:ident) => {
        super::ojuz!($ojuz)
    };
}

const DATA: &[Year] = olympiad!(
    "BOI";
    order: ojuz, dmoj;
    2001 => {"Postman", no_ojuz, btoi01p1}, {"Crack the Code", no_ojuz, btoi01p2}, {"Box of Mirrors", no_ojuz, btoi01p3}, {"Knights", no_ojuz, btoi01p4}, {"Mars Maps", no_ojuz, btoi01p5}, {"Teleports", no_ojuz, btoi01p6};
    2002 => {"Speed Limits", no_ojuz, btoi02p1}, {"Tennis Club", no_ojuz, btoi02p2}, {"Triangles", no_ojuz, btoi02p3}, {"Bicriterial Routing", no_ojuz, btoi02p4};
    2003 => {"Barrel", no_ojuz, btoi03p1}, {"Gems", no_ojuz, btoi03p2}, {"Table", no_ojuz, btoi03p3}, {"The Gangs", no_ojuz, btoi03p4}, {"Lamps", no_ojuz, btoi03p5};
    2004 => {"Ships", no_ojuz, btoi04p1}, {"Scales", no_ojuz, btoi04p2}, {"Sequence", no_ojuz, btoi04p3}, {"Repeats", no_ojuz, btoi04p4}, {"Rectangles", no_ojuz, btoi04p5}, {"Car Park", no_ojuz, btoi04p6};
    2005 => {"Camouflaged camp", no_ojuz, btoi05p1}, {"LISP", no_ojuz, btoi05p2}, {"Maze", no_ojuz, btoi05p3}, {"Ancient Manuscript", no_ojuz, btoi05p4}, {"Bus Trip", no_ojuz, btoi05p5}, {"Polygon", no_ojuz, btoi05p6};
    2006 => {"Bitwise", BOI06_bitwise, btoi06p1}, {"Coins", BOI06_coins, btoi06p2}, {"Countries", BOI06_countries, no_dmoj}, {"City", BOI06_city, no_dmoj}, {"RLE", BOI06_rle, btoi06p5}, {"Jump", BOI06_jump, btoi06p6};
    2007 => {"Escape", no_ojuz, btoi07p1}, {"Sorting", no_ojuz, btoi07p2}, {"Sound", no_ojuz, btoi07p3}, {"Building a Fence", no_ojuz, btoi07p4}, {"Connected Points", no_ojuz, btoi07p5}, {"Sequence", no_ojuz, btoi07p6};
    2008 => {"Game", no_ojuz, btoi08p1}, {"Gates", no_ojuz, btoi08p2}, {"Magical Stones", no_ojuz, btoi08p3}, {"Elections", no_ojuz, btoi08p4}, {"Grid", no_ojuz, btoi08p5}, {"Gloves", no_ojuz, btoi08p6};
    2009 => {"Beetle", no_ojuz, btoi09p1}, {"Candy Machine", no_ojuz, btoi09p2}, {"Subway Signaling Error", no_ojuz, btoi09p3}, {"Rectangle", no_ojuz, btoi09p4}, {"Triangulation", no_ojuz, btoi09p5}, {"Monument", no_ojuz, btoi09p6};
    2010 => {"BEARs", no_ojuz, btoi10p1}, {"Lego", no_ojuz, btoi10p2}, {"Printed Circuit Board", no_ojuz, btoi10p3}, {"Matching Bins", no_ojuz, btoi10p4}, {"Candies", no_ojuz, btoi10p5}, {"Mines", no_ojuz, btoi10p6};
    2011 => {"Growing Trees", BOI11_grow, btoi11p1}, {"Icecream", BOI11_icecream, btoi11p2}, {"Lamp", BOI11_lamp, btoi11p3}, {"Vikings", BOI11_vikings, btoi11p4}, {"Meetings", no_ojuz, btoi11p5}, {"Plagiarism", no_ojuz, btoi11p6}, {"Polygon", no_ojuz, btoi11p7}, {"Tree Mirroring", no_ojuz, btoi11p8};
    2012 => {"Brackets", no_ojuz, btoi12p1}, {"Mobile", no_ojuz, btoi12p2}, {"Peaks", no_ojuz, btoi12p3}, {"Fireworks in RightAngleles", no_ojuz, btoi12p4}, {"Melody", no_ojuz, btoi12p5}, {"Tiny", no_ojuz, btoi12p6};
    2013 => {"Ball Machine", BOI13_ballmachine, btoi13p1}, {"Palindrome-Free Numbers", BOI13_numbers, btoi13p2}, {"Pipes", BOI13_pipes, btoi13p3}, {"Brunhilda", BOI13_brunhilda, btoi13p4}, {"Tracks in the Snow", BOI13_tracks, btoi13p5}, {"Vim", BOI13_vim, btoi13p6};
    2014 => {"Cop and Robber", BOI14_coprobber, no_dmoj}, {"Three Friends", BOI14_friends, btoi14p2}, {"Sequence", BOI14_sequence, btoi14p3}, {"Demarcation", BOI14_demarcation, btoi14p4}, {"Portals", BOI14_portals, btoi14p5}, {"Senior Postmen", BOI14_postmen, btoi14p6};
    2015 => {"Bowling", BOI15_bow, btoi15p1}, {"Editor", BOI15_edi, btoi15p2}, {"Network", BOI15_net, btoi15p3}, {"File Paths", BOI15_fil, btoi15p4}, {"Hacker", BOI15_hac, btoi15p5}, {"Tug of War", BOI15_tug, btoi15p6};
    2016 => {"Bosses", BOI16_bosses, btoi16p1}, {"Park", BOI16_park, btoi16p2}, {"Spiral", BOI16_spiral, btoi16p3}, {"Cities", BOI16_cities, btoi16p4}, {"Maze", BOI16_maze, btoi16p5}, {"Swap", BOI16_swap, btoi16p6};
    2017 => {"Political Development", BOI17_politicaldevelopment, btoi17p1}, {"Toll", BOI17_toll, btoi17p2}, {"Railway", BOI17_railway, btoi17p3}, {"Friends", BOI17_friends, btoi17p4}, {"Plus Minus", BOI17_plusminus, btoi17p5}, {"Cat in a tree", BOI17_catinatree, btoi17p6};
    2018 => {"Love Polygon", BOI18_polygon, btoi18p1}, {"Martian DNA", BOI18_dna, btoi18p2}, {"Worm Worries", BOI18_worm, btoi18p3}, {"Alternating Current", BOI18_alternating, btoi18p4}, {"Genetics", BOI18_genetics, btoi18p5}, {"Paths", BOI18_paths, btoi18p6};
    2019 => {"Flash", BOI19_flash, no_dmoj}, {"Nautilus", BOI19_nautilus, btoi19p2}, {"Valley", BOI19_valley, btoi19p3}, {"Kitchen", BOI19_kitchen, btoi19p4}, {"Necklace", BOI19_necklace, btoi19p5}, {"Olympiads", BOI19_olympiads, btoi19p6};
    2020 => {"Colors", BOI20_colors, no_dmoj}, {"Mixture", BOI20_mixture, no_dmoj}, {"Joker", BOI20_joker, btoi20p3}, {"Graph", BOI20_graph, no_dmoj}, {"Village", BOI20_village, no_dmoj}, {"Viruses", BOI20_viruses, no_dmoj};
    2021 => {"A Difficult(y) Choice", BOI21_books, no_dmoj}, {"Inside information", BOI21_servers, no_dmoj}, {"From Hacks to Snitches", BOI21_watchmen, no_dmoj}, {"The short shank; Redemption", BOI21_prison, no_dmoj}, {"The Collection Game", BOI21_swaps, no_dmoj}, {"The Xana coup", BOI21_xanadu, no_dmoj};
    2022 => {"Art Collections", BOI22_art, no_dmoj}, {"Event Hopping", BOI22_events, no_dmoj}, {"Uplifting Excursion", BOI22_vault, no_dmoj}, {"Flight to the Ford", BOI22_communication, no_dmoj}, {"Stranded Far From Home", BOI22_island, no_dmoj}, {"Boarding Passes", BOI22_passes, no_dmoj};
    2024 => {"Jobs", BOI24_jobs, no_dmoj}, {"Portal", BOI24_portal, no_dmoj}, {"Trains", BOI24_trains, no_dmoj}, {"Fire", BOI24_fire, no_dmoj}, {"Tiles", BOI24_tiles, no_dmoj}, {"Flooding Wall", BOI24_wall, no_dmoj};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "BOI", |y, p, i| {
        format!("BOI {} Problem {} {}", y.year, i, p.name).into()
    })
}
