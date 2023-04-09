use crate::types::*;

fn get_available_moves(game: &Game) -> Vec<Move> {
    if let 1 = game.round {
        let player_has_settlement = game.game_board.iter().any(|x| match x.item {
            (_, GameBoardItem::Corner(HexCorner::Settlement(h))) if h == game.current_color => true,
            _ => false,
        });
        let player_has_road = game.game_board.iter().any(|item| match item.item {
            (_, GameBoardItem::Edge(HexEdge::Road(r))) if r == game.current_color => true,
            _ => false,
        });
        match (player_has_settlement, player_has_road) {
            (true, true) => vec![Move::EndTurn],
            (false, _) => vec![Move::PlaceSettlement],
            (_, false) => vec![Move::PlaceRoad],
        }
    } else {
        vec![Move::PlaceSettlement]
    }
}

fn check_if_valid_move(game_move: Move, game: &Game) -> Result<(), GameError> {
    let moves = get_available_moves(game);
    if moves.contains(&game_move) {
        Ok(())
    } else {
        Err(GameError::InvalidMove(game_move))
    }
}

fn place_settlement(id: i32, game: &Game) -> Result<(), GameError> {
    let current_color = game.current_color;
    match game.game_board.iter_mut().find(|x| match x.item {
        (corner_id, GameBoardItem::Corner(_)) if corner_id == id => true,
        _ => false,
    }) {
        Some(item) => {
            item.item = (
                id,
                GameBoardItem::Corner(HexCorner::Settlement(current_color)),
            );
            check_if_valid_move(Move::PlaceSettlement, game)
        }
        None => Err(GameError::ItemIsNotACorner(id)),
    }
}

fn list_available_settlement_locations(game: &Game) -> Vec<(i32, HexCorner)> {
    let mut vec = vec![];
    for GameBoardPoint { item, .. } in game.game_board {
        if let (id, GameBoardItem::Corner(c)) = item {
            vec.push((id, c));
        }
    }
    vec
}

fn list_available_road_locations(game: &Game) -> Vec<(i32, HexEdge)> {
    let mut vec = vec![];
    for GameBoardPoint { item, .. } in game.game_board {
        if let (id, GameBoardItem::Edge(e)) = item {
            vec.push((id, e));
        }
    }
    vec
}

fn get_game_item_by_id(id: i32, game: &Game) -> Result<GameBoardItem, GameError> {
    for GameBoardPoint { item, .. } in game.game_board {
        let (item_id, item) = item;
        if item_id == id {
            return Ok(item);
        }
    }
    Err(GameError::BoardItemNotFound(id))
}
