use crate::map::*;
use crate::tile::*;

pub fn move_left(position: &mut MapPosition) {
    position.x -= 1;
}

pub fn move_right(position: &mut MapPosition) {
    position.x += 1;
}

pub fn move_up(position: &mut MapPosition) {
    position.y -= 1;
}

pub fn move_down(position: &mut MapPosition) {
    position.y += 1;
}

pub fn can_move_left(player_position: &MapPosition, map: &Map) -> bool {
    if player_position.x > 0 {
        map.tiles[player_position.x + player_position.y * map.width - 1]
            .clone()
            .is_walkable()
    } else {
        false
    }
}

pub fn can_move_right(player_position: &MapPosition, map: &Map) -> bool {
    if player_position.x < map.width - 1 {
        map.tiles[player_position.x + player_position.y * map.width + 1]
            .clone()
            .is_walkable()
    } else {
        false
    }
}

pub const fn can_move_up(player_position: &MapPosition) -> bool {
    player_position.y > 0
}

pub const fn can_move_down(player_position: &MapPosition, map: &Map) -> bool {
    player_position.y < map.height - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_five_by_five_map() -> Map {
        Map {
            width: 5,
            height: 5,
            tiles: vec![TileType::Grass; 5 * 5],
        }
    }

    const POSITION_MIDDLE: MapPosition = MapPosition { x: 2, y: 2 };
    const POSITION_TOP_LEFT: MapPosition = MapPosition { x: 0, y: 0 };
    const POSITION_TOP_RIGHT: MapPosition = MapPosition { x: 4, y: 0 };
    const POSITION_BOTTOM_LEFT: MapPosition = MapPosition { x: 0, y: 4 };
    const POSITION_BOTTOM_RIGHT: MapPosition = MapPosition { x: 4, y: 4 };

    #[test]
    fn test_can_move_left() {
        let map = create_five_by_five_map();
        assert!(!can_move_left(&POSITION_TOP_LEFT, &map));
        assert!(!can_move_left(&POSITION_BOTTOM_LEFT, &map));
        assert!(can_move_left(&POSITION_TOP_RIGHT, &map));
        assert!(can_move_left(&POSITION_BOTTOM_RIGHT, &map));
        assert!(can_move_left(&POSITION_MIDDLE, &map));
    }

    #[test]
    fn test_can_move_right() {
        let map = create_five_by_five_map();
        assert!(!can_move_right(&POSITION_TOP_RIGHT, &map));
        assert!(!can_move_right(&POSITION_BOTTOM_RIGHT, &map));
        assert!(can_move_right(&POSITION_TOP_LEFT, &map));
        assert!(can_move_right(&POSITION_BOTTOM_LEFT, &map));
        assert!(can_move_right(&POSITION_MIDDLE, &map));
    }

    #[test]
    fn test_can_move_up() {
        assert!(!can_move_up(&POSITION_TOP_LEFT));
        assert!(!can_move_up(&POSITION_TOP_RIGHT));
        assert!(can_move_up(&POSITION_BOTTOM_LEFT));
        assert!(can_move_up(&POSITION_BOTTOM_RIGHT));
        assert!(can_move_up(&POSITION_MIDDLE));
    }

    #[test]
    fn test_can_move_down() {
        let map = create_five_by_five_map();
        assert!(!can_move_down(&POSITION_BOTTOM_LEFT, &map));
        assert!(!can_move_down(&POSITION_BOTTOM_RIGHT, &map));
        assert!(can_move_down(&POSITION_TOP_LEFT, &map));
        assert!(can_move_down(&POSITION_TOP_RIGHT, &map));
        assert!(can_move_down(&POSITION_MIDDLE, &map));
    }
}
