#![warn(clippy::nursery, clippy::pedantic)]
#![allow(clippy::cast_precision_loss)]
#![windows_subsystem = "windows"]

use std::env::args;

use speedy2d::{
    window::{WindowCreationOptions, WindowPosition, WindowSize},
    Window,
};

use glam::u32::UVec2;

mod app;
use app::App;
mod config;
use config::Config;

mod game;

fn main() {
    let mut config = Config::new("config.ini");
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() {
        config.input = "./".to_string();
    } else {
        let arg = &args[0];
        if arg.contains("--input=") {
            let (_before, after) = arg.split_once("--input=").unwrap();
            after.clone_into(&mut config.input);
        } else {
            config.input = arg.to_string();
        }
    }

    let window_size = UVec2::new(config.window_width, config.window_height);
    let window_pixels = WindowSize::PhysicalPixels(window_size);
    let window = Window::new_with_options(
        &config.title,
        WindowCreationOptions::new_windowed(window_pixels, Some(WindowPosition::Center))
            .with_decorations(true)
            .with_transparent(false),
    )
    .expect("Wasn't able to create a window!");
    window.run_loop(App::new(window_size, config));
}
