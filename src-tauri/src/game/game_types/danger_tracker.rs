use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{game::board::danger_level::DangerLevel, models::dummy_room::DummyPlayer};

/// `DangerTracker` tracks the danger level of each player in the game.
///
/// It maintains a mapping of players to their danger levels, and a reverse mapping of danger levels to the set of players at that level.
/// This allows efficient retrieval of players at a specific danger level, as well as the ability to quickly determine the highest danger level present in the game.
pub struct DangerTracker {
    /// `player_level`: Maps each player to their current danger level.
    player_level: HashMap<DummyPlayer, DangerLevel>,
    /// `players_by_level`: Maps each danger level to the set of players at that level.
    players_by_level: BTreeMap<DangerLevel, HashSet<DummyPlayer>>,
    /// `max_level`: The maximum danger level currently present among all players.
    max_level: DangerLevel,
    /// `empty_set`: An empty HashSet used as a default return value when no players are at the maximum danger level.
    empty_set: HashSet<DummyPlayer>,
}

impl DangerTracker {
    /// Creates a new `DangerTracker` instance.
    ///
    /// Initializes the tracker with a list of players, setting their initial danger level to `DangerLevel::Empty`.
    /// The `players_by_level` map is initialized with an empty danger level containing all the players.
    ///
    /// # Arguments
    ///
    /// * `players`: A vector of `DummyPlayer` representing the players in the game.
    pub fn new(players: Vec<DummyPlayer>) -> Self {
        let mut map = HashMap::new();
        let mut tree: BTreeMap<DangerLevel, HashSet<DummyPlayer>> = BTreeMap::new();
        let mut first = true;
        players.into_iter().for_each(|player| {
            map.insert(player.clone(), DangerLevel::Empty);
            if first {
                let mut set = HashSet::new();
                set.insert(player);
                tree.insert(DangerLevel::Empty, set);
                first = false;
            } else {
                let set = tree.get_mut(&DangerLevel::Empty).unwrap();
                set.insert(player);
            }
        });
        Self {
            player_level: map,
            players_by_level: tree,
            max_level: DangerLevel::Empty,
            empty_set: HashSet::new(),
        }
    }

    /// Inserts or updates the danger level for a player.
    ///
    /// Updates the player's danger level in the tracker. If the player is not already in the tracker, they are added.
    /// If the player's danger level is updated, the data structures are updated to reflect the change.
    /// It handles removing the player from their previous danger level, and adding them to their new danger level.
    /// It also updates the `max_level` if the new danger level is higher than the current `max_level`.
    ///
    /// # Arguments
    ///
    /// * `player`: The `DummyPlayer` to insert or update.
    /// * `danger_level`: The new `DangerLevel` for the player.
    pub fn insert(&mut self, player: DummyPlayer, danger_level: DangerLevel) {
        let old_value = self.player_level.get(&player);

        match old_value {
            Some(old_value) => {
                if old_value == &danger_level {
                    return;
                }
            }
            None => {
                self.player_level.insert(player.clone(), danger_level);
                self.players_by_level
                    .entry(danger_level)
                    .or_default()
                    .insert(player);
                if self.max_level < danger_level {
                    self.max_level = danger_level;
                }
                return;
            }
        }
        let old_value = old_value.unwrap();
        let players_old_level = self.players_by_level.get_mut(old_value).unwrap();
        players_old_level.remove(&player);
        // Check if level of danger is empty
        if players_old_level.is_empty() {
            self.players_by_level.remove(old_value);
            if old_value == &self.max_level {
                self.max_level = self
                    .players_by_level
                    .last_key_value()
                    .map(|(&level, _)| level)
                    .unwrap_or(DangerLevel::Empty);
            }
        }
        self.players_by_level
            .entry(danger_level)
            .or_default()
            .insert(player.clone());
        self.player_level.insert(player, danger_level);
        if self.max_level < danger_level {
            self.max_level = danger_level;
        }
    }

    /// Retrieves the set of players with the highest danger level.
    ///
    /// Returns a reference to the `HashSet` containing the players at the maximum danger level.
    /// If no players are at any danger level, returns a reference to an empty `HashSet`.
    ///
    /// # Returns
    ///
    /// A reference to a `HashSet<DummyPlayer>` containing the players at the highest danger level.
    pub fn get_highest(&mut self) -> &HashSet<DummyPlayer> {
        self.players_by_level
            .get(&self.max_level)
            .unwrap_or(&self.empty_set)
    }
}
