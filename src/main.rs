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

    test_animations().await;
}

async fn test_animations() {
    let texture = Rc::new(load_texture("resources/rookie_sheet.png").await.unwrap());
    texture.set_filter(FilterMode::Nearest);

    let animation = Anim::d_new_idle(Rc::clone(&texture), 0);
    let animation2 = Anim::d_new_idle(Rc::clone(&texture), 1);
    let animation3 = Anim::d_new_action(Rc::clone(&texture), 0);
    let animation4 = Anim::d_new_defeated(Rc::clone(&texture), 0);

    let mut anims = [animation, animation2, animation3, animation4];
    let positions: [(f32, f32); 4] = [(100., 100.), (130., 130.), (160., 160.), (190., 190.)];

    loop {
        clear_background(LIGHTGRAY);

        for a in anims.iter_mut() {
            a.update(get_frame_time() as f64);
        }

        for i in 0..4 {
            anims[i].draw(positions[i].0, positions[1].1)
        }

        next_frame().await;
    }
}
