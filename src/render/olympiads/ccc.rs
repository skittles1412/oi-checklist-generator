use super::{dmoj, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "CCC";
    order: dmoj;
    1996 => {"Deficient, Perfect, and Abundant", ccc96s1}, {"Divisibility by 11", ccc96s2}, {"Pattern Generator", ccc96s3}, {"When in Rome", ccc96s4}, {"Maximum Distance", ccc96s5};
    1997 => {"Sentences", ccc97s1}, {"Nasty Numbers", ccc97s2}, {"Double Knockout Competition", ccc97s3}, {"Dynamic Dictionary Coding", ccc97s4}, {"Long Division", ccc97s5};
    1998 => {"Censor", ccc98s1}, {"Cross Number Puzzle", ccc98s2}, {"Mars Rover", ccc98s3}, {"Lottery", ccc98s4}, {"Mountain Passage", ccc98s5};
    1999 => {"Card Game", ccc99s1}, {"Year 2000", ccc99s2}, {"Divided Fractals", ccc99s3}, {"A Knightly Pursuit", ccc99s4}, {"Letter Arithmetic", ccc99s5};
    2000 => {"Slot Machines", ccc00s1}, {"Babbling Brooks", ccc00s2}, {"Surfing", ccc00s3}, {"Golf", ccc00s4}, {"Sheep and Coyotes", ccc00s5};
    2001 => {"Keeping Score", ccc01s1}, {"Spirals", ccc01s2}, {"Strategic Bombing", ccc01s3}, {"Cookies", ccc01s4}, {"Post's Correspondence Problem", ccc01s5};
    2002 => {"The Students' Council Breakfast", ccc02s1}, {"Fraction Action", ccc02s2}, {"Blindfold", ccc02s3}, {"Bridge Crossing", ccc02s4}, {"Follow the Bouncing Ball", ccc02s5};
    2003 => {"Snakes and Ladders", ccc03s1}, {"Poetry", ccc03s2}, {"Floor Plan", ccc03s3}, {"Substrings", ccc03s4}, {"Trucking Troubles", ccc03s5};
    2004 => {"Fix", ccc04s1}, {"TopYodeller", ccc04s2}, {"Spreadsheet", ccc04s3}, {"Space Turtle", ccc04s4}, {"Super Plumber", ccc04s5};
    2005 => {"Snow Calls", ccc05s1}, {"Mouse Move", ccc05s2}, {"Quantum Operations", ccc05s3}, {"Pyramid Message Scheme", ccc05s4}, {"Pinball Ranking", ccc05s5};
    2006 => {"Maternity", ccc06s1}, {"Attack of the CipherTexts", ccc06s2}, {"Tin Can Telephone", ccc06s3}, {"Groups", ccc06s4}, {"Origin of Life", ccc06s5};
    2007 => {"Federal Voting Age", ccc07s1}, {"Boxes", ccc07s2}, {"Friends", ccc07s3}, {"Waterpark", ccc07s4}, {"Bowling for Numbers", ccc07s5};
    2008 => {"It's Cold Here!", ccc08s1}, {"Pennies in the Ring", ccc08s2}, {"Maze", ccc08s3}, {"Twenty-four", ccc08s4}, {"Nukit", ccc08s5};
    2009 => {"Cool Numbers", ccc09s1}, {"Lights Going On and Off", ccc09s2}, {"Degrees Of Separation", ccc09s3}, {"Shop and Ship", ccc09s4}, {"Wireless", ccc09s5};
    2010 => {"Computer Purchase", ccc10s1}, {"Huffman Encoding", ccc10s2}, {"Firehose", ccc10s3}, {"Animal Farm", ccc10s4}, {"Nutrient Tree", ccc10s5};
    2011 => {"English or French?", ccc11s1}, {"Multiple Choice", ccc11s2}, {"Alice Through the Looking Glass", ccc11s3}, {"Blood Distribution", ccc11s4}, {"Switch", ccc11s5};
    2012 => {"Don't pass me the ball!", ccc12s1}, {"Aromatic Numbers", ccc12s2}, {"Absolutely Acidic", ccc12s3}, {"A Coin Game", ccc12s4}, {"Mouse Journey", ccc12s5};
    2013 => {"From 1987 to 2013", ccc13s1}, {"Bridge Transport", ccc13s2}, {"Chances of Winning", ccc13s3}, {"Who is Taller?", ccc13s4}, {"Factor Solitaire", ccc13s5};
    2014 => {"Party Invitation", ccc14s1}, {"Assigning Partners", ccc14s2}, {"The Geneva Confection", ccc14s3}, {"Tinted Glass Window", ccc14s4}, {"Lazy Fox", ccc14s5};
    2015 => {"Zero That Out", ccc15s1}, {"Jerseys", ccc15s2}, {"Gates", ccc15s3}, {"Convex Hull", ccc15s4}, {"Greedy For Pies", ccc15s5};
    2016 => {"Ragaman", ccc16s1}, {"Tandem Bicycle", ccc16s2}, {"Phonomenal Reviews", ccc16s3}, {"Combining Riceballs", ccc16s4}, {"Circle of Life", ccc16s5};
    2017 => {"Sum Game", ccc17s1}, {"High Tide, Low Tide", ccc17s2}, {"Nailed It!", ccc17s3}, {"Minimum Cost Flow", ccc17s4}, {"RMT", ccc17s5};
    2018 => {"Voronoi Villages", ccc18s1}, {"Sunflowers", ccc18s2}, {"RoboThieves", ccc18s3}, {"Balanced Trees", ccc18s4}, {"Maximum Strategic Savings", ccc18s5};
    2019 => {"Flipper", ccc19s1}, {"Pretty Average Primes", ccc19s2}, {"Arithmetic Square", ccc19s3}, {"Tourism", ccc19s4}, {"Triangle: The Data Structure", ccc19s5};
    2020 => {"Surmising a Sprinter's Speed", ccc20s1}, {"Escape Room", ccc20s2}, {"Searching for Strings", ccc20s3}, {"Swapping Seats", ccc20s4}, {"Josh's Double Bacon Deluxe", ccc20s5};
    2021 => {"Crazy Fencing", ccc21s1}, {"Modern Art", ccc21s2}, {"Lunch Concert", ccc21s3}, {"Daily Commute", ccc21s4}, {"Math Homework", ccc21s5};
    2022 => {"Good Fours and Good Fives", ccc22s1}, {"Good Groups", ccc22s2}, {"Good Samples", ccc22s3}, {"Good Triplets", ccc22s4}, {"Good Influencers", ccc22s5};
    2023 => {"Trianglane", ccc23s1}, {"Symmetric Mountains", ccc23s2}, {"Palindromic Poster", ccc23s3}, {"Minimum Cost Roads", ccc23s4}, {"The Filter", ccc23s5};
    2024 => {"Hat Circle", ccc24s1}, {"Heavy-Light Composition", ccc24s2}, {"Swipe", ccc24s3}, {"Painting Roads", ccc24s4}, {"Chocolate Bar Partition", ccc24s5};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "CCC Senior", |y, p, i| {
        format!("CCC {} Senior {} {}", y.year, i, p.name).into()
    })
}
