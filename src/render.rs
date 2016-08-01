use glium;
use std::fs::File;
use std::io::prelude::*;
use catapult;
use glium::backend::glutin_backend;
use glium::Surface;
use camera;
use state::Settings;

pub const DEFAULT_MATRIX: [[f32; 4]; 4] = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

/**
 * Initialize rendering (& Settings). Probably should be partially outsourced.
 */
pub fn init<'a>(display: &glutin_backend::GlutinFacade) -> Settings<'a> {
    let vertex_shader_src = read_file("vertex_shader.shader");
    let fragment_shader_src = read_file("fragment_shader.shader");
    // Draw one time to get the target to create the initial perspective_matrix
    let target = display.draw();
    let mut settings = Settings {
        program: glium::Program::from_source(
            display, &vertex_shader_src, &fragment_shader_src, None
        ).unwrap(),
        draw_params: glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            .. Default::default()
        },
        camera: camera::CameraState::new(),
        light: [1.4, 0.4, -0.7f32],
        objects: Vec::new()
    };

    let (width, height): (u32, u32) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;
    settings.set_aspect_ratio(aspect_ratio);
    settings.objects = vec![catapult::init_catapult(&display, &settings)];

    target.finish().unwrap();
    settings
}

/**
 * Renders the whole scene.
 */
pub fn render<'a>(display: &glutin_backend::GlutinFacade, settings: &Settings<'a>) {
    let mut target = display.draw();
    target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);
    settings.objects[0].draw(&settings, &mut target, DEFAULT_MATRIX).unwrap();

    target.finish().unwrap();
}

pub fn perspective_matrix(target: &glium::Frame) -> [[f32; 4]; 4] {
    use glium::Surface;
    let (width, height): (u32, u32) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;
    let fov: f32 = 3.141592 / 3.0;
    let zfar = 1024.0;
    let znear = 0.1;
    let f = 1.0 / (fov / 2.0).tan();
    [
        [f * aspect_ratio, 0.0, 0.0, 0.0],
        [0.0, f, 0.0, 0.0],
        [0.0, 0.0, (zfar + znear) / (zfar - znear), 1.0],
        [0.0, 0.0, -(2.0 * zfar * znear) / (zfar - znear), 0.0],
    ]
}

fn read_file(path: &str) -> String {
    let mut string = String::new();
    match File::open(path) {
        Ok(mut file) => file.read_to_string(&mut string).unwrap(),
        Err(_) => panic!("Error loading the file {}", path),
    };
    string
}

