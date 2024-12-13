use speedy2d::color::Color;
use speedy2d::image::{ImageFileFormat, ImageHandle, ImageSmoothingMode};
use speedy2d::window::VirtualKeyCode;
use speedy2d::Graphics2D;
use speedy2d::Rect;

use glam::{UVec2, Vec2};

use crate::app::{Keyboard, Mouse};
use crate::config::Config;
use crate::robot::Robot;
use crate::spritesheet::Spritesheet;

pub struct Game {
    config: Config,
    images: Vec<ImageHandle>,
    spritesheets: Vec<Spritesheet>,

    robot: Option<Robot>,
    mouse: Vec2,

    counter: usize,

    viewport_size: UVec2,
}

impl Game {
    pub const fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        Self {
            config,
            images: Vec::new(),
            spritesheets: Vec::new(),

            robot: None,
            mouse: Vec2::new(0.0, 0.0),

            counter: 0,
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        let image_handle = graphics
            .create_image_from_file_path(
                Some(ImageFileFormat::PNG),
                ImageSmoothingMode::Linear,
                "assets/robot.png",
            )
            .unwrap();
        //self.images.push(spritesheet);
        self.robot = Some(Robot::new(Spritesheet::new(image_handle, 9, 5)));
    }

    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
        self.mouse = mouse.position;
        if let Some(robot) = &mut self.robot {
            if keyboard.just_pressed.contains(&VirtualKeyCode::Space) {
                robot.dbg_next_state(self.counter);
                self.counter += 1;
            }
        }
    }

    pub fn update(&mut self, current_frame: u64) {
        if let Some(robot) = &mut self.robot {
            robot.update(current_frame);
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        if let Some(robot) = &self.robot {
            let c = Color::from_gray(0.5);
            let vp = self.viewport_size;
            let center = Vec2::new(vp.x as f32 / 2.0, vp.y as f32 / 2.0);
            graphics.draw_line(self.mouse, center, 2.0, c);
            robot.draw(&Rect::new(center, center + Vec2::new(50.0, 50.0)), graphics);
        }
    }
}
