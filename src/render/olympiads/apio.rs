use super::{dmoj, ojuz, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "APIO";
    order: ojuz, dmoj;
    2007 => {"Mobiles", no_ojuz, apio07p1}, {"Backup", no_ojuz, apio07p2}, {"Zoo", no_ojuz, apio07p3};
    2008 => {"", no_ojuz, no_dmoj}, {"Roads", no_ojuz, apio08p2}, {"DNA", no_ojuz, apio08p3};
    2009 => {"Digging for Oil", no_ojuz, apio09p1}, {"The Siruseri Convention Centre", no_ojuz, apio09p2}, {"The Great ATM Robbery", no_ojuz, apio09p3};
    2010 => {"Commando", no_ojuz, apio10p1}, {"Patrol", no_ojuz, apio10p2}, {"Signaling", no_ojuz, apio10p3};
    2011 => {"Table Coloring", no_ojuz, apio11p1}, {"Find the Path", no_ojuz, apio11p2}, {"Guess My Word!", no_ojuz, apio11p3};
    2012 => {"Dispatching", no_ojuz, apio12p1}, {"Guard", no_ojuz, apio12p2}, {"Kunai", no_ojuz, apio12p3};
    2013 => {"Robots", APIO13_robots, apio13p1}, {"Toll", APIO13_toll, apio13p2}, {"Tasks Author", APIO13_tasksauthor, no_dmoj};
    2014 => {"Palindromes", APIO14_palindrome, apio14p1}, {"Split the Sequence", APIO14_sequence, apio14p2}, {"Beads and Wires", APIO14_beads, apio14p3};
    2015 => {"Bali Sculptures", APIO15_sculpture, apio15p1}, {"Jakarta Skyscrapers", APIO15_skyscraper, apio15p2}, {"Palembang Bridges", APIO15_bridge, apio15p3};
    2016 => {"Boat", APIO16_boat, apio16p1}, {"Fireworks", APIO16_fireworks, apio16p2}, {"Gap", APIO16_gap, no_dmoj};
    2017 => {"Land of the Rainbow Gold", APIO17_rainbow, apio17p1}, {"Travelling Merchant", APIO17_merchant, apio17p2}, {"Koala Game", APIO17_koala, no_dmoj};
    2018 => {"New Home", APIO18_new_home, apio18p1}, {"Circle Selection", APIO18_circle_selection, apio18p2}, {"Duathlon", APIO18_duathlon, apio18p3};
    2019 => {"Strange Device", APIO19_strange_device, apio19p1}, {"Bridges", APIO19_bridges, apio19p2}, {"Street Lamps", APIO19_street_lamps, apio19p3};
    2020 => {"Painting Walls", APIO20_paint, no_dmoj}, {"Swapping Cities", APIO20_swap, no_dmoj}, {"Fun Tour", APIO20_fun, no_dmoj};
    2021 => {"Hexagonal Territory", APIO21_hexagon, no_dmoj}, {"Rainforest Jumps", APIO21_jumps, no_dmoj}, {"Road Closures", APIO21_roads, no_dmoj};
    2022 => {"Mars", APIO22_mars, no_dmoj}, {"Game", APIO22_game, apio22p2}, {"Permutation", APIO22_perm, apio22p3};
    2023 => {"Cyberland", APIO23_cyberland, apio23p1}, {"Sequence", APIO23_sequence, apio23p2}, {"Alice, Bob, and Circuit", APIO23_abc, apio23p3};
    2024 => {"September", APIO24_september, no_dmoj}, {"Train", APIO24_train, no_dmoj}, {"Magic Show", APIO24_show, no_dmoj};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "APIO", |y, p, i| {
        format!("APIO {} Problem {} {}", y.year, i, p.name).into()
    })
}
