use glium;
use camera;
use render::{perspective_matrix};
use drawable::{Drawable};

/**
 * Contains the global state of the program.
 */
pub struct Settings<'a> {
    pub program: glium::Program,
    pub draw_params: glium::DrawParameters<'a>,
    pub camera: camera::CameraState,
    pub objects: Vec<Box<Drawable>>,
    pub light: [f32; 3],
}

impl<'a> Settings<'a> {
    pub fn perspective_matrix(&self, target: &glium::Frame) -> [[f32; 4]; 4] {
        perspective_matrix(target)
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.camera.set_aspect_ratio(aspect_ratio);
    }
}

