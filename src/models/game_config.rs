use super::{Assignment, Deck, FactionId, GameMap};

#[derive(Clone, Eq, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct GameConfig {
    pub name: String,
    pub factions: Vec<FactionId>,
    pub assignment: Assignment,
    pub map: GameMap,
    pub deck: Deck,
}
