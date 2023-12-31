use super::{dmoj, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "CNOI";
    order: dmoj;
    1997 => {"Competition Ranking", noi97p1}, {"Optimal Routing", noi97p2}, {"File Matching", noi97p3}, {"Perfect Tour", noi97p4}, {"Building Game", noi97p5}, {"Satellite Coverage", noi97p6};
    1998 => {"Personal Income Tax", noi98p1}, {"Free Pizza", noi98p2}, {"Software Installation Disk", noi98p3}, {"Scarf Cutting", noi98p4}, {"SERNET Simulation", noi98p5}, {"Parallel Computing", noi98p6};
    1999 => {"01 Sequence", noi99p1}, {"Nails and Ball", noi99p2}, {"Birthday Cake", noi99p3}, {"Chessboard Division", noi99p4}, {"Optimally Connected Subset", noi99p5}, {"Memory Allocation", noi99p6};
    2000 => {"Ceramic Necklace", noi00p1}, {"Program Parser", noi00p2}, {"Mystery of the Lost City", noi00p3}, {"Trie", noi00p4}, {"Frogs Crossing the River", noi00p5}, {"Symbol Deciphering", noi00p6};
    2001 => {"Food Chain", noi01p1}, {"Artillery Positioning", noi01p2}, {"The Clever Typist", noi01p3}, {"Applications of Arctangent", noi01p4}, {"Equation Solutions", noi01p5}, {"Secret of the Meteorite", noi01p6};
    2002 => {"Legend of the Galactic Heroes", noi02p1}, {"Naughty Kid", noi02p2}, {"The Greedy Kuzuryū", noi02p3}, {"Island of Cavemen", noi02p4}, {"New Tetris", noi02p5}, {"Robot No. M", noi02p6};
    2003 => {"Stick Game", noi03p1}, {"Text Editor", noi03p2}, {"", no_dmoj}, {"Data Generator", noi03p4}, {"", no_dmoj}, {"Wisdom Breaking Connection", noi03p6};
    2004 => {"The Depressed Cashier", noi04p1}, {"", no_dmoj}, {"Manhattan", noi04p3}, {"Rainfall", noi04p4}, {"Little H's Little Hut", noi04p5}, {"", no_dmoj};
    2005 => {"The Magnificent Waltz", noi05p1}, {"Maintaining a Sequence", noi05p2}, {"Wisdom Beads Game", noi05p3}, {"CongCong and KoKo", noi05p4}, {"Little H's Party", noi05p5}, {"Lemon Tree Under the Moon", noi05p6};
    2006 => {"Network Charges", noi06p1}, {"", no_dmoj}, {"Millennium Worm", noi06p3}, {"Maximum Profit", noi06p4}, {"The Clever Tour Guide", noi06p5}, {"The Magical Bag", noi06p6};
    2007 => {"Social Network", noi07p1}, {"Cash Exchange", noi07p2}, {"Surrounding Battalions", noi07p3}, {"Necklace Factory", noi07p4}, {"Counting Spanning Trees", noi07p5}, {"Thief Catching", noi07p6};
    2008 => {"Masquerade Party", noi08p1}, {"Course Design", noi08p2}, {"Hiring Employees", noi08p3}, {"Olympic Logistics", noi08p4}, {"Candy Rain", noi08p5}, {"Tournament Matching", noi08p6};
    2009 => {"Transformed Sequence", noi09p1}, {"Little G the Poet", noi09p2}, {"Modified Treap", noi09p3}, {"Plants vs. Zombies", noi09p4}, {"Pipe Marbles", noi09p5}, {"Tracing", noi09p6};
    2010 => {"Energy Harvesting", noi10p1}, {"Super Piano", noi10p2}, {"Altitude", noi10p3}, {"Flight Control", noi10p4}, {"Route Planning", noi10p5}, {"Happily Growing", noi10p6};
    2011 => {"Rabbit Farming", noi11p1}, {"Intelligent Car Racing", noi11p2}, {"Ali's Typewriter", noi11p3}, {"Road Construction", noi11p4}, {"NOI Carnival", noi11p5}, {"Bunny and Eggy's Game", noi11p6};
    2012 => {"Random Number Generator", noi12p1}, {"Highway Cycling", noi12p2}, {"Magic Chessboard", noi12p3}, {"Lost in the Park", noi12p4}, {"Food Festival", noi12p5}, {"Triple Town", noi12p6};
    2013 => {"Inner Product", noi13p1}, {"Tree Count", noi13p2}, {"Little Q's Training", noi13p3}, {"Matrix Game", noi13p4}, {"Calligrapher", noi13p5}, {"Fast Food Restaurant", noi13p6};
    2014 => {"Getting-Up Syndrome", noi14p1}, {"Enchanted Forest", noi14p2}, {"Deletion Game", noi14p3}, {"Zoo", noi14p4}, {"Random Number Generator", noi14p5}, {"Ticket Purchase", noi14p6};
    2015 => {"Automated Program Analyzer", noi15p1}, {"Software Package Manager", noi15p2}, {"Sushi Dinner", noi15p3}, {"Homeric Epics", noi15p4}, {"Cocktail Party", noi15p5}, {"Farm", noi15p6};
    2016 => {"Good Partitions", noi16p1}, {"Grid", noi16p2}, {"The Beauty of Cycles", noi16p3}, {"Interval", noi16p4}, {"Drinking Water", noi16p5}, {"Computation", noi16p6};
    2017 => {"Integers", noi17p1}, {"Queue", noi17p2}, {"Pool", noi17p3}, {"", no_dmoj}, {"Vegetables", noi17p5}, {"Magic", noi17p6};
    2018 => {"", no_dmoj}, {"", no_dmoj}, {"", no_dmoj}, {"", no_dmoj}, {"", no_dmoj}, {"", no_dmoj};
    2019 => {"Route", noi19p1}, {"Robot", noi19p2}, {"Sequence", noi19p3}, {"Jump", noi19p4}, {"Landlords", noi19p5}, {"Explore", noi19p6};
    2020 => {"Delicacy", noi20p1}, {"Destiny", noi20p2}, {"Tears", noi20p3}, {"Dish", noi20p4}, {"Surreal", noi20p5}, {"Road", noi20p6};
    2021 => {"Light Heavy Edges", noi21p1}, {"Intersecting Paths", noi21p2}, {"Celebration", noi21p3}, {"Quantum Communication", noi21p4}, {"The Locked Box", noi21p5}, {"Robot Game", noi21p6};
    2022 => {"Major", noi22p1}, {"Stone", noi22p2}, {"Count", noi22p3}, {"Challenge NPC", noi22p4}, {"Bubble Sort", noi22p5}, {"Quadratic Integer Program", noi22p6};
    2023 => {"Square Coloring", noi23p1}, {"Osmanthus Tree", noi23p2}, {"Depth First Search", noi23p3}, {"Trade", noi23p4}, {"String", noi23p5}, {"Merge the Books", noi23p6};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "CNOI", |y, p, i| {
        format!("CNOI {} Problem {} {}", y.year, i, p.name).into()
    })
}
