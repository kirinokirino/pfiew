use std::collections::VecDeque;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use crossbeam_channel::{unbounded, Receiver, Sender};
use glam::UVec2;
use image::ImageReader;

use crate::game::world::{EntityId, World};
use speedy2d::{
    image::{ImageDataType, ImageSmoothingMode},
    Graphics2D,
};

enum Task {
    Load { id: EntityId, path: PathBuf },
}

enum TaskResult {
    DecodedImage {
        id: EntityId,
        bytes: Vec<u8>,
        width: u32,
        height: u32,
    },
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

        let task_receiver = Arc::new(task_receiver);
        let result_sender = Arc::new(result_sender);

        let thread_count = 1; // or however many you want
        for _ in 0..thread_count {
            let task_receiver = Arc::clone(&task_receiver);
            let result_sender = Arc::clone(&result_sender);

            thread::spawn(move || {
                while let Ok(task) = task_receiver.recv() {
                    match task {
                        Task::Load { id, path } => {
                            if let Ok(reader) = ImageReader::open(&path) {
                                let image = reader.decode();
                                match image {
                                    Ok(image) => {
                                        // Convert to raw RGBA bytes
                                        let rgba = image.to_rgba8();
                                        let (width, height) = rgba.dimensions();
                                        let bytes = rgba.into_raw();

                                        let _ = result_sender.send(TaskResult::DecodedImage {
                                            id,
                                            bytes,
                                            width,
                                            height,
                                        });
                                    }
                                    Err(err) => {
                                        eprintln!("Failed to decode image: {err}");
                                    }
                                }
                            }
                        }
                    }
                }
            });
        }

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
                TaskResult::DecodedImage {
                    id,
                    bytes,
                    width,
                    height,
                } => {
                    if let Ok(image) = graphics.create_image_from_raw_pixels(
                        ImageDataType::RGBA,
                        ImageSmoothingMode::Linear,
                        UVec2::new(width, height),
                        &bytes,
                    ) {
                        world.insert_image(id, image);
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
