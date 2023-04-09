use crate::types::*;

fn get_available_moves(game: &Game) -> Vec<GameMove> {
    fn handle_round_one(game: &Game) -> Vec<GameMove> {
        let player_has_settlement = game
            .game_board
            .iter()
            .any(|item| match item.item {
                (_, Corner(Settlement(h))) if h == game.current_color => true,
                _ => false,
            });
        let player_has_road = game
            .game_board
            .iter()
            .any(|item| match item.item {
                (_, Edge(Road(r))) if r == game.current_color => true,
                _ => false,
            });
        match (player_has_settlement, player_has_road) {
            (true, true) => vec![GameMove::EndTurn],
            (false, _) => vec![GameMove::PlaceSettlement],
            (_, false) => vec![GameMove::PlaceRoad],
        }
    }

    match game.round {
        1 => handle_round_one(game),
        _ => vec![GameMove::PlaceSettlement],
    }
}

fn execute_if_allowed<F: FnOnce() -> Result<Game, GameError>>(
    game_move: GameMove,
    f: F,
    game: &Game,
) -> Result<Game, GameError> {
    let move_if_valid = |allowed_moves: Vec<GameMove>| {
        if allowed_moves.contains(&game_move) {
            f()
        } else {
            Err(GameError::InvalidMove(game_move))
        }
    };
    let allowed_moves = get_available_moves(game);
    move_if_valid(allowed_moves)
}

fn place_settlement(id: ID, game: &Game) -> Result<Game, GameError> {
    let continue_with_settlement_placement = || {
        let current_color = game.current_color;
        let update_state_of_corner = |items: &mut Vec<GameBoardItem>| {
            match items.iter_mut().find(|item| match item.item {
                (ID(corner_id), Corner(_)) if corner_id == id => true,
                _ => false,
            }) {
                Some(item) => {
                    item.item = (ID(id), Corner(Settlement(current_color)));
                    Ok(())
                }
                None => Err(GameError::ItemIsNotACorner(ID(id))),
            }
        };

        update_state_of_corner(&mut game.game_board.clone())
            .map(|_| game.clone())
    };
    execute_if_allowed(GameMove::PlaceSettlement, continue_with_settlement_placement, game)
}

fn list_available_settlement_locations(game: &Game) -> Vec<(ID, Color)> {
    game.game_board
        .iter()
        .filter_map(|item| match item.item {
            (id, Corner(c)) => Some((id, c)),
            _ => None,
        })
        .collect()
}

fn list_available_road_locations(game: &Game) -> Vec<(ID, Color)> {
    game.game_board
        .iter()
        .filter_map(|item| match item.item {
            (id, Edge(e)) => Some((id, e)),
            _ => None,
        })
        .collect()
}

fn get_game_item_by_id(id: ID, game: &Game) -> Result<GameBoardItem, GameError> {
    let result = game
        .game_board
        .iter()
        .find(|item| match item.item {
            (ID(x), _) if *x == id => true,
            _ => false,
        })
        .map(|item| item.item);
    match result {
        Some(item) => Ok(item),
        None => Err(GameError::BoardItemNotFound(ID(id)))

