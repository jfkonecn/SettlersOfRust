// https://catan.fandom.com/wiki/Catan
use crate::types::*;

pub static sea_frames: Vec<SeaFrame> = vec![SeaFrame::SingleHarbor; 3]
    .into_iter()
    .chain(vec![SeaFrame::TwoHarbor; 3].into_iter())
    .collect();

pub static harbor_tokens: Vec<HarborToken> = vec![
    ResourceCard::Lumber,
    ResourceCard::Wool,
    ResourceCard::Grain,
    ResourceCard::Brick,
    ResourceCard::Ore,
]
.into_iter()
.map(|x| HarborToken {
    give_amount: 2,
    resource_to_give: Some(x),
})
.chain(
    vec![
        HarborToken {
            give_amount: 3,
            resource_to_give: None,
        };
        4
    ]
    .into_iter(),
)
.collect();

pub static terrain_hexes: Vec<TerrainHex> = vec![
    Forest, Forest, Forest, Forest, Pasture, Pasture, Pasture, Pasture, Fields, Fields, Fields,
    Fields, Hills, Hills, Hills, Mountains, Mountains, Mountains, Desert,
]
.into_iter()
.collect();

pub static number_tokens: Vec<NumberToken> = vec![
    NumberToken {
        letter: 'B',
        value: 2,
        color: Black,
    },
    NumberToken {
        letter: 'C',
        value: 6,
        color: Red,
    },
    NumberToken {
        letter: 'P',
        value: 6,
        color: Red,
    },
    NumberToken {
        letter: 'E',
        value: 8,
        color: Red,
    },
    NumberToken {
        letter: 'K',
        value: 8,
        color: Red,
    },
    NumberToken {
        letter: 'H',
        value: 12,
        color: Black,
    },
    NumberToken {
        letter: 'D',
        value: 3,
        color: Black,
    },
    NumberToken {
        letter: 'Q',
        value: 3,
        color: Black,
    },
    NumberToken {
        letter: 'J',
        value: 4,
        color: Black,
    },
    NumberToken {
        letter: 'N',
        value: 4,
        color: Black,
    },
    NumberToken {
        letter: 'A',
        value: 5,
        color: Black,
    },
    NumberToken {
        letter: 'O',
        value: 5,
        color: Black,
    },
    NumberToken {
        letter: 'G',
        value: 9,
        color: Black,
    },
    NumberToken {
        letter: 'M',
        value: 9,
        color: Black,
    },
    NumberToken {
        letter: 'L',
        value: 10,
        color: Black,
    },
    NumberToken {
        letter: 'F',
        value: 10,
        color: Black,
    },
    NumberToken {
        letter: 'I',
        value: 11,
        color: Black,
    },
    NumberToken {
        letter: 'R',
        value: 11,
        color: Black,
    },
]
.into_iter()
.collect();

pub static development_cards: Vec<DevelopmentCard> = vec![KnightCard; 14]
    .into_iter()
    .chain(vec![ProgressCard; 6].into_iter())
    .chain(vec![VictoryPointCard; 5].into_iter())
    .collect();
