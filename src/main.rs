use std::rc::Rc;

use bytemon_dungeon::engine::{actor::Actor, log, world::World};
use bytemon_dungeon::ui_macroquad::anim::Anim;
use macroquad::prelude::*;

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

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

    let mut c_x = 0.;
    let mut c_y = 0.;
    let mut c_zoom_x = 0.1;
    let mut c_zoom_y = 0.1;
    // Setup 'render_target', used to hold the rendering result so we can resize it
    let render_target = render_target(VIRTUAL_WIDTH as u32, VIRTUAL_HEIGHT as u32);
    render_target.texture.set_filter(FilterMode::Linear);

    // Setup camera for the virtual screen, that will render to 'render_target'
    let mut render_target_cam =
        Camera2D::from_display_rect(Rect::new(0., 0., VIRTUAL_WIDTH, VIRTUAL_HEIGHT));
    render_target_cam.render_target = Some(render_target.clone());

    loop {
        // Get required scaling value
        let scale: f32 = f32::min(
            screen_width() / VIRTUAL_WIDTH,
            screen_height() / VIRTUAL_HEIGHT,
        );
        if is_key_down(KeyCode::D) {
            c_x -= 0.01;
        }
        if is_key_down(KeyCode::A) {
            c_x += 0.01;
        }
        if is_key_down(KeyCode::S) {
            c_y += 0.01;
        }
        if is_key_down(KeyCode::W) {
            c_y -= 0.01;
        }

        render_target_cam.offset = Vec2 { x: c_x, y: c_y };
        // Mouse position in the virtual screen
        let virtual_mouse_pos = Vec2 {
            x: (mouse_position().0 - (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5) / scale,
            y: (mouse_position().1 - (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5) / scale,
        };

        // ------------------------------------------------------------------------
        // Begin drawing the virtual screen to 'render_target'
        // ------------------------------------------------------------------------
        set_camera(&render_target_cam);

        clear_background(BLACK);

        draw_circle(VIRTUAL_WIDTH / 2.0 - 65.0, VIRTUAL_HEIGHT / 2.0, 35.0, RED);
        draw_circle(VIRTUAL_WIDTH / 2.0 + 65.0, VIRTUAL_HEIGHT / 2.0, 35.0, BLUE);
        draw_circle(
            VIRTUAL_WIDTH / 2.0,
            VIRTUAL_HEIGHT / 2.0 - 65.0,
            35.0,
            YELLOW,
        );

        draw_world(&world).await;

        draw_circle(virtual_mouse_pos.x, virtual_mouse_pos.y, 15.0, PINK);
        // ------------------------------------------------------------------------
        // Begin drawing the window screen
        // ------------------------------------------------------------------------
        set_default_camera();

        clear_background(DARKGREEN); // Will be the letterbox color

        // Draw 'render_target' to window screen, porperly scaled and letterboxed
        draw_texture_ex(
            &render_target.texture,
            (screen_width() - (VIRTUAL_WIDTH * scale)) * 0.5,
            (screen_height() - (VIRTUAL_HEIGHT * scale)) * 0.5,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(VIRTUAL_WIDTH * scale, VIRTUAL_HEIGHT * scale)),
                flip_y: true, // Must flip y otherwise 'render_target' will be upside down
                ..Default::default()
            },
        );

        draw_text(
            &format!("{:?}/{:?}", mouse_position().0, mouse_position().1),
            mouse_position().0 + 20.,
            mouse_position().1 + 20.,
            18.,
            WHITE,
        );
        draw_text("Hello Letterbox", 20.0, 20.0, 30.0, DARKGRAY);
        next_frame().await;
    }
}

async fn draw_world(w: &World) {
    for i in w.get_map().get_tiles().iter() {
        draw_rectangle(
            i.position.0 as f32 * 17.,
            i.position.1 as f32 * 17.,
            16.,
            16.,
            GREEN,
        );
    }
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
