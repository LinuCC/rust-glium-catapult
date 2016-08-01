use render::*;
use glium;
use glium::backend::glutin_backend;
use glium::Surface;
use glium::VertexBuffer;
use glium::IndexBuffer;
use glium::index::PrimitiveType::TrianglesList;
use glium::glutin;
use quaternion::*;
use vecmath::Vector3;
use matrix::{mul_matrices, rot_matrix_by};
use std::f32::consts::PI;
use quaternion;
use std::rc::Rc;

#[derive(Copy, Clone)]
pub struct Vertex {
    position: (f32, f32, f32),
    tex_coords: [f32; 2],
}
implement_vertex!(Vertex, position, tex_coords);

fn normalize(vec: [f32; 3]) -> [f32; 3] {
    let len = vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2];
    let len = len.sqrt();
    [vec[0] / len, vec[1] / len, vec[2] / len]
}


pub struct Catapult<V, N, I> where
    V: glium::vertex::Vertex, N: glium::vertex::Vertex, I: glium::index::Index
{
    throw_rotation: f32,
    throw_rotation_limits: (f32, f32),
    throwing: bool,
    winding_throw_up: bool,
    fat_plank_right: DrawObject<V, N, I>,
    fat_plank_left: DrawObject<V, N, I>,
    standup_plank_right: DrawObject<V, N, I>,
    standup_plank_left: DrawObject<V, N, I>,
    stopper_plank: DrawObject<V, N, I>,
    standup_strut_right: DrawObject<V, N, I>,
    standup_strut_left: DrawObject<V, N, I>,
    throw_arm: DrawObject<V, N, I>,
    winder: DrawObject<V, N, I>,
    model_matrix: [[f32; 4]; 4],
}

impl<V, N, I> Drawable for Catapult<V, N, I> where
    V: glium::vertex::Vertex, N: glium::vertex::Vertex, I: glium::index::Index
{
    fn update<'a>(&mut self, keyboard_events: &Vec<glium::glutin::Event>) {
        for ev in keyboard_events {
            match ev {
                &glutin::Event::KeyboardInput(
                    glutin::ElementState::Pressed, _,
                    Some(glutin::VirtualKeyCode::Space)
                ) => {
                    self.throwing = true;
                },
                &glutin::Event::KeyboardInput(
                    glutin::ElementState::Pressed, _,
                    Some(glutin::VirtualKeyCode::Back)
                ) => {
                    self.winding_throw_up = true;
                },
                _ => {}
            }
        }

        if self.winding_throw_up {
            if self.throw_rotation > self.throw_rotation_limits.0 + 0.01 {
                self.throw_rotation -= 0.01;
            }
            else {
                self.winding_throw_up = false;
            }
        }
        else if self.throwing {
            if self.throw_rotation < self.throw_rotation_limits.1 - 0.15 {
                self.throw_rotation += 0.15;
            }
            else {
                self.throwing = false;
            }
        }
    }

    fn draw(&self, settings: &Settings, target: &mut glium::Frame, world_matrix: [[f32; 4]; 4])
        -> Result<(), glium::DrawError>
    {
        let context = mul_matrices(world_matrix, self.model_matrix);
        self.fat_plank_right.draw(settings, target, context).unwrap();
        self.fat_plank_left.draw(settings, target, context).unwrap();
        self.standup_plank_right.draw(settings, target, context).unwrap();
        self.standup_plank_left.draw(settings, target, context).unwrap();
        self.standup_strut_right.draw(settings, target, context).unwrap();
        self.standup_strut_left.draw(settings, target, context).unwrap();
        self.stopper_plank.draw(settings, target, context).unwrap();
        let rot = quaternion::axis_angle(
            normalize([0.0, 0.0, 1.0]) as Vector3<f32>, self.throw_rotation
        );
        self.throw_arm.draw(
            settings, target,
            mul_matrices(context, rot_matrix_by(&rot, [3.65, 0.5, 0.0]))
        ).unwrap();
        let rot = quaternion::axis_angle(
            normalize([0.0, 0.0, 1.0]) as Vector3<f32>, self.throw_rotation * 4.0
        );
        self.winder.draw(
            settings, target,
            mul_matrices(context, rot_matrix_by(&rot, [0.5, 0.8, -0.125]))
        ).unwrap();
        Ok(())
    }
}

