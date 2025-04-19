use std::collections::HashMap;
use std::path::PathBuf;

use speedy2d::image::{ImageHandle, ImageSmoothingMode};
use speedy2d::window::{MouseButton, VirtualKeyCode};
use speedy2d::{Graphics2D, Rect};

use glam::{UVec2, Vec2};
use walkdir::WalkDir;

use crate::app::{Keyboard, Mouse};
use crate::config::Config;

mod camera;
use crate::game::camera::Camera;

pub struct Game {
    config: Config,
    image_paths: Vec<PathBuf>,
    loaded_images: HashMap<usize, ImageHandle>,

    selected: usize,
    camera: Camera,

    mouse: Mouse,

    counter: usize,

    viewport_size: UVec2,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        let mut image_paths = Vec::new();
        let paths: Vec<&str> = vec![&config.input];
        println!("Reading {} paths", paths.len());
        for path in paths {
            println!("{path}");
            for entry in WalkDir::new(path)
                .follow_links(true)
                .max_depth(1)
                .sort_by_file_name()
                .into_iter()
                .filter_map(|e| e.ok())
            {
                if entry.file_type().is_file() {
                    image_paths.push(entry.into_path());
                }
            }
        }

        Self {
            config,
            image_paths,
            loaded_images: HashMap::new(),
            selected: 0,
            camera: Camera::new(),

            mouse: Mouse::new(),

            counter: 0,
            viewport_size,
        }
    }

    pub fn setup(&mut self, graphics: &mut Graphics2D) {}

    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
        let mouse_delta = mouse.position - self.mouse.position;
        let scroll_delta = mouse.scroll_accumulated - self.mouse.scroll_accumulated;
        self.mouse = mouse.clone(); 
        
        self.camera.handle_input(mouse, &mouse_delta, scroll_delta, keyboard);
        if keyboard.just_pressed.contains(&VirtualKeyCode::Q) {
            self.selected += 1;
            if self.selected >= self.image_paths.len() {
                self.selected = 0;
            }
            println!("Selecting image {}", self.selected);
        }
    }

    pub fn update(&mut self, graphics: &mut Graphics2D, current_frame: u64) {
        if let None = &self.loaded_images.get(&self.selected) {
                let path_buf = &self.image_paths[self.selected];
                println!("Loading image {path_buf:?}");
                if let Ok(image) = graphics.create_image_from_file_path(
                    None,
                    ImageSmoothingMode::Linear,
                    path_buf,
                ){
                    self.loaded_images.insert(self.selected, image);
                }
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        // if let Some(robot) = &self.robot {
        //     let c = Color::from_gray(0.5);
        //     let vp = self.viewport_size;
        //     let center = Vec2::new(vp.x as f32 / 2.0, vp.y as f32 / 2.0);
        //     graphics.draw_line(self.mouse, center, 2.0, c);
        //     robot.draw(&Rect::new(center, center + Vec2::new(50.0, 50.0)), graphics);
        // }

        if let Some(image_handle) = &self.loaded_images.get(&self.selected) {
            let size = image_handle.size();
            let bounds = Rect::new(Vec2::ZERO, Vec2::new(size.x as f32, size.y as f32));
            graphics.draw_rectangle_image(self.camera.transform(&bounds), image_handle);
        };
    }
}
