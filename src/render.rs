use std::path::PathBuf;

pub struct Renderer {
    resources_dir: PathBuf
}

impl Renderer {
    pub fn new(resources_dir: PathBuf) -> Self {
        Renderer { resources_dir }
    }

    pub fn render(&mut self) {
        
    }
}
