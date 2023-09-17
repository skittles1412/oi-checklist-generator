use super::{ojuz, olympiad, OlympiadProps, Year};
use crate::online_judges::OnlineJudges;

const DATA: &[Year] = olympiad!(
    "JOISC";
    order: ojuz;
    2012 => {"Broadcasting", JOI12_broadcasting}, {"Constellation", JOI12_constellation}, {"Rotate", JOI12_rotate};
    2013 => {"Collecting", JOI13_collecting}, {"Construction", JOI13_construction}, {"Mascots", JOI13_mascots}, {"Spy", JOI13_spy}, {"Cake", JOI13_cake}, {"Koala", JOI13_koala}, {"Mountain", JOI13_mountain};
    2014 => {"Bus", JOI14_bus}, {"Growing", JOI14_growing}, {"Historical", JOI14_historical}, {"Ramen", JOI14_ramen}, {"Bottle", JOI14_bottle}, {"Friends", JOI14_friends}, {"Stamps", JOI14_stamps}, {"JOIOJI", JOI14_joioji}, {"Scarecrows", JOI14_scarecrows}, {"Voltage", JOI14_voltage}, {"Constellation 2", JOI14_constellation2}, {"Kanji", JOI14_kanji}, {"Straps", JOI14_straps};
    2015 => {"Copypaste 2", JOI15_copypaste2}, {"Enjoi", JOI15_enjoi}, {"Growing 2", JOI15_growing2}, {"Ioioi Cards", JOI15_ioioi_cards}, {"Building 3", JOI15_building3}, {"Keys", JOI15_keys}, {"Road Development", JOI15_road_development}, {"Aaqqz", JOI15_aaqqz}, {"Cardgame 2", JOI15_cardgame2}, {"Navigation", JOI15_navigation}, {"Inheritance", JOI15_inheritance}, {"Memory", JOI15_memory}, {"Walls", JOI15_walls};
    2016 => {"Matryoshka", JOI16_matryoshka}, {"Memory 2", JOI16_memory2}, {"Solitaire", JOI16_solitaire}, {"Employment", JOI16_employment}, {"Sandwich", JOI16_sandwich}, {"Toilets", JOI16_toilets}, {"Dungeon 2", JOI16_dungeon2}, {"Sushi", JOI16_sushi}, {"Telegraph", JOI16_telegraph}, {"Skating", JOI16_skating}, {"Snowy", JOI16_snowy}, {"Worst Reporter 2", JOI16_worst_reporter2};
    2017 => {"Cultivation", JOI17_cultivation}, {"Port Facility", JOI17_port_facility}, {"Sparklers", JOI17_sparklers}, {"Arranging Tickets", JOI17_arranging_tickets}, {"Broken Device", JOI17_broken_device}, {"Railway Trip", JOI17_railway_trip}, {"Long Distance Coach", JOI17_coach}, {"Long Mansion", JOI17_long_mansion}, {"Park", JOI17_park}, {"Abduction 2", JOI17_abduction2}, {"City", JOI17_city}, {"Dragon 2", JOI17_dragon2};
    2018 => {"Construction of Highway", JOI18_construction}, {"Fences", JOI18_fences}, {"Tents", JOI18_tents}, {"Asceticism", JOI18_asceticism}, {"Road Service", JOI18_road_service}, {"Worst Reporter 3", JOI18_worst_reporter3}, {"Airline Route Map", JOI18_airline}, {"Bitaro", JOI18_bitaro}, {"Security Gate", JOI18_security_gate}, {"Candies", JOI18_candies}, {"Library", JOI18_library}, {"Wild Boar", JOI18_wild_boar};
    2019 => {"Examination", JOI19_examination}, {"Meetings", JOI19_meetings}, {"Naan", JOI19_naan}, {"Two Antennas", JOI19_antennas}, {"Two Dishes", JOI19_dishes}, {"Two Transportations", JOI19_transportations}, {"Designated Cities", JOI19_designated_cities}, {"Lamps", JOI19_lamps}, {"Timeleap", JOI19_timeleap}, {"Cake 3", JOI19_cake3}, {"Mergers", JOI19_mergers}, {"Minerals", JOI19_minerals};
    2020 => {"Building 4", JOI20_building4}, {"Hamburg Steak", JOI20_hamburg}, {"Sweeping", JOI20_sweeping}, {"Chameleon", JOI20_chameleon}, {"Making Friends on Joitter is Fun", JOI20_joitter2}, {"Ruins 3", JOI20_ruins3}, {"Constellation 3", JOI20_constellation3}, {"Harvest", JOI20_harvest}, {"Stray Cat", JOI20_stray}, {"Capital City", JOI20_capital_city}, {"Legendary Dango Maker", JOI20_dango2}, {"Treatment Project", JOI20_treatment};
    2021 => {"Aerobatics", JOI21_aerobatics}, {"IOI Fever", JOI21_fever}, {"Food Court", JOI21_foodcourt}, {"Escape Route", JOI21_escape_route}, {"Road Construction", JOI21_road_construction}, {"Shopping", JOI21_shopping}, {"Ancient Machine", JOI21_ancient_machine}, {"Bodyguard", JOI21_bodyguard}, {"Meetings 2", JOI21_meetings2}, {"Event Hopping 2", JOI21_event2}, {"Navigation 2", JOI21_navigation2}, {"Worst Reporter 4", JOI21_worst_reporter4};
    2022 => {"Jail", JOI22_jail}, {"Sightseeing in Kyoto", JOI22_kyoto}, {"Misspelling", JOI22_misspelling}, {"Copy and Paste 3", JOI22_copypaste3}, {"Flights", JOI22_flights}, {"Team Contest", JOI22_team}, {"Broken Device 2", JOI22_device2}, {"Sprinkler", JOI22_sprinkler}, {"Ants and Sugar", JOI22_sugar}, {"Super Dango Maker", JOI22_dango3}, {"Fish 2", JOI22_fish2}, {"Reconstruction Project", JOI22_reconstruction};
);

pub(in super::super) fn to_olympiad(ojs: &OnlineJudges) -> OlympiadProps {
    super::to_olympiad(ojs, DATA, "JOISC", |y, p, _| {
        format!("JOISC {} {}", y.year, p.name).into()
    })
}
