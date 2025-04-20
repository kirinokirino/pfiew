use glam::Vec2;
use speedy2d::window::MouseButton;
use speedy2d::Rect;

use crate::app::{Keyboard, Mouse};

#[derive(Debug)]
pub struct Camera {
    pub offset: Vec2,
    pub scale: f32,
}

impl Camera {
    pub const fn new() -> Self {
        Self {
            offset: Vec2::ZERO,
            scale: 1.0,
        }
    }

    pub fn world_to_screen(&self, world_pos: Vec2) -> Vec2 {
        world_pos * self.scale + self.offset
    }

    pub fn screen_to_world(&self, screen_pos: Vec2) -> Vec2 {
        (screen_pos - self.offset) / self.scale
    }

    pub fn transform_point(&self, point: Vec2) -> Vec2 {
        self.world_to_screen(point)
    }

    pub fn transform(&self, rect: &Rect) -> Rect {
        Rect::new(
            self.world_to_screen(rect.top_left),
            self.world_to_screen(rect.bottom_right),
        )
    }

    fn calculate_scale(scroll_lines: f64) -> f32 {
        let zoom_speed: f32 = 0.95;
        zoom_speed.powf(scroll_lines as f32)
    }

    pub fn handle_input(
        &mut self,
        mouse: &Mouse,
        mouse_delta: Vec2,
        scroll_delta: f64,
        _keyboard: &Keyboard,
    ) {
        if mouse.pressed.contains(&MouseButton::Left) {
            self.offset += mouse_delta;
        }

        let scale_change = Self::calculate_scale(scroll_delta);
        if scroll_delta != 0.0 {
            let mouse_world_before = self.screen_to_world(mouse.position);
            self.scale *= scale_change;
            let mouse_world_after = self.screen_to_world(mouse.position);

            // Move the camera so the world point under the mouse stays under the cursor
            let correction = (mouse_world_after - mouse_world_before) * self.scale;
            self.offset += correction;
        }
    }
}
