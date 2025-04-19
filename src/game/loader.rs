use speedy2d::{image::ImageSmoothingMode, Graphics2D};

use super::world::{EntityId, World};

pub struct Loader {
    queue: Vec<EntityId>,
}

impl Loader {
    pub fn new() -> Self {
        Self { queue: Vec::new() }
    }

    pub fn load(&mut self, id: EntityId) {
        if !self.queue.contains(&id) {
            self.queue.push(id);
        }
    }

    pub fn update(&mut self, world: &mut World, graphics: &mut Graphics2D) {
        if let Some(id) = self.queue.pop() {
            if world.get_image(id).is_none() {
                if let Some(path) = world.get_path(id) {
                    println!("Loading image {path:?}");
                    if let Ok(image) = graphics.create_image_from_file_path(
                        None,
                        ImageSmoothingMode::Linear,
                        path,
                    ) {
                        world.insert_image(id, image);
                    }
                }
            }
        }
    }
    
    pub fn is_idle(&self) -> bool {
        self.queue.is_empty()
    }
}
