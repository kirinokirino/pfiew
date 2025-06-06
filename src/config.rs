use configparser::ini::Ini;

use std::default::Default;
use std::error::Error;

#[derive(Debug)]
pub struct Config {
    path: Option<String>,
    pub title: String,
    pub input: String,
    pub sleep_ms_per_frame: u64,
    pub window_width: u32,
    pub window_height: u32,
}

impl Config {
    pub fn new(path: &str) -> Self {
        let mut config = Self {
            path: Some(path.to_string()),
            ..Default::default()
        };
        if let Err(err) = config.reload() {
            eprintln!("{err}");
        }
        config
    }

    pub fn reload(&mut self) -> Result<(), Box<dyn Error>> {
        let mut ini = Ini::new();
        match &self.path {
            Some(path) => {
                if ini.load(path).is_ok() {
                    let default_section = "default";
                    if let Some(input) = ini.get(default_section, "input") {
                        self.input = input;
                    }
                    if let Some(title) = ini.get(default_section, "title") {
                        self.title = title;
                    }
                    if let Some(sleep) = ini.getuint(default_section, "sleep_ms_per_frame")? {
                        self.sleep_ms_per_frame = sleep;
                    }
                    if let Some(width) = ini.getuint(default_section, "window_width")? {
                        self.window_width = width.try_into()?;
                    }
                    if let Some(height) = ini.getuint(default_section, "window_height")? {
                        self.window_height = height.try_into()?;
                    }
                }
            }
            None => return Err("Tried to reload config with no set path!".into()),
        }
        Ok(())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: None,
            input: "assets/folder".to_string(),
            title: "pfiew".to_string(),
            sleep_ms_per_frame: 5,
            window_width: 640,
            window_height: 640,
        }
    }
}
