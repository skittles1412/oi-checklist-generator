use super::{dmoj, ojuz, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "CEOI";
    order: ojuz, dmoj;
    2006 => {"Antenna", CEOI06_antenna, no_dmoj}, {"Queue", CEOI06_queue, no_dmoj}, {"Walk", CEOI06_walk, no_dmoj}, {"Connect", CEOI06_connect, no_dmoj}, {"Link", CEOI06_link, no_dmoj}, {"Meandian", CEOI06_meandian, no_dmoj};
    2008 => {"Dominance", CEOI08_dominance, no_dmoj}, {"Knights", CEOI08_knights, no_dmoj}, {"Information", CEOI08_information, no_dmoj}, {"Snake", CEOI08_snake, no_dmoj}, {"Order", CEOI08_order, no_dmoj}, {"Fence", CEOI08_fence, no_dmoj};
    2009 => {"Boxes", CEOI09_boxes, no_dmoj}, {"Harbingers", CEOI09_harbingers, ceoi09p2}, {"Photo", CEOI09_photo, no_dmoj}, {"Logs", CEOI09_logs, no_dmoj}, {"Sorting", CEOI09_sorting, no_dmoj}, {"Tri", CEOI09_tri, no_dmoj};
    2010 => {"Alliances", CEOI10_alliances, no_dmoj}, {"Arithmetic Rectangle", CEOI10_arithmetic, no_dmoj}, {"Bodyguards", CEOI10_bodyguards, no_dmoj}, {"MP3 Player", CEOI10_mp3player, no_dmoj}, {"PIN", CEOI10_pin, no_dmoj}, {"A Huge Tower", CEOI10_tower, no_dmoj};
    2011 => {"Balloons", CEOI11_bal, no_dmoj}, {"Matching", CEOI11_mat, no_dmoj}, {"Treasure Hunt", CEOI11_tre, no_dmoj}, {"Hotel", CEOI11_hot, no_dmoj}, {"Teams", CEOI11_tea, no_dmoj}, {"Traffic", CEOI11_tra, no_dmoj};
    2012 => {"Job Scheduling", CEOI12_jobs, no_dmoj}, {"Printed Circuit Board", CEOI12_circuit, no_dmoj}, {"Sailing Race", CEOI12_race, no_dmoj}, {"Highway design", CEOI12_highway, no_dmoj};
    2013 => {"Treasure (different grader from official contest)", CEOI13_treasure2, no_dmoj}, {"Tram", CEOI13_tram, no_dmoj}, {"Splot", CEOI13_splot, no_dmoj}, {"Board", CEOI13_board, no_dmoj}, {"Adriatic", CEOI13_adriatic, no_dmoj}, {"Watering", CEOI13_watering, no_dmoj};
    2014 => {"Carnival", CEOI14_carnival, no_dmoj}, {"The Forest of Fangorn", CEOI14_fangorn, ceoi14p2}, {"Question (Grader is different from the original contest)", CEOI14_question_grader, no_dmoj}, {"007", CEOI14_007, no_dmoj}, {"Cake", CEOI14_cake, no_dmoj}, {"Wall", CEOI14_wall, no_dmoj};
    2015 => {"Potemkin cycle", CEOI15_indcyc, no_dmoj}, {"Calvinball championship", CEOI15_teams, no_dmoj}, {"Pipes", CEOI15_pipes, no_dmoj}, {"Ice Hockey World Championship", CEOI15_bobek, no_dmoj}, {"Nuclearia", CEOI15_nuclearia, no_dmoj}, {"Calvinball championship, again", CEOI15_calvinball, no_dmoj};
    2016 => {"ICC", CEOI16_icc, no_dmoj}, {"Kangaroo", CEOI16_kangaroo, ceoi16p2}, {"Trick", CEOI16_trick, no_dmoj}, {"Match", CEOI16_match, ceoi16p4}, {"Popeala", CEOI16_popeala, ceoi16p5}, {"Router", CEOI16_router, no_dmoj};
    2017 => {"One-Way Streets", CEOI17_oneway, ceoi17p1}, {"Sure Bet", CEOI17_sure, ceoi17p2}, {"Mousetrap", CEOI17_mousetrap, ceoi17p3}, {"Building Bridges", CEOI17_building, ceoi17p4}, {"Palindromic Partitions", CEOI17_palindromic, ceoi17p5}, {"Chase", CEOI17_chase, ceoi17p6};
    2018 => {"Cloud Computing", CEOI18_clo, ceoi18p1}, {"Global Warming", CEOI18_glo, ceoi18p2}, {"Lottery", CEOI18_lot, ceoi18p3}, {"Fibonacci Representations", CEOI18_fib, ceoi18p4}, {"Toys", CEOI18_toy, ceoi18p5}, {"Triangles", CEOI18_tri, no_dmoj};
    2019 => {"Building Skyscrapers", CEOI19_skyscrapers, ceoi19p1}, {"Dynamic Diameter", CEOI19_diameter, ceoi19p2}, {"Cubeword", CEOI19_cubeword, ceoi19p3}, {"Amusement Park", CEOI19_amusementpark, ceoi19p4}, {"Magic Tree", CEOI19_magictree, ceoi19p5}, {"Scissors and Tape", CEOI19_scissors, no_dmoj};
    2020 => {"Fancy Fence", CEOI20_fancyfence, no_dmoj}, {"Roads", CEOI20_roads, no_dmoj}, {"Star Trek", CEOI20_startrek, no_dmoj}, {"The Potion of Great Power", CEOI20_potion, no_dmoj}, {"Spring cleaning", CEOI20_cleaning, no_dmoj}, {"Chess Rush", CEOI20_chessrush, no_dmoj};
    2021 => {"Diversity", CEOI21_diversity, no_dmoj}, {"L-triominoes", CEOI21_ltriominoes, no_dmoj}, {"Newspapers", CEOI21_newspapers, no_dmoj}, {"Stones", CEOI21_stones, no_dmoj}, {"Tortoise", CEOI21_tortoise, no_dmoj}, {"Wells", CEOI21_wells, no_dmoj};
    2022 => {"Abracadabra", CEOI22_abracadabra, ceoi22p1}, {"Homework", CEOI22_homework, ceoi22p2}, {"Prize", CEOI22_prize, ceoi22p3}, {"Drawing", CEOI22_drawing, ceoi22p4}, {"Measures", CEOI22_measures, ceoi22p5}, {"Parking", CEOI22_parking, ceoi22p6};
    2023 => {"A Light Inconvenience", CEOI23_light, no_dmoj}, {"Bring Down the Grading Server", CEOI23_gradingserver, no_dmoj}, {"Brought Down the Grading Server?", CEOI23_balance, no_dmoj}, {"Tricks of the Trade", CEOI23_trade, no_dmoj}, {"The Ties That Guide Us", CEOI23_incursion, no_dmoj}, {"How to Avoid Disqualification in 75 Easy Steps", CEOI23_avoid, no_dmoj};
    2024 => {"Naval battle", CEOI24_battle, no_dmoj}, {"COVID tests", CEOI24_covid, no_dmoj}, {"Text editor", CEOI24_editor, no_dmoj}, {"Toy", CEOI24_toy, no_dmoj}, {"Petrol stations", CEOI24_stations, no_dmoj}, {"Sprinklers", CEOI24_sprinklers, no_dmoj};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "CEOI", |y, p, i| {
        format!("CEOI {} Problem {} {}", y.year, i, p.name).into()
    })
}
