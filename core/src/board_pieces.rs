// https://catan.fandom.com/wiki/Catan
use crate::types::*;

pub static SEA_FRAMES: Vec<SeaFrame> = vec![
    SeaFrame::SingleHarbor,
    SeaFrame::SingleHarbor,
    SeaFrame::SingleHarbor,
    SeaFrame::TwoHarbor,
    SeaFrame::TwoHarbor,
    SeaFrame::TwoHarbor,
];

pub static HARBOR_TOKENS: Vec<HarborToken> = vec![
    HarborToken {
        give_amount: 2,
        resource_to_give: Some(ResourceCard::Lumber),
    },
    HarborToken {
        give_amount: 2,
        resource_to_give: Some(ResourceCard::Wool),
    },
    HarborToken {
        give_amount: 2,
        resource_to_give: Some(ResourceCard::Grain),
    },
    HarborToken {
        give_amount: 2,
        resource_to_give: Some(ResourceCard::Brick),
    },
    HarborToken {
        give_amount: 2,
        resource_to_give: Some(ResourceCard::Ore),
    },
    HarborToken {
        give_amount: 3,
        resource_to_give: None,
    },
    HarborToken {
        give_amount: 3,
        resource_to_give: None,
    },
    HarborToken {
        give_amount: 3,
        resource_to_give: None,
    },
    HarborToken {
        give_amount: 3,
        resource_to_give: None,
    },
];

pub static TERRAIN_HEXES: Vec<Terrain> = vec![
    Terrain::Forest,
    Terrain::Forest,
    Terrain::Forest,
    Terrain::Forest,
    Terrain::Pasture,
    Terrain::Pasture,
    Terrain::Pasture,
    Terrain::Pasture,
    Terrain::Fields,
    Terrain::Fields,
    Terrain::Fields,
    Terrain::Fields,
    Terrain::Hills,
    Terrain::Hills,
    Terrain::Hills,
    Terrain::Mountains,
    Terrain::Mountains,
    Terrain::Mountains,
    Terrain::Desert,
];

pub static NUMBER_TOKENS: Vec<CircularToken> = vec![
    CircularToken {
        letter: 'B',
        value: 2,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'C',
        value: 6,
        color: CircularTokenColor::Red,
    },
    CircularToken {
        letter: 'P',
        value: 6,
        color: CircularTokenColor::Red,
    },
    CircularToken {
        letter: 'E',
        value: 8,
        color: CircularTokenColor::Red,
    },
    CircularToken {
        letter: 'K',
        value: 8,
        color: CircularTokenColor::Red,
    },
    CircularToken {
        letter: 'H',
        value: 12,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'D',
        value: 3,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'Q',
        value: 3,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'J',
        value: 4,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'N',
        value: 4,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'A',
        value: 5,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'O',
        value: 5,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'G',
        value: 9,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'M',
        value: 9,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'L',
        value: 10,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'F',
        value: 10,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'I',
        value: 11,
        color: CircularTokenColor::Black,
    },
    CircularToken {
        letter: 'R',
        value: 11,
        color: CircularTokenColor::Black,
    },
];

pub static DEVELOPMENT_CARDS: Vec<DevelopmentCard> = vec![
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::KnightCard,
    DevelopmentCard::ProgressCard,
    DevelopmentCard::ProgressCard,
    DevelopmentCard::ProgressCard,
    DevelopmentCard::ProgressCard,
    DevelopmentCard::ProgressCard,
    DevelopmentCard::ProgressCard,
    DevelopmentCard::VictoryPointCard,
    DevelopmentCard::VictoryPointCard,
    DevelopmentCard::VictoryPointCard,
    DevelopmentCard::VictoryPointCard,
    DevelopmentCard::VictoryPointCard,
];
