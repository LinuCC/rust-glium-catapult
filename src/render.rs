#[macro_use]

use glium;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use teapot;
use catapult;
use image;
use glium::backend::glutin_backend;
use glium::Surface;
use camera;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: (f32, f32, f32),
}

#[derive(Copy, Clone)]
pub struct Normal {
    pub normal: (f32, f32, f32)
}

implement_vertex!(Vertex, position);
implement_vertex!(Normal, normal);

pub struct Settings<'a> {
    pub program: glium::Program,
    pub draw_params: glium::DrawParameters<'a>,
    pub camera: camera::CameraState,
    light: [f32; 3],
    pub rot: f32,
}

impl<'a> Settings<'a> {
    fn perspective_matrix(&self, target: &glium::Frame) -> [[f32; 4]; 4] {
        perspective_matrix(target)
    }

    fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.camera.set_aspect_ratio(aspect_ratio);
    }
}

struct Camera {
    position: [f32; 3],
    direction: [f32; 3],
    up: [f32; 3],
}

pub struct RenderData<V, N, I> where
    V: glium::vertex::Vertex, N: glium::vertex::Vertex, I: glium::index::Index
{
    pub positions: glium::VertexBuffer<V>,
    pub normals: glium::VertexBuffer<N>,
    pub indices: glium::IndexBuffer<I>,
}

pub struct DrawObject<V, N, I> where
    V: glium::vertex::Vertex, N: glium::vertex::Vertex, I: glium::index::Index
{
    pub data: RenderData<V, N, I>,
    pub model_matrix: [[f32; 4]; 4],
}

impl<V, N, I> DrawObject<V, N, I> where
    V: glium::vertex::Vertex,
    N: glium::vertex::Vertex,
    I: glium::index::Index,
{
    fn draw(&self, settings: &Settings, target: &mut glium::Frame) {
        let uniforms = uniform! {
            model: self.model_matrix,
            view: settings.camera.get_view(),
            perspective: settings.perspective_matrix(&target),
            u_light: settings.light,
        };
        target.draw(
            (&self.data.positions, &self.data.normals), &self.data.indices,
            &settings.program, &uniforms, &settings.draw_params
        ).unwrap();
    }
}

pub fn render<'a>(display: &glutin_backend::GlutinFacade, settings: &mut Settings<'a>) {
    settings.rot += 0.1;
    let mut target = display.draw();
    target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);
    let teapot = init_teapot(&display, &settings);
    let catapult = catapult::init_catapult(&display, &settings);

    // teapot.draw(&settings, &mut target);
    catapult.draw(&settings, &mut target);

    target.finish().unwrap();
}

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
        rot: 0.0,
        camera: camera::CameraState::new(),
        light: [1.4, 0.4, -0.7f32],
    };

    let (width, height): (u32, u32) = target.get_dimensions();
    let aspect_ratio = height as f32 / width as f32;
    settings.set_aspect_ratio(aspect_ratio);

    target.finish().unwrap();
    settings
}

fn init_teapot(display: &glutin_backend::GlutinFacade, settings: &Settings)
    -> DrawObject<teapot::Vertex, teapot::Normal, u16>
{
    let positions = glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES
    ).unwrap();
    DrawObject {
        data: RenderData {
            positions: positions,
            normals: normals,
            indices: indices,
        },
        model_matrix: model_matrix(settings.rot),
    }
}

pub fn model_matrix(rot: f32) -> [[f32; 4]; 4] {
    let rot_norm = rot % 1.0;
    let bouncy: f32 = if rot % 2.0 > 1.0 {
        rot_norm
    } else {
        1.0 - rot_norm
    };

    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn perspective_matrix(target: &glium::Frame) -> [[f32; 4]; 4] {
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

fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
    let f = {
        let f = direction;
        let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
        let len = len.sqrt();
        [f[0] / len, f[1] / len, f[2] / len]
    };

    let s = [up[1] * f[2] - up[2] * f[1],
             up[2] * f[0] - up[0] * f[2],
             up[0] * f[1] - up[1] * f[0]];

    let s_norm = {
        let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
        let len = len.sqrt();
        [s[0] / len, s[1] / len, s[2] / len]
    };

    let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
             f[2] * s_norm[0] - f[0] * s_norm[2],
             f[0] * s_norm[1] - f[1] * s_norm[0]];

    let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
             -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
             -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];

    [
        [s_norm[0], u[0], f[0], 0.0],
        [s_norm[1], u[1], f[1], 0.0],
        [s_norm[2], u[2], f[2], 0.0],
        [p[0], p[1], p[2], 1.0],
    ]
}

fn read_file(path: &str) -> String {
    let mut string = String::new();
    match File::open(path) {
        Ok(mut file) => file.read_to_string(&mut string).unwrap(),
        Err(error) => panic!("Error loading the file {}", path),
    };
    string
}

