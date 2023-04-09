use crate::types::*;
use itertools::Itertools;
use rand::rngs::{StdRng, ThreadRng};
use rand::{Rng, SeedableRng};
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

fn create_all_polar_points(x: f64, y: f64) -> Vec<(f64, f64)> {
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

fn create_all_cartesian_points(x: f64, y: f64) -> Vec<(f64, f64)> {
    let points = create_all_polar_points(x, y);
    polar_to_cartesian(points)
}

fn startGame(seed: u64) {
    // give rng type
    let mut rng = StdRng::seed_from_u64(seed);
    let t: usize = rng.gen();
}
