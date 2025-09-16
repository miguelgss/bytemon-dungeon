use crate::engine::{actor::Actor, log::add_to_log};

const PLAYER_CHAR: &str = "@";

#[derive(Debug)]
enum ETileType {
    Floor,
    Wall,
}

impl ETileType {
    pub fn get_char(&self) -> char {
        match self {
            ETileType::Floor => '.',
            ETileType::Wall => '#',
        }
    }
}

struct Tile {
    ttype: ETileType,
    position: (i16, i16),
}

struct Map {
    origin: (i16, i16),
    //end: (i16, i16),
    tiles: Vec<Tile>,
    limit_x: i16,
    limit_y: i16,
}

impl Map {
    fn init_basic_square() -> Map {
        let mut square_map = Map {
            tiles: Vec::with_capacity(100),
            limit_x: 10,
            limit_y: 10,
            origin: (0, 0),
        };

        for x in 0..square_map.limit_x {
            for y in 0..square_map.limit_y {
                let mut tile_type = ETileType::Floor;
                if x == 0 || x == square_map.limit_x - 1 || y == 0 || y == square_map.limit_y - 1 {
                    tile_type = ETileType::Wall;
                }

                square_map.tiles.push(Tile {
                    ttype: tile_type,
                    position: (x, y),
                });
            }
        }
        square_map
    }
}

pub struct World {
    player: Actor,
    actors: Vec<Option<Actor>>,
    map: Map,
}

impl World {
    pub fn player_move(&mut self, direction: (i16, i16)) {
        let tile_to_move = self.map.tiles.iter().find(|x| {
            x.position.0 == self.player.get_position().0 + direction.0
                && x.position.1 == self.player.get_position().1 + direction.1
        });

        if let Some(x) = tile_to_move {
            match x.ttype {
                ETileType::Floor => {
                    self.player.move_to_direction(direction);
                }
                ETileType::Wall => {
                    add_to_log(format!("{} hitted a wall.", self.player.get_name()));
                }
            }
        }
    }

    pub fn print_map(&self) {
        let mut map_str = "".to_owned();
        self.map.tiles.iter().for_each(|x| {
            let mut str_insert = format!("{}", x.ttype.get_char());
            if self.player.get_position() == x.position {
                str_insert = PLAYER_CHAR.to_owned();
            }

            map_str += &str_insert;

            if x.position.1 + 1 == self.map.limit_y {
                map_str += "\n";
            }
        });

        println!("{map_str}");
    }

    pub fn print_map_other_origin(&self) {
        let (origin_x, origin_y) = self.map.origin;

        // Collect visible tiles with adjusted positions relative to the origin
        let mut visible_tiles: Vec<(i16, i16, &Tile)> = self
            .map
            .tiles
            .iter()
            .filter_map(|tile| {
                let adjusted_x = tile.position.0.saturating_sub(origin_x);
                let adjusted_y = tile.position.1.saturating_sub(origin_y);
                if adjusted_x < self.map.limit_x && adjusted_y < self.map.limit_y {
                    Some((adjusted_x, adjusted_y, tile))
                } else {
                    None
                }
            })
            .collect();

        // Sort by row (y) then column (x) for proper 2D layout
        visible_tiles.sort_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));

        let mut map_str = String::new();

        let mut current_row = 0;

        for (_, y, tile) in visible_tiles {
            // Insert newline if we moved to a new row
            if y != current_row {
                map_str += "\n";
                current_row = y;
            }

            // Check if the player is on this tile position before offset
            let player_pos = self.player.get_position();
            let is_player_here = player_pos.0 == tile.position.0 && player_pos.1 == tile.position.1;
            let ch = if is_player_here {
                PLAYER_CHAR.to_owned()
            } else {
                tile.ttype.get_char().to_string()
            };

            map_str += &ch;
        }

        map_str += "\n";

        println!("{map_str}");
    }
    pub fn start_empty_square(player: Actor) -> Self {
        Self {
            player,
            actors: Vec::new(),
            map: Map::init_basic_square(),
        }
    }
}
