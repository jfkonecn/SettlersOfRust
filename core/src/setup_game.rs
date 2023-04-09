use crate::types::*;

#[derive(Debug)]
struct TokenException(String);

fn validate_blueprint(
    player_blueprints: &[PlayerBlueprint],
) -> Result<Vec<Player>, Vec<GameError>> {
    let not_enough_players = if player_blueprints.len() < 2 {
        vec![GameError::NotEnoughPlayers]
    } else {
        vec![]
    };
    let character_limit = 50;
    let too_many_characters = player_blueprints
        .iter()
        .filter(|(Name(name), _)| name.len() > character_limit)
        .map(|x| GameError::NameExceededCharacterLimit(character_limit, x.clone()))
        .collect::<Vec<_>>();

    let duplicated_color = player_blueprints
        .iter()
        .group_by(|(_, color)| color.clone())
        .into_iter()
        .filter(|(_, x)| x.len() != 1)
        .map(|(x, y)| GameError::DuplicatedColor(x, y.map(|x| x.clone()).collect()))
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
                let (Name(name), color) = p;
                Player {
                    name: name.clone(),
                    color: color.clone(),
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
