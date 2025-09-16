use bytemon_dungeon::engine::{actor::Actor, log, world::World};

fn main() {
    let player = Actor::new_player("Irineu".to_owned());
    let mut world = World::start_empty_square(player);
    world.player_move((1, 0));
    world.player_move((1, 0));
    world.player_move((1, 0));
    world.player_move((1, 0));
    world.player_move((0, 2));
    world.player_move((0, 1));
    world.player_move((0, 1));
    world.player_move((0, -6));
    log::tail();
    world.print_map_other_origin();
    world.print_map();
}
