use glam::{UVec2, Vec2};
use speedy2d::window::VirtualKeyCode;
use speedy2d::{color::Color, Graphics2D, Rect};
use walkdir::WalkDir;

use crate::app::{Keyboard, Mouse};
use crate::config::Config;

mod camera;
use crate::game::camera::Camera;

mod world;
use crate::game::world::World;

mod task_manager;
use crate::game::task_manager::TaskManager;

pub struct Game {
    config: Config,
    world: World,
    task_manager: TaskManager,

    selected: usize,
    camera: Camera,
    mouse: Mouse,

    counter: usize,
    viewport_size: UVec2,
    inverted: bool,
}

impl Game {
    pub fn new(config: Config) -> Self {
        let supported_extensions = vec![
            "png", "jpg", "jpeg", "gif", "bmp", "ico", "tiff", "tif", "webp", "avif", "pbm", "pgm",
            "ppm", "pnm", "dds", "tga", "ff",
        ];

        let mut world = World::new();
        let viewport_size = UVec2::new(config.window_width, config.window_height);
        let paths: Vec<&str> = vec![&config.input];
        println!("Reading {} asset paths:", paths.len());
        for path in paths {
            let mut count = 0;
            print!("In {path}: ");
            for entry in WalkDir::new(path)
                .follow_links(true)
                .max_depth(1)
                .sort_by_file_name()
                .into_iter()
                .filter_map(Result::ok)
            {
                if entry.file_type().is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if supported_extensions.contains(&ext.to_string_lossy().as_ref()) {
                            count += 1;
                            world.spawn_asset(entry.into_path());
                        }
                    }
                }
            }
            println!("{count} assets.");
        }

        if world.len() == 0 {
            println!("No images to display, exiting");
            std::process::exit(0);
        }

        Self {
            config,
            world,
            task_manager: TaskManager::new(4),
            selected: 0,
            camera: Camera::new(),

            mouse: Mouse::new(),

            counter: 0,
            viewport_size,
            inverted: false,
        }
    }

    pub fn setup(&mut self, _graphics: &mut Graphics2D) {}

    pub fn input(&mut self, viewport_size: UVec2, mouse: &Mouse, keyboard: &Keyboard) {
        self.viewport_size = viewport_size;
        let mouse_delta = mouse.position - self.mouse.position;
        let scroll_delta = mouse.scroll_accumulated - self.mouse.scroll_accumulated;
        self.mouse = mouse.clone();

        self.camera
            .handle_input(mouse, mouse_delta, scroll_delta, keyboard);
        if keyboard.just_pressed.contains(&VirtualKeyCode::E) {
            self.selected += 1;
            if self.selected >= self.world.len() {
                self.selected = 0;
            }
            println!("Selecting image {}", self.selected);
        }
        if keyboard.just_pressed.contains(&VirtualKeyCode::Q) {
            if self.selected == 0 {
                self.selected = self.world.len();
            }
            // no images in folder
            if self.selected == 0 {
                return;
            }
            self.selected -= 1;
            println!("Selecting image {}", self.selected);
        }
        if keyboard.just_pressed.contains(&VirtualKeyCode::R) {
            self.inverted = !self.inverted;
            println!("Inverted: {}", self.inverted);
        }
    }

    pub fn update(&mut self, graphics: &mut Graphics2D, _current_frame: u64) {
        let mut preload_ids = vec![self.selected];

        if self.selected > 0 {
            preload_ids.push(self.selected - 1);
        }
        if self.selected + 1 < self.world.len() {
            preload_ids.push(self.selected + 1);
        }
        if self.selected + 2 < self.world.len() {
            preload_ids.push(self.selected + 2);
        }

        // Request loading if needed
        for id in preload_ids {
            if self.world.get_image(id).is_none() {
                if let Some(path) = self.world.get_path(id) {
                    self.task_manager.load(id, path.clone());
                }
            }
        }

        // Apply completed tasks
        self.task_manager.update(&mut self.world, graphics);

        if self.world.get_image(self.selected).is_none() {
            if let Some(path) = self.world.get_path(self.selected) {
                self.task_manager.load(self.selected, path.clone());
            }
        }
        let next_selected = self.selected + 1;
        if self.task_manager.is_idle()
            && next_selected < self.world.len()
            && self.world.get_image(next_selected).is_none()
        {
            if let Some(path) = self.world.get_path(next_selected) {
                self.task_manager.load(next_selected, path.clone());
            }
        }

        self.task_manager.update(&mut self.world, graphics);
    }

    pub fn draw(&self, graphics: &mut Graphics2D) {
        // if let Some(robot) = &self.robot {
        //     let c = Color::from_gray(0.5);
        //     let vp = self.viewport_size;
        //     let center = Vec2::new(vp.x as f32 / 2.0, vp.y as f32 / 2.0);
        //     graphics.draw_line(self.mouse, center, 2.0, c);
        //     robot.draw(&Rect::new(center, center + Vec2::new(50.0, 50.0)), graphics);
        // }

        if let Some(image_handle) = &self.world.get_image(self.selected) {
            let size = image_handle.size();
            let bounds = Rect::new(Vec2::ZERO, Vec2::new(size.x as f32, size.y as f32));

            if self.inverted {
                // TODO: atm its making the image less bright instead. 
                graphics.draw_rectangle_image_tinted(
                    self.camera.transform(&bounds),
                    Color::from_gray(0.5),
                    image_handle,
                );
            } else {
                graphics.draw_rectangle_image(self.camera.transform(&bounds), image_handle);
            }
        };
    }
}
