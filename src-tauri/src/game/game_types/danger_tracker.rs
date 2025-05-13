use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{game::board::danger_level::DangerLevel, models::dummy_room::DummyPlayer};

pub struct DangerTracker {
    player_level: HashMap<DummyPlayer, DangerLevel>,
    players_by_level: BTreeMap<DangerLevel, HashSet<DummyPlayer>>,
    max_level: DangerLevel,
    empty_set: HashSet<DummyPlayer>,
}

impl DangerTracker {
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
    pub fn insert(&mut self, player: DummyPlayer, danger_level: DangerLevel) {
        let old_value = self.player_level.get(&player).unwrap();
        if old_value == &danger_level {
            return;
        }
        let players_old_level = self.players_by_level.get_mut(old_value).unwrap();
        players_old_level.remove(&player);
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
    pub fn get_highest(&mut self) -> &HashSet<DummyPlayer> {
        self.players_by_level
            .get(&self.max_level)
            .unwrap_or(&self.empty_set)
    }
}
