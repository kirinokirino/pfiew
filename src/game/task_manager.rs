use std::collections::VecDeque;

use speedy2d::{image::ImageSmoothingMode, Graphics2D};

use super::world::{EntityId, World};

enum Task {
    Load { id: EntityId },
}

pub struct TaskManager {
    queue: VecDeque<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn load(&mut self, id: EntityId) {
            self.queue.push_front(Task::Load { id });
    }

    pub fn update(&mut self, world: &mut World, graphics: &mut Graphics2D) {
        if let Some(task) = self.queue.pop_front() {
            match task {
                Task::Load { id } => {
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
        }
    }

    pub fn is_idle(&self) -> bool {
        self.queue.is_empty()
    }
}
