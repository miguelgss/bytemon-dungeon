use std::rc::Rc;

use bytemon_dungeon::engine::{actor::Actor, log, world::World};
use bytemon_dungeon::ui_macroquad::anim::Anim;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "ByteD".to_owned(),
        fullscreen: false,
        window_resizable: false,
        window_width: 640,
        window_height: 360,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
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

    let texture = Rc::new(load_texture("resources/rookie_sheet.png").await.unwrap());
    texture.set_filter(FilterMode::Nearest);

    let mut animation = Anim::d_new_idle(Rc::clone(&texture), 0);
    let mut animation2 = Anim::d_new_idle(Rc::clone(&texture), 1);
    loop {
        clear_background(LIGHTGRAY);

        let dt = get_frame_time() as f64;
        animation.update(dt);
        animation2.update(dt);

        animation.draw(100.0, 100.0);
        animation2.draw(120.0, 100.0);

        next_frame().await;
    }
}
