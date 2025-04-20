use std::io::Cursor;
use std::thread;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use crossbeam_channel::{unbounded, Sender, Receiver};

use speedy2d::{Graphics2D, image::{ImageDataType, ImageFileFormat, ImageSmoothingMode}};
use crate::game::world::{EntityId, World};

enum Task {
    Load { id: EntityId, path: PathBuf },
}

enum TaskResult {
    Loaded { id: EntityId, bytes: Vec<u8> },
}

pub struct TaskManager {
    sender: Sender<Task>,
    result_receiver: Receiver<TaskResult>,
    queue: VecDeque<EntityId>,
}

impl TaskManager {
    pub fn new() -> Self {
        let (task_sender, task_receiver) = unbounded::<Task>();
        let (result_sender, result_receiver) = unbounded::<TaskResult>();

        // Spawn loader thread
        thread::spawn(move || {
            while let Ok(task) = task_receiver.recv() {
                match task {
                    Task::Load { id, path } => {
                        if let Ok(bytes) = std::fs::read(&path) {
                            let _ = result_sender.send(TaskResult::Loaded { id, bytes });
                        } else {
                            eprintln!("Failed to load image: {:?}", path);
                        }
                    }
                }
            }
        });

        Self {
            sender: task_sender,
            result_receiver,
            queue: VecDeque::new(),
        }
    }

    pub fn load(&mut self, id: EntityId, path: PathBuf) {
        if self.queue.contains(&id) {
            return;
        }
        let _ = self.sender.send(Task::Load { id, path });
        self.queue.push_back(id);
    }

    pub fn update(&mut self, world: &mut World, graphics: &mut Graphics2D) {
        while let Ok(result) = self.result_receiver.try_recv() {
            match result {
                TaskResult::Loaded { id, bytes } => {
                    if let Ok(handle) = graphics.create_image_from_file_bytes(
                        None, // or auto-detect if you prefer
                        ImageSmoothingMode::Linear,
                        Cursor::new(bytes),
                    ) {
                        world.insert_image(id, handle);
                    }
                    self.queue.retain(|queued_id| *queued_id != id);
                }
            }
        }
    }

    pub fn is_idle(&self) -> bool {
        self.queue.is_empty()
    }
}
