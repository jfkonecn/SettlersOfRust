use crate::board_pieces::{NUMBER_TOKENS, TERRAIN_HEXES};
use crate::types::*;
use itertools::Itertools;
use rand::rngs::*;
use rand::seq::SliceRandom;
use rand::*;
use std::f64::consts::PI;

#[derive(Debug)]
struct TokenException(String);

fn validate_blueprint(
    player_blueprints: Vec<PlayerBlueprint>,
) -> Result<Vec<Player>, Vec<GameError>> {
    let not_enough_players = if player_blueprints.len() < 2 {
        vec![GameError::NotEnoughPlayers]
    } else {
        vec![]
    };
    let character_limit = 50;
    let too_many_characters = player_blueprints
        .iter()
        .filter(|x| x.name.len() > character_limit)
        .map(|x| GameError::NameExceededCharacterLimit(character_limit, *x))
        .collect::<Vec<_>>();

    let duplicated_color = player_blueprints
        .iter()
        .group_by(|x| (*x).color)
        .into_iter()
        .filter(|(_, x)| x.count() != 1)
        .map(|(x, y)| GameError::DuplicatedColor(x, y.map(|x| *x).collect::<Vec<_>>()))
        .collect::<Vec<_>>();

    let errors = not_enough_players
        .into_iter()
        .chain(too_many_characters)
        .chain(duplicated_color)
        .collect::<Vec<_>>();

    if errors.is_empty() {
        let players = player_blueprints
            .iter()
            .map(|p| {
                let name = (*p).name;
                let color = (*p).color;
                Player {
                    name,
                    color,
                    hand: Hand {
                        total_settlements: 5,
                        total_cities: 4,
                        total_roads: 15,
                    },
                }
            })
            .collect();
        Ok(players)
    } else {
        Err(errors)
    }
}

fn create_all_polar_points((x, y): (f64, f64)) -> Vec<(f64, f64)> {
    let theta = y.atan2(x);
    let r = (x * x + y * y).sqrt();
    (0..5)
        .map(|x| (x as f64) * 60.0 * PI / 180.0)
        .map(|x| (r, x + theta))
        .collect()
}

fn round2(x: f64) -> f64 {
    (x * 100.0).round() / 100.0
}

fn polar_to_cartesian(points: Vec<(f64, f64)>) -> Vec<(f64, f64)> {
    points
        .iter()
        .map(|(r, t)| (r * t.cos(), r * t.sin()))
        .map(|(r, t)| (round2(r), round2(t)))
        .collect()
}

fn create_all_cartesian_points((x, y): (f64, f64)) -> Vec<(f64, f64)> {
    let points = create_all_polar_points((x, y));
    polar_to_cartesian(points)
}

fn build_terrain_tiles<R>(rng: &mut R) -> Result<Vec<GameBoardItem>, GameError>
where
    R: Rng + ?Sized,
{
    let terrains = TERRAIN_HEXES;
    let mut terrain_tiles = Vec::new();
    let mut tokens = NUMBER_TOKENS.clone();
    tokens.shuffle(rng);
    let mut tokens = tokens.iter();

    for terrain in terrains.iter() {
        let terrain_tile = match terrain {
            Terrain::Desert => {
                let token = tokens.next().ok_or(GameError::FailedToShuffleTiles(
                    "Ran out of number tokens".to_owned(),
                ))?;
                (
                    GameBoardItem::Terrain(TerrainTile::Barren {
                        terrain: *terrain,
                        robber_opt: Some(Robber::Robber),
                    }),
                    token,
                )
            }
            _ => {
                let token = tokens.next().ok_or(GameError::FailedToShuffleTiles(
                    "Ran out of number tokens".to_owned(),
                ))?;
                (
                    GameBoardItem::Terrain(TerrainTile::Productive {
                        terrain: *terrain,
                        circularToken: *token,
                        robber_opt: None,
                    }),
                    token,
                )
            }
        };
        terrain_tiles.push(terrain_tile.0);
    }

    Ok(terrain_tiles)
}

fn create_hex_points() -> Vec<GameBoardPoint> {
    let hex_center_to_edge = 1.0;
    let hex_edge_length = 2.0 / 3.0 * f64::sqrt(3.0);
    let mut sea_points = Vec::new();
    let mut level3_terrain = Vec::new();
    let mut level2_terrain = Vec::new();

    let sea_offsets = vec![
        (hex_center_to_edge * 6.0, 0.0),
        (hex_center_to_edge * 5.0, hex_edge_length * 1.5),
        (hex_center_to_edge * 4.0, hex_edge_length * 3.0),
    ];

    let level3_offsets = vec![
        (hex_center_to_edge * 4.0, 0.0),
        (hex_center_to_edge * 3.0, hex_edge_length * 1.5),
    ];

    let level2_offsets = vec![(hex_center_to_edge * 2.0, 0.0)];

    for offset in sea_offsets {
        let points = create_all_polar_points(offset);
        sea_points.extend(points.iter().cloned());
    }
    sea_points.sort_by(|(_, t1), (_, t2)| t1.partial_cmp(&t2).unwrap());
    sea_points = polar_to_cartesian(sea_points);

    for offset in level3_offsets {
        let points = create_all_polar_points(offset);
        level3_terrain.extend(points.iter().cloned());
    }
    level3_terrain = polar_to_cartesian(level3_terrain);

    for offset in level2_offsets {
        let points = create_all_polar_points(offset);
        level2_terrain.extend(points.iter().cloned());
    }
    level2_terrain = polar_to_cartesian(level2_terrain);

    let mut result = sea_points;
    result.append(&mut level3_terrain);
    result.append(&mut level2_terrain);
    result.push((0.0, 0.0));
    result
        .iter()
        .zip(sea_tiles.iter().chain(terrain_tiles.iter()))
        .enumerate()
        .map(|(i, ((&x, &y), &(tile, item)))| GameBoardPoint {
            x,
            y,
            item: (i + 1, tile),
        })
        .collect()
}

fn startGame(seed: u64) {
    // give rng type
    let mut rng = StdRng::seed_from_u64(seed);
    let t: usize = rng.gen();
}
