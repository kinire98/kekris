use serde::{Deserialize, Serialize};

use super::game_info::{GameInfo, GameTypeInfo};

#[derive(Debug, Serialize, Deserialize)]
pub struct EmitGameInfo {
    last_game_info: GameInfo,
    top_five_results: Vec<GameInfo>,
    last_in_top_five: i64,
}

impl EmitGameInfo {
    pub fn new(last_game_info: GameInfo, previous_results: Vec<GameInfo>) -> Self {
        let top_five = match previous_results.first() {
            Some(first) => match first.type_of_info() {
                super::game_info::GameTypeInfo::Classic(_classic_game_info) => {
                    let GameTypeInfo::Classic(_info) = last_game_info.type_of_info() else {
                        panic!("GameInfo type must be equal to the one of previous results");
                    };
                    Self::top_five_classic(previous_results)
                }
                super::game_info::GameTypeInfo::Lines(_lines_game_info) => {
                    let GameTypeInfo::Lines(_info) = last_game_info.type_of_info() else {
                        panic!("GameInfo type must be equal to the one of previous results");
                    };
                    Self::top_five_lines(previous_results)
                }
                super::game_info::GameTypeInfo::Blitz(_blitz_game_info) => {
                    let GameTypeInfo::Blitz(_info) = last_game_info.type_of_info() else {
                        panic!("GameInfo type must be equal to the one of previous results");
                    };
                    Self::top_five_blitz(previous_results)
                }
            },
            None => vec![],
        };
        EmitGameInfo {
            last_game_info,
            last_in_top_five: Self::in_top_five(last_game_info, top_five.as_ref()),
            top_five_results: top_five,
        }
    }
    fn top_five_classic(mut previous_results: Vec<GameInfo>) -> Vec<GameInfo> {
        previous_results.sort_by(|a, b| match (a.type_of_info(), b.type_of_info()) {
            (
                super::game_info::GameTypeInfo::Classic(classic_game_info_a),
                super::game_info::GameTypeInfo::Classic(classic_game_info_b),
            ) => classic_game_info_a
                .points()
                .cmp(&classic_game_info_b.points()),
            _ => panic!("Invalid state"),
        });
        previous_results
            .into_iter()
            .take(5)
            .collect::<Vec<GameInfo>>()
    }
    fn top_five_lines(mut previous_results: Vec<GameInfo>) -> Vec<GameInfo> {
        previous_results.sort_by(|a, b| match (a.type_of_info(), b.type_of_info()) {
            (
                super::game_info::GameTypeInfo::Lines(lines_game_info_a),
                super::game_info::GameTypeInfo::Lines(lines_game_info_b),
            ) => lines_game_info_a
                .time_endured()
                .cmp(&lines_game_info_b.time_endured()),
            _ => panic!("Invalid state"),
        });
        previous_results
            .into_iter()
            .take(5)
            .collect::<Vec<GameInfo>>()
    }
    fn top_five_blitz(mut previous_results: Vec<GameInfo>) -> Vec<GameInfo> {
        previous_results.sort_by(|a, b| match (a.type_of_info(), b.type_of_info()) {
            (
                super::game_info::GameTypeInfo::Blitz(blitz_game_info_a),
                super::game_info::GameTypeInfo::Blitz(blitz_game_info_b),
            ) => blitz_game_info_a.points().cmp(&blitz_game_info_b.points()),
            _ => panic!("Invalid state"),
        });
        previous_results
            .into_iter()
            .take(5)
            .collect::<Vec<GameInfo>>()
    }
    fn in_top_five(info: GameInfo, top_five: &[GameInfo]) -> i64 {
        // Not using Option, because it will be sent to JS
        for (i, el) in top_five.iter().enumerate() {
            if &info == el {
                return i as i64;
            }
        }
        -1
    }
}
