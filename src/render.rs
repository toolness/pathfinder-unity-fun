use std::path::PathBuf;

use crate::logging::log;
use crate::gl_util::get_viewport_size;

pub struct Renderer {
    resources_dir: PathBuf
}

impl Renderer {
    pub fn new(resources_dir: PathBuf) -> Self {
        log(format!("Initializing renderer with viewport {:?}.", get_viewport_size()));
        Renderer { resources_dir }
    }

    pub fn render(&mut self) {
        let _ = self.resources_dir;
    }
}
