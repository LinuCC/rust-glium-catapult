use render::*;
use glium;
use glium::backend::glutin_backend;


pub fn init_catapult(display: &glutin_backend::GlutinFacade, settings: &Settings)
    -> DrawObject<Vertex, Normal, u16>
{
    let positions = glium::VertexBuffer::new(display, &BASE_VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(display, &BASE_NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(
        display, glium::index::PrimitiveType::TrianglesList, &BASE_INDICES
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

const BASE_VERTICES: [Vertex; 8] = [
    /*
     * Fat plank #1
     */
    // Bottom
    Vertex { position: (0.0, 0.0, 0.0) },
    Vertex { position: (7.5, 0.0, 0.0) },
    Vertex { position: (7.5, 0.0, 0.5) },
    Vertex { position: (0.0, 0.0, 0.5) },
    // Top
    Vertex { position: (0.0, 1.0, 0.0) },
    Vertex { position: (7.5, 1.0, 0.0) },
    Vertex { position: (7.5, 1.0, 0.5) },
    Vertex { position: (0.0, 1.0, 0.5) },
];

const BASE_NORMALS: [Normal; 0] = [

];

 const BASE_INDICES: [u16; 12] = [
    /*
     * Fat plank #1
     */
    // Bottom
    0, 1, 2,
    0, 3, 2,
    // Right
    0, 1, 5,
    0, 4, 5,
 ];
