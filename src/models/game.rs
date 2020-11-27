use super::*;
use sqlx::query;

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename = "game")]
pub struct Game {
    name: String,
    assignment: Assignment,
    map: GameMap,
    phase: Phase,

    // General game state
    players: Vec<Player>,

    // Board setup
    positions: Vec<Position>,
    forests: Vec<Forest>,
    clearings: Vec<Clearing>,
    water: Vec<Water>,
    connections: Vec<Connection>,
    rivers: Vec<River>,
    ferry: Option<Ferry>,
    tower: Option<Tower>,

    factions: Vec<Faction>,

    buildings: Vec<Building>,
    built_buildings: Vec<BuiltBuilding>,

    tokens: Vec<Token>,
    placed_tokens: Vec<PlacedToken>,

    cards: Vec<Card>,
    discards: Vec<Discard>,
    hand: Vec<Hand>,

    items: Vec<Item>,
    owned_items: Vec<OwnedItem>,
    ruin_items: Vec<RuinItem>,

    warriors: Vec<Warrior>,
    placed_warriors: Vec<PlacedWarrior>,

    // Faction specific details
    marquise: Option<Marquise>,

    eyrie: Option<Eyrie>,
    eyrie_decree: Vec<EyrieDecree>,
    eyrie_leaders: Vec<EyrieLeader>,
    eyrie_current_leader: Option<EyrieCurrentLeader>,

    alliance: Option<Alliance>,
    alliance_supporters: Vec<AllianceSupporter>,
    officers: Vec<Officer>,

    vagabond: Option<Vagabond>,
    vagabond2: Option<Vagabond>,
    vagabond_items: Vec<VagabondItem>,
    vagabond_relationships: Vec<VagabondRelationship>,
    quests: Vec<Quest>,
    active_quests: Vec<ActiveQuest>,
    completed_quests: Vec<CompletedQuest>,

    cult: Option<Cult>,
    acolytes: Vec<Acolyte>,
    lost_souls: Vec<LostSoul>,

    riverfolk: Option<Riverfolk>,
    commitments: Vec<Commitment>,
    funds: Vec<Fund>,
    payments: Vec<Payment>,

    duchy: Option<Duchy>,
    burrow: Vec<Burrow>,
    ministers: Vec<Minister>,

    conspiracy: Option<Conspiracy>,
}

impl Game {
    pub async fn load(name: &str) -> sqlx::Result<Self> {
        let mut conn = crate::POOL.get().unwrap().begin().await?;
        let game = query!(r#"SELECT assignment as "assignment: Assignment", map as "map: GameMap", phase as "phase: Phase" FROM games WHERE name = $1"#, name).fetch_one(&mut conn).await?;
        let game = Self {
            name: name.to_owned(),
            assignment: game.assignment,
            map: game.map,
            phase: game.phase,
            players: Player::load(name, &mut conn).await?,
            positions: Position::load(name, &mut conn).await?,
            forests: Forest::load(name, &mut conn).await?,
            clearings: Clearing::load(name, &mut conn).await?,
            water: Water::load(name, &mut conn).await?,
            connections: Connection::load(name, &mut conn).await?,
            rivers: River::load(name, &mut conn).await?,
            ferry: Ferry::load(name, &mut conn).await?,
            tower: Tower::load(name, &mut conn).await?,
            factions: Faction::load(name, &mut conn).await?,
            buildings: Building::load(name, &mut conn).await?,
            built_buildings: BuiltBuilding::load(name, &mut conn).await?,
            tokens: Token::load(name, &mut conn).await?,
            placed_tokens: PlacedToken::load(name, &mut conn).await?,
            cards: Card::load(name, &mut conn).await?,
            discards: Discard::load(name, &mut conn).await?,
            hand: Hand::load(name, &mut conn).await?,
            items: Item::load(name, &mut conn).await?,
            owned_items: OwnedItem::load(name, &mut conn).await?,
            ruin_items: RuinItem::load(name, &mut conn).await?,
            warriors: Warrior::load(name, &mut conn).await?,
            placed_warriors: PlacedWarrior::load(name, &mut conn).await?,
            marquise: Marquise::load(name, &mut conn).await?,
            eyrie: Eyrie::load(name, &mut conn).await?,
            eyrie_decree: EyrieDecree::load(name, &mut conn).await?,
            eyrie_leaders: EyrieLeader::load(name, &mut conn).await?,
            eyrie_current_leader: EyrieCurrentLeader::load(name, &mut conn).await?,
            alliance: Alliance::load(name, &mut conn).await?,
            alliance_supporters: AllianceSupporter::load(name, &mut conn).await?,
            officers: Officer::load(name, &mut conn).await?,
            vagabond: Vagabond::load(name, FactionId::Vagabond, &mut conn).await?,
            vagabond2: Vagabond::load(name, FactionId::Vagabond2, &mut conn).await?,
            vagabond_items: VagabondItem::load(name, &mut conn).await?,
            vagabond_relationships: VagabondRelationship::load(name, &mut conn).await?,
            quests: Quest::load(name, &mut conn).await?,
            active_quests: ActiveQuest::load(name, &mut conn).await?,
            completed_quests: CompletedQuest::load(name, &mut conn).await?,
            cult: Cult::load(name, &mut conn).await?,
            acolytes: Acolyte::load(name, &mut conn).await?,
            lost_souls: LostSoul::load(name, &mut conn).await?,
            riverfolk: Riverfolk::load(name, &mut conn).await?,
            commitments: Commitment::load(name, &mut conn).await?,
            funds: Fund::load(name, &mut conn).await?,
            payments: Payment::load(name, &mut conn).await?,
            duchy: Duchy::load(name, &mut conn).await?,
            burrow: Burrow::load(name, &mut conn).await?,
            ministers: Minister::load(name, &mut conn).await?,
            conspiracy: Conspiracy::load(name, &mut conn).await?,
        };
        conn.commit().await?;
        Ok(game)
    }

    pub async fn save(&self) -> sqlx::Result<()> {
        todo!()
    }
    
    pub fn create(config: GameConfig) -> Self {
        Game {
            name: config.name,
            assignment: config.assignment,
            map: config.map,
            ..Self::default()
        }
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn phase(&self) -> Phase {
        self.phase
    }

    pub fn players(&self) -> &[Player] {
        self.players.as_slice()
    }

    pub fn factions(&self) -> &[Faction] {
        self.factions.as_slice()
    }

    pub fn add_player(&mut self, name: &str) -> Result<(), String> {
        if self.phase != Phase::Lobby {
            return Err("Players cannot be changed once the game has started.".into());
        }
        let is_existing_player = self.players.iter().any(|player| player.name() == name);
        if is_existing_player {
            return Err(format!(
                "There is already a player named {} in this game.",
                name
            ));
        }
        if self.players.len() == self.factions.len() {
            return Err("This game is already full.".into());
        }
        self.players.push(Player::new(name.to_owned()));
        Ok(())
    }

    pub fn remove_player(&mut self, name: &str) -> Result<(), String> {
        if self.phase != Phase::Lobby {
            return Err("Players cannot be changed once the game has started.".into());
        }
        let index = self.players.iter().position(|player| player.name() == name);
        if let Some(index) = index {
            self.players.remove(index);
        }
        Ok(())
    }
}
