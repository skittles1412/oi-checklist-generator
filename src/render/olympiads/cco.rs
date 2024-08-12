use super::{dmoj, ojuz, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "CCO";
    order: dmoj, ojuz;
    1996 => {"Train Swapping", cco96p1, no_ojuz}, {"SafeBreaker", cco96p2, no_ojuz}, {"Quadtrees", cco96p3, no_ojuz}, {"Where's Waldorf?", cco96p4, no_ojuz}, {"All Roads Lead Where?", cco96p5, no_ojuz}, {"Hoppers", cco96p6, no_ojuz};
    1997 => {"Palindromes", cco97p1, no_ojuz}, {"Alien Invasion", cco97p2, no_ojuz}, {"Porous Stone", cco97p3, no_ojuz}, {"High Tide", cco97p4, no_ojuz}, {"Space Aliens", cco97p5, no_ojuz}, {"Aligning DNA", cco97p6, no_ojuz};
    1998 => {"Fibonacci Numbers", cco98p1, no_ojuz}, {"Message Deciphering", cco98p2, no_ojuz}, {"Bus Schedule", cco98p3, no_ojuz}, {"Scoring Curling", cco98p4, no_ojuz}, {"Text Segmentation", cco98p5, no_ojuz}, {"Derek's Dilemma", cco98p6, no_ojuz};
    1999 => {"You Can't Get There From Here", cco99p1, no_ojuz}, {"Common Words", cco99p2, no_ojuz}, {"Manelzuma's Revenge", cco99p3, no_ojuz}, {"Maple Roundup", cco99p4, no_ojuz}, {"Sum Of Products", cco99p5, no_ojuz}, {"Fast Food", cco99p6, no_ojuz};
    2000 => {"Subsets", cco00p1, no_ojuz}, {"Ransom Note", cco00p2, no_ojuz}, {"The Game Of 31", cco00p3, no_ojuz}, {"Packet Routing", cco00p4, no_ojuz}, {"Millikan's Oil Droplet Experiment", cco00p5, no_ojuz}, {"Extension Cords", cco00p6, no_ojuz};
    2001 => {"The Monkey Dance", cco01p1, no_ojuz}, {"Coke or Chocolate Milk", cco01p2, no_ojuz}, {"Partitions", cco01p3, no_ojuz}, {"", no_dmoj, no_ojuz}, {"Fast Food", cco01p5, no_ojuz}, {"Election Night", cco01p6, no_ojuz};
    2002 => {"Spam", cco02p1, no_ojuz}, {"Game Show Math", cco02p2, no_ojuz}, {"Return To Blind Man's Bluff", cco02p3, no_ojuz}, {"Duathlon", cco02p4, no_ojuz}, {"Connect The Campus", cco02p5, no_ojuz}, {"Pit Stop Strategy", cco02p6, no_ojuz};
    2003 => {"BFed", cco03p1, no_ojuz}, {"Concentration Cards", cco03p2, no_ojuz}, {"Cube", cco03p3, no_ojuz}, {"Constrained Permutations", cco03p4, no_ojuz}, {"Longest Substring", cco03p5, no_ojuz}, {"Cheap Gas", cco03p6, no_ojuz};
    2004 => {"Scribble", cco04p1, no_ojuz}, {"Hockey Scores", cco04p2, no_ojuz}, {"S and K", cco04p3, no_ojuz}, {"Return of Space Turtle", cco04p4, no_ojuz}, {"Jengaism", cco04p5, no_ojuz}, {"Orko", cco04p6, no_ojuz};
    2005 => {"Number Matrix", cco05p1, no_ojuz}, {"The Great Spamway Strike", cco05p2, no_ojuz}, {"That's News To Me", cco05p3, no_ojuz}, {"Primed Sequences", cco05p4, no_ojuz}, {"Segments", cco05p5, no_ojuz}, {"Windows", cco05p6, no_ojuz};
    2006 => {"CN Tower", cco06p1, no_ojuz}, {"R & J", cco06p2, no_ojuz}, {"Codec", cco06p3, no_ojuz}, {"CN Tower 2", cco06p4, no_ojuz}, {"Domino", cco06p5, no_ojuz}, {"Paint by Numbers", cco06p6, no_ojuz};
    2007 => {"Cows", cco07p1, no_ojuz}, {"Snowflakes", cco07p2, no_ojuz}, {"Bowling for Numbers++", cco07p3, no_ojuz}, {"Gerrymandering", cco07p4, no_ojuz}, {"Particle Catcher", cco07p5, no_ojuz}, {"Road Construction", cco07p6, no_ojuz};
    2008 => {"Moving Day", cco08p1, no_ojuz}, {"King & Weber", cco08p2, no_ojuz}, {"Mobile", cco08p3, no_ojuz}, {"Herding", cco08p4, no_ojuz}, {"Candy", cco08p5, no_ojuz}, {"Landing", cco08p6, no_ojuz};
    2009 => {"Invasion of the Boxes", cco09p1, no_ojuz}, {"Dinner", cco09p2, no_ojuz}, {"Beware the Geoducks", cco09p3, no_ojuz}, {"Bottle Caps", cco09p4, no_ojuz}, {"Parade", cco09p5, no_ojuz}, {"A Weighty Problem", cco09p6, no_ojuz};
    2010 => {"Barking Dogs!", cco10p1, no_ojuz}, {"Tree Pruning", cco10p2, no_ojuz}, {"Wowow", cco10p3, no_ojuz}, {"Computer Purchase Return", cco10p4, no_ojuz}, {"Space Miner", cco10p5, no_ojuz}, {"Shuffle", cco10p6, no_ojuz};
    2011 => {"Putnam", cco11p1, no_ojuz}, {"Vampire Tunnels", cco11p2, no_ojuz}, {"Spies Like Us", cco11p3, no_ojuz}, {"Reorganization", cco11p4, no_ojuz}, {"Fixing Disks", cco11p5, no_ojuz}, {"Biggest (Zero Carbon) Footprint", cco11p6, no_ojuz};
    2012 => {"Choose Your Own Arithmetic", cco12p1, no_ojuz}, {"The Hungary Games", cco12p2, no_ojuz}, {"Mhocskian Languages", cco12p3, no_ojuz}, {"Editor Distance", cco12p4, no_ojuz}, {"Sample Size", cco12p5, no_ojuz}, {"The Winds Of War", cco12p6, no_ojuz};
    2013 => {"All Your Base Belong to Palindromes", cco13p1, no_ojuz}, {"Tourney", cco13p2, no_ojuz}, {"LHC", cco13p3, no_ojuz}, {"A Romantic Movie Outing", cco13p4, no_ojuz}, {"Transforming Comets", cco13p5, no_ojuz}, {"Repetitivity", cco13p6, no_ojuz};
    2014 => {"Troyangles", cco14p1, no_ojuz}, {"King Gruff", cco14p2, no_ojuz}, {"Werewolf", cco14p3, no_ojuz}, {"Where's That Fuel?", cco14p4, no_ojuz}, {"Early Exam Evacuation", cco14p5, no_ojuz}, {"Gates", cco14p6, no_ojuz};
    2015 => {"Hungry Fox", cco15p1, no_ojuz}, {"Artskjid", cco15p2, no_ojuz}, {"Solar Flight", cco15p3, no_ojuz}, {"Cars on Ice", cco15p4, no_ojuz}, {"Timpanist", cco15p5, no_ojuz}, {"Eggscavation", cco15p6, no_ojuz};
    2016 => {"Field Trip", cco16p1, no_ojuz}, {"Splitting Hares", cco16p2, no_ojuz}, {"Legends", cco16p3, no_ojuz}, {"O Canada", cco16p4, no_ojuz}, {"Zombie Apocalypse", cco16p5, no_ojuz}, {"Pirates", cco16p6, no_ojuz};
    2017 => {"Vera and Trail Building", cco17p1, no_ojuz}, {"Cartesian Conquest", cco17p2, no_ojuz}, {"Vera and Modern Art", cco17p3, CCO17_art}, {"Rainfall Storage", cco17p4, no_ojuz}, {"Professional Network", cco17p5, no_ojuz}, {"Shifty Grid", cco17p6, CCO17_shifty};
    2018 => {"Geese vs. Hawks", cco18p1, no_ojuz}, {"Wrong Answer", cco18p2, no_ojuz}, {"Fun Palace", cco18p3, CCO18_fun}, {"Gradient Descent", cco18p4, no_ojuz}, {"Boring Lectures", cco18p5, no_ojuz}, {"Flop Sorting", cco18p6, no_ojuz};
    2019 => {"Human Error", cco19p1, CCO19_day1problem1}, {"Sirtet", cco19p2, CCO19_day1problem2}, {"Winter Driving", cco19p3, CCO19_day1problem3}, {"Card Scoring", cco19p4, CCO19_day2problem1}, {"Marshmallow Molecules", cco19p5, CCO19_day2problem2}, {"Bad Codes", cco19p6, CCO19_day2problem3};
    2020 => {"A Game with Grundy", cco20p1, CCO20_day1problem1}, {"Exercise Deadlines", cco20p2, CCO20_day1problem2}, {"Mountains and Valleys", cco20p3, CCO20_day1problem3}, {"Travelling Salesperson", cco20p4, CCO20_day2problem1}, {"Interval Collection", cco20p5, CCO20_day2problem2}, {"Shopping Plans", cco20p6, CCO20_day2problem3};
    2021 => {"Swap Swap Sort", cco21p1, CCO21_day1problem1}, {"Weird Numeral System", cco21p2, CCO21_day1problem2}, {"Through Another Maze Darkly", cco21p3, CCO21_day1problem3}, {"Travelling Merchant", cco21p4, CCO21_day2problem1}, {"Bread First Search", cco21p5, CCO21_day2problem2}, {"Loop Town", cco21p6, CCO21_day2problem3};
    2022 => {"Alternating Heights", cco22p1, CCO22_day1problem1}, {"Rainy Markets", cco22p2, CCO22_day1problem2}, {"Double Attendance", cco22p3, CCO22_day1problem3}, {"Bi-ing Lottery Treekets", cco22p4, CCO22_day2problem1}, {"Phone Plans", cco22p5, CCO22_day2problem2}, {"Good Game", cco22p6, CCO22_day2problem3};
    2023 => {"Binaria", cco23p1, CCO23_day1problem1}, {"Real Mountains", cco23p2, CCO23_day1problem2}, {"Line Town", cco23p3, CCO23_day1problem3}, {"Flip it and Stick it", cco23p4, CCO23_day2problem1}, {"Travelling Trader", cco23p5, CCO23_day2problem2}, {"Triangle Collection", cco23p6, CCO23_day2problem3};
    2024 => {"Treasure Hunt", cco24p1, CCO24_day1problem1}, {"Pizza Party", cco24p2, CCO24_day1problem2}, {"Summer Driving", cco24p3, CCO24_day1problem3}, {"Infiltration", cco24p4, CCO24_day2problem1}, {"Heavy Light Decomposition", cco24p5, CCO24_day2problem2}, {"Telephone Plans", cco24p6, CCO24_day2problem3};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "CCO", |y, p, i| {
        format!("CCO {} Problem {} {}", y.year, i, p.name).into()
    })
}
