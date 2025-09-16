use macroquad::prelude::*;
use std::rc::Rc;

pub struct Anim {
    texture: Rc<Texture2D>,
    frame_width: f32,
    frame_height: f32,
    current_frame: usize,
    frame_count: usize,
    row: usize,
    frame_time: f64,
    frame_duration: f64,
    x_offset: f32,
    y_offset: f32,
}

impl Anim {
    pub fn new(
        texture: Rc<Texture2D>,
        frame_width: f32,
        frame_height: f32,
        frame_count: usize,
        row: usize,
        fps: f64,
    ) -> Self {
        Anim {
            texture,
            frame_width,
            frame_height,
            current_frame: 0,
            frame_count,
            row,
            frame_time: 0.0,
            frame_duration: 1.0 / fps,
            x_offset: 4.0,
            y_offset: 1.0,
        }
    }

    pub fn d_new_idle(texture: Rc<Texture2D>, row: usize) -> Self {
        Anim {
            texture,
            row,
            frame_count: 2,
            ..Default::default()
        }
    }

    pub fn update(&mut self, delta: f64) {
        self.frame_time += delta;
        if self.frame_time >= self.frame_duration {
            self.frame_time -= self.frame_duration;
            self.current_frame = (self.current_frame + 1) % self.frame_count;
        }
    }

    pub fn draw(&self, x: f32, y: f32) {
        let src_rect = Rect {
            x: self.current_frame as f32 * (self.frame_width + self.x_offset),
            y: self.row as f32 * (self.frame_height + self.y_offset),
            w: self.frame_width,
            h: self.frame_height,
        };
        draw_texture_ex(
            &self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(src_rect),
                ..Default::default()
            },
        );
    }

    pub fn set_frame(&mut self, frame: usize) {
        if frame < self.frame_count {
            self.current_frame = frame;
        }
    }
}

impl Default for Anim {
    fn default() -> Self {
        Self {
            texture: Rc::new(Texture2D::empty()),
            frame_width: 16f32,
            frame_height: 16f32,
            current_frame: 0,
            frame_count: 4,
            row: 0,
            frame_time: 0.0,
            frame_duration: 1.0 / 2.0,
            x_offset: 4.0,
            y_offset: 1.0,
        }
    }
}
