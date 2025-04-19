use std::collections::HashMap;
use std::path::PathBuf;
use speedy2d::image::ImageHandle;

pub type EntityId = usize;

pub struct World {
    next_id: EntityId,
    assets: HashMap<EntityId, PathBuf>,
    images: HashMap<EntityId, ImageHandle>,
}

impl World {
    pub fn len(&self) -> usize {
        self.next_id
    }

    pub fn spawn_asset(&mut self, path: PathBuf) -> EntityId {
        let id = self.next_id;
        self.next_id += 1;
        self.assets.insert(id, path);
        id
    }

    pub fn insert_image(&mut self, id: EntityId, image: ImageHandle) {
        self.images.insert(id, image);
    }

    pub fn get_path(&self, id: EntityId) -> Option<&PathBuf> {
        self.assets.get(&id)
    }

    pub fn get_image(&self, id: EntityId) -> Option<&ImageHandle> {
        self.images.get(&id)
    }

    pub fn new() -> Self {
        Self {
            next_id: 0,
            assets: HashMap::new(),
            images: HashMap::new(),
        }
    }
}
