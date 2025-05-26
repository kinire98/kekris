use serde::{Deserialize, Serialize};

use super::game_info::{ClassicGameInfo, GameInfo, GameTypeInfo};

/// `EmitGameInfo` is a struct used to encapsulate game information for emitting to the UI.
#[derive(Debug, Serialize, Deserialize)]
pub struct EmitGameInfo {
    /// The information about the last game played.
    last_game_info: GameInfo,
    /// The top five game results.
    top_five_results: Vec<GameInfo>,
    /// The index of the last game in the top five results, or -1 if it's not in the top five.
    last_in_top_five: i64,
    /// Indicates whether the data is empty or not.
    empty: bool,
}

impl EmitGameInfo {
    /// Creates a new `EmitGameInfo` instance.
    ///
    /// # Arguments
    ///
    /// * `last_game_info` - The information about the last game played.
    /// * `previous_results` - The list of previous game results.
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
            empty: false,
        }
    }
    /// Creates an empty `EmitGameInfo` instance.
    pub fn empty() -> Self {
        EmitGameInfo {
            last_game_info: GameInfo::new_from(
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                GameTypeInfo::Classic(ClassicGameInfo::default()),
            ),
            top_five_results: vec![],
            last_in_top_five: -1,
            empty: true,
        }
    }
    /// Sorts the previous results and returns the top five classic game results.
    fn top_five_classic(mut previous_results: Vec<GameInfo>) -> Vec<GameInfo> {
        previous_results.sort_by(|a, b| match (a.type_of_info(), b.type_of_info()) {
            (
                super::game_info::GameTypeInfo::Classic(classic_game_info_a),
                super::game_info::GameTypeInfo::Classic(classic_game_info_b),
            ) => classic_game_info_b
                .points()
                .cmp(&classic_game_info_a.points()),
            _ => panic!("Invalid state"),
        });
        previous_results
            .into_iter()
            .take(5)
            .collect::<Vec<GameInfo>>()
    }
    /// Sorts the previous results and returns the top five lines game results.
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
    /// Sorts the previous results and returns the top five blitz game results.
    fn top_five_blitz(mut previous_results: Vec<GameInfo>) -> Vec<GameInfo> {
        previous_results.sort_by(|a, b| match (a.type_of_info(), b.type_of_info()) {
            (
                super::game_info::GameTypeInfo::Blitz(blitz_game_info_a),
                super::game_info::GameTypeInfo::Blitz(blitz_game_info_b),
            ) => blitz_game_info_b.points().cmp(&blitz_game_info_a.points()),
            _ => panic!("Invalid state"),
        });
        previous_results
            .into_iter()
            .take(5)
            .collect::<Vec<GameInfo>>()
    }
    /// Checks if a game info is in the top five results.
    ///
    /// # Arguments
    ///
    /// * `info` - The game info to check.
    /// * `top_five` - The top five results.
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
