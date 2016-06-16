#[macro_use]

use glium;
use glium::glutin;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use teapot;
use image;
use glium::backend::glutin_backend;
use glium::Surface;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

pub struct Settings<'a> {
    program: glium::Program,
    draw_params: glium::DrawParameters<'a>,
    rot: f32,
}

implement_vertex!(Vertex, position);

pub fn render<'a>(display: &glutin_backend::GlutinFacade, settings: &Settings<'a>) {

    use glium::{DisplayBuild, Surface};
    use std::io::Cursor;
    let image = image::load(Cursor::new(&include_bytes!("/home/linucc/pictures/core.png")[..]),
                        image::PNG).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);

    let mut vertex_shader_src = read_file("vertex_shader.shader");
    let mut fragment_shader_src = read_file("fragment_shader.shader");

    let vertex1 = Vertex {position: [-0.5, -0.5]};
    let vertex2 = Vertex {position: [0.0, 0.5]};
    let vertex3 = Vertex {position: [0.5, -0.25]};
    let shape = vec![vertex1, vertex2, vertex3];

    let light = [1.0, 0.4, 0.9];

    let program = glium::Program::from_source(
            display, &vertex_shader_src, &fragment_shader_src, None
        ).unwrap();

    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };

    let positions = glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(display, &shape).unwrap();

    let texture = glium::texture::Texture2d::new(display, image).unwrap();

    let mut t: f32 = -0.5;
    let mut tx: f32 = 0.005;
    let mut rot: f32 = t;

    let mut fullscreen = false;

    loop {
        if t > 1.0 || t < -1.0 {
            tx = -tx;
        }
        t += tx;
        rot = (t*t) * 30.0;
        println!("{:?}", t);
        let matrix = [
            [rot.cos() * 0.01, rot.sin() * 0.01, 0.0, 0.0],
            [-rot.sin() * 0.01, rot.cos() * 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 1.0f32],
        ];

        let mut target = display.draw();
        target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);


        // let indices = glium::index::NoIndices(
        //     glium::index::PrimitiveType::TrianglesList
        // );

        let perspective_matrix = perspective_matrix(&target);
        let view_matrix = view_matrix(
            &[2.0, -1.0, 1.0], &[-2.0, 1.0, 1.0], &[0.0, 1.0, 0.0]
        );

        let uniforms = uniform! {
            model: matrix, u_light: light, perspective: perspective_matrix,
            view: view_matrix
        };

        target.draw(
            (&positions, &normals), &indices, &program, &uniforms, &params
        ).unwrap();

        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glutin::Event::Closed => return,
                glutin::Event::KeyboardInput(
                    glutin::ElementState::Pressed, _,
                    Some(glutin::VirtualKeyCode::Return)
                ) => {
                    if fullscreen {
                        glium::glutin::WindowBuilder::new()
                            .rebuild_glium(&display)
                            .unwrap();
                        fullscreen = false;
                    }
                    else {
                        glium::glutin::WindowBuilder::new()
                            .with_fullscreen(glutin::get_primary_monitor())
                            .rebuild_glium(&display)
                            .unwrap();
                        fullscreen = true;
                    }
                },
                _ => ()
            }
        }
    }
}

pub fn render_new<'a>(display: &glutin_backend::GlutinFacade, settings: &mut Settings<'a>) {
    settings.rot += 0.1;
    let mut target = display.draw();
    let light = [1.4, 0.4, -0.7f32];
    let positions = glium::VertexBuffer::new(display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();
    target.clear_color_and_depth((0.1, 0.1, 0.1, 1.0), 1.0);
    // let indices = glium::index::NoIndices(
    //     glium::index::PrimitiveType::TrianglesList
    // );
    let view_matrix = view_matrix(
        &[2.0, 1.0, 1.0], &[-2.0, -1.0, 1.0], &[0.0, 1.0, 0.0]
    );
    let uniforms = uniform! {
        model: model_matrix(settings.rot),
        view: view_matrix,
        perspective: perspective_matrix(&target),
        u_light: light,
    };

    target.draw(
        (&positions, &normals), &indices, &settings.program, &uniforms,
        &settings.draw_params
    ).unwrap();

    target.finish().unwrap();
}

pub fn init<'a>(display: &glutin_backend::GlutinFacade) -> Settings<'a> {
    let vertex_shader_src = read_file("vertex_shader.shader");
    let fragment_shader_src = read_file("fragment_shader.shader");
    Settings {
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
    }
}

fn model_matrix(rot: f32) -> [[f32; 4]; 4] {
    let rot_norm = rot % 1.0;
    let bouncy: f32 = if rot % 2.0 > 1.0 {
        rot_norm
    } else {
        1.0 - rot_norm
    };

    [
        [(bouncy / 100.) + 0.01, 0.0, 0.0, 0.0],
        [0.0, 0.01, 0.0, 0.0],
        [0.0, 0.0, 0.01, 0.0],
        [0.0, 0.0, 2.0, 1.0],
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

