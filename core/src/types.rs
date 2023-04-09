#[derive(Clone, PartialEq, Eq)]
pub enum PlayerColor {
    Red,
    White,
    Orange,
    Blue,
}

pub struct Hand {
    total_settlements: i32,
    total_cities: i32,
    total_roads: i32,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            total_settlements: 5,
            total_cities: 4,
            total_roads: 15,
        }
    }
}

pub struct PlayerBlueprint {
    pub name: String,
    pub color: PlayerColor,
}

pub struct Player {
    name: String,
    color: PlayerColor,
    hand: Hand,
}

impl Player {
    pub fn new(name: String, color: PlayerColor, hand: Hand) -> Self {
        Self {
            name,
            color,
            hand: Hand::new(),
        }
    }
}

pub enum ResourceCard {
    Brick,
    Grain,
    Lumber,
    Ore,
    Wool,
}

pub enum Terrain {
    Hills,
    Forest,
    Mountains,
    Fields,
    Pasture,
    Desert,
}

pub enum SeaFrame {
    SingleHarbor,
    TwoHarbor,
}

pub struct HarborToken {
    give_amount: i32,
    resource_to_give: Option<ResourceCard>,
}

pub enum CircularTokenColor {
    Black,
    Red,
}

pub struct CircularToken {
    value: i32,
    color: CircularTokenColor,
    letter: char,
}

pub enum SpecialCard {
    LongestRoad,
    LargestArmy,
}

pub struct City {
    color: PlayerColor,
}

pub struct Settlement {
    color: PlayerColor,
}

pub enum Robber {
    Robber,
}

pub enum TerrainTile {
    Productive(Terrain, CircularToken, Option<Robber>),
    Barren(Terrain, Option<Robber>),
}

pub enum DevelopmentCard {
    KnightCard,
    ProgressCard,
    VictoryPointCard,
}

pub enum SeaTile {
    Harbor(HarborToken),
    Water,
}

pub enum HexEdge {
    Road(PlayerColor),
    Empty,
}

pub enum HexCorner {
    Settlement(PlayerColor),
    Empty,
}

pub enum GameBoardItem {
    Terrain(TerrainTile),
    Sea(SeaTile),
    Edge(HexEdge),
    Corner(HexCorner),
}

pub struct GameBoardPoint {
    pub x: f64,
    pub y: f64,
    pub item: (i32, GameBoardItem),
}

pub struct AvailableResourceCards {
    brick: i32,
    grain: i32,
    lumber: i32,
    ore: i32,
    wool: i32,
}

pub struct Game {
    pub game_board: Vec<GameBoardPoint>,
    pub players: Vec<Player>,
    pub available_resource_cards: AvailableResourceCards,
    pub development_cards: Vec<DevelopmentCard>,
    pub round: i32,
    pub starting_color: PlayerColor,
    pub current_color: PlayerColor,
}

pub enum Move {
    PlaceSettlement,
    PlaceRoad,
    EndTurn,
}

pub enum GameError {
    NameExceededCharacterLimit(usize, PlayerBlueprint),
    DuplicatedColor(PlayerColor, Vec<PlayerBlueprint>),
    NotEnoughPlayers,
    InvalidMove(Move),
    BoardItemNotFound(i32),
    ItemIsNotACorner(i32),
}
