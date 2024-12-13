use std::path::PathBuf;

use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::VirtualKeyCode;
use speedy2d::Graphics2D;

use glam::{UVec2, Vec2};
use walkdir::WalkDir;

use crate::app::{Keyboard, Mouse};
use crate::config::Config;

pub struct Game {
    config: Config,
    paths_to_open: Vec<PathBuf>,
    images: Vec<ImageHandle>,
    selected: usize,

    mouse: Vec2,

    counter: usize,

    viewport_size: UVec2,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        let mut paths_to_open = Vec::new();
        let paths: Vec<&str> = config.input.split_whitespace().collect();
        for path in paths {
            for entry in WalkDir::new(path)
                .follow_links(true)
                .max_depth(1)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    paths_to_open.push(entry.into_path());
                }
            }
        }

        Self {
            config,
            paths_to_open,
            images: Vec::new(),
            selected: 0,

            mouse: Vec2::new(0.0, 0.0),

            counter: 0,
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {
        for path in &self.paths_to_open {
            let image_handle =
                graphics.create_image_from_file_path(None, ImageSmoothingMode::Linear, path);
            //image_handle.clone().unwrap();
            if let Ok(image) = image_handle {
                self.images.push(image);
            }
        }

        dbg!(&self.images);
    }

    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
        self.mouse = mouse.position;
    }

    pub fn update(&mut self, current_frame: u64) {}

    pub fn draw(&self, graphics: &mut Graphics2D) {
        // if let Some(robot) = &self.robot {
        //     let c = Color::from_gray(0.5);
        //     let vp = self.viewport_size;
        //     let center = Vec2::new(vp.x as f32 / 2.0, vp.y as f32 / 2.0);
        //     graphics.draw_line(self.mouse, center, 2.0, c);
        //     robot.draw(&Rect::new(center, center + Vec2::new(50.0, 50.0)), graphics);
        // }
        if let Some(image) = self.images.get(self.selected) {
            let pos = Vec2::new(0.0, 0.0);
            graphics.draw_image(pos, image);
        }
    }
}