pub fn init_catapult(display: &glutin_backend::GlutinFacade, settings: &Settings)
    -> Box<Drawable>
{
    use std::io::Cursor;
    use image;
    let image = image::load(
        Cursor::new(&include_bytes!("../wood.png")[..]), image::PNG
    ).unwrap().to_rgba();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        image.into_raw(),
        image_dimensions
    );
    let texture = Rc::new(
        glium::texture::Texture2d::new(display, image).unwrap()
    );

    Box::new(Catapult {
        model_matrix: model_matrix(settings.rot),
        fat_plank_right: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((0.0, 0.0, 0.0), (7.5, 1.0, 0.5))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            model_matrix: model_matrix(settings.rot),
            texture: texture.clone(),
            children: Vec::new(),
        },
        fat_plank_left: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                        display, &gen_rectangle((0.0, 0.0, 3.0), (7.5, 1.0, 3.5))
                    ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                        display, TrianglesList, &RECTANGLE_INDICES
                    ).unwrap(),
            },
            model_matrix: model_matrix(settings.rot),
            texture: texture.clone(),
            children: Vec::new(),
        },
        standup_plank_right: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((4.0, 1.0, 0.0), (5.0, 3.75, 0.5))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            model_matrix: model_matrix(settings.rot),
            texture: texture.clone(),
            children: Vec::new(),
        },
        standup_plank_left: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((4.0, 1.0, 3.0), (5.0, 3.75, 3.5))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            model_matrix: model_matrix(settings.rot),
            texture: texture.clone(),
            children: Vec::new(),
        },
        stopper_plank: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((3.75, 3.0, -0.25), (4.5, 3.5, 3.75))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            model_matrix: model_matrix(settings.rot),
            texture: texture.clone(),
            children: Vec::new(),
        },
        standup_strut_right: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((5.5, 1.0, 0.1), (6.0, 4.0, 0.4))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            model_matrix: [
                [0.55f32.cos(), 0.55f32.sin(), 0.0, 0.0],
                [-0.55f32.sin(), 0.55f32.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [1.9, -3.0, 0.0, 1.0],
            ],
            texture: texture.clone(),
            children: Vec::new(),
        },
        standup_strut_left: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((5.5, 1.0, 3.1), (6.0, 4.0, 3.4))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            model_matrix: [
                [0.55f32.cos(), 0.55f32.sin(), 0.0, 0.0],
                [-0.55f32.sin(), 0.55f32.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [1.9, -3.0, 0.0, 1.0],
            ],
            texture: texture.clone(),
            children: Vec::new(),
        },
        throw_arm: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((3.5, 0.5, 1.5), (3.8, 5.15, 2.0))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            // model_matrix: model_matrix(settings.rot),
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            texture: texture.clone(),
            children: vec![
                DrawObject {
                    data: RenderData {
                        positions: VertexBuffer::new(
                            display, &gen_rectangle((3.25, 5.0, 1.25), (3.75, 6.0, 2.25))
                        ).unwrap(),
                        normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                        indices: IndexBuffer::new(
                            display, TrianglesList, &RECTANGLE_INDICES
                        ).unwrap(),
                    },
                    model_matrix: model_matrix(settings.rot),
                    texture: texture.clone(),
                    children: Vec::new(),
                },
                DrawObject {
                    data: RenderData {
                        positions: VertexBuffer::new(
                            display, &gen_rectangle((3.4, 0.25, -0.25), (3.9, 0.75, 3.75))
                        ).unwrap(),
                        normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                        indices: IndexBuffer::new(
                            display, TrianglesList, &RECTANGLE_INDICES
                        ).unwrap(),
                    },
                    model_matrix: model_matrix(settings.rot),
                    texture: texture.clone(),
                    children: Vec::new(),
                },
            ],
        },
        winder: DrawObject {
            data: RenderData {
                positions: VertexBuffer::new(
                    display, &gen_rectangle((0.2, 0.5, -0.25), (0.8, 1.1, 0.0))
                ).unwrap(),
                normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                indices: IndexBuffer::new(
                    display, TrianglesList, &RECTANGLE_INDICES
                ).unwrap(),
            },
            // model_matrix: model_matrix(settings.rot),
            model_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            texture: texture.clone(),
            children: vec![
                DrawObject {
                    data: RenderData {
                        positions: VertexBuffer::new(
                            display, &gen_rectangle((-0.3, 0.7, -0.2), (1.3, 0.9, -0.05))
                        ).unwrap(),
                        normals: VertexBuffer::new(display, &NO_NORMALS).unwrap(),
                        indices: IndexBuffer::new(
                            display, TrianglesList, &RECTANGLE_INDICES
                        ).unwrap(),
                    },
                    model_matrix: model_matrix(settings.rot),
                    texture: texture.clone(),
                    children: Vec::new(),
                },
            ],
        },
        throw_rotation: 0.0,
        throw_rotation_limits: (-(PI / 2.0) + (PI / 16.0), 0.0),
        throwing: false,
        winding_throw_up: false,
    })
}

/*
 *  Generate vertices of a rectangle by two given points
 */
pub fn gen_rectangle(min: (f32, f32, f32), max: (f32, f32, f32)) -> [Vertex; 8] {
    [
        // Bottom
        Vertex { position: (min.0, min.1, min.2), tex_coords: [0.0, 0.0] },
        Vertex { position: (max.0, min.1, min.2), tex_coords: [1.0, 0.0] },
        Vertex { position: (max.0, min.1, max.2), tex_coords: [1.0, 1.0] },
        Vertex { position: (min.0, min.1, max.2), tex_coords: [0.0, 1.0] },
        // Top
        Vertex { position: (min.0, max.1, min.2), tex_coords: [0.0, 0.0] },
        Vertex { position: (max.0, max.1, min.2), tex_coords: [1.0, 0.0] },
        Vertex { position: (max.0, max.1, max.2), tex_coords: [1.0, 1.0] },
        Vertex { position: (min.0, max.1, max.2), tex_coords: [0.0, 1.0] },
    ]
}

const NO_NORMALS: [Normal; 0] = [];

const RECTANGLE_INDICES: [u16; 36] = [
   // Bottom
   0, 1, 2,
   0, 3, 2,
   // Right
   0, 1, 5,
   0, 4, 5,
   // Top
   4, 5, 6,
   4, 7, 6,
   // Left
   3, 2, 6,
   3, 7, 6,
   // Front
   1, 5, 6,
   1, 2, 6,
   // Back
   0, 4, 7,
   0, 3, 7,
];
