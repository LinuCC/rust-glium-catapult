use glium;
use glium::Surface;
use state::Settings;
use std::rc::Rc;
use matrix::mul_matrices;

/**
 * Defines Entity-representations for drawable things
 */

pub struct RenderData<V, N, I> where
    V: glium::vertex::Vertex, N: glium::vertex::Vertex, I: glium::index::Index
{
    pub positions: glium::VertexBuffer<V>,
    pub normals: glium::VertexBuffer<N>,
    pub indices: glium::IndexBuffer<I>,
}

/**
 * Defines traits for some drawable thing
 */
pub trait Drawable {
    /**
     * Draws the drawable. The world_matrix will be used to transform the
     * model-matrix, can be used to pass transformations of the parents
     */
    fn draw(&self, settings: &Settings, target: &mut glium::Frame, world_matrix: [[f32; 4]; 4])
        -> Result<(), glium::DrawError>;
    /**
     * Updates the Drawable, gets called every tick
     */
    fn update<'a>(&mut self, keyboard_events: &Vec<glium::glutin::Event>);
}

/**
 * Represents a drawable entity
 */
pub struct DrawObject<V, N, I> where
    V: glium::vertex::Vertex, N: glium::vertex::Vertex, I: glium::index::Index
{
    pub data: RenderData<V, N, I>,
    pub model_matrix: [[f32; 4]; 4],
    pub texture: Rc<glium::texture::Texture2d>,
    pub children: Vec<DrawObject<V, N, I>>,
}

impl<V, N, I> Drawable for DrawObject<V, N, I> where
    V: glium::vertex::Vertex,
    N: glium::vertex::Vertex,
    I: glium::index::Index,
{
    fn update<'b>(&mut self, _: &Vec<glium::glutin::Event>) {

    }

    fn draw(
            &self,
            settings: &Settings,
            target: &mut glium::Frame,
            world_matrix: [[f32; 4]; 4]
    )
        -> Result<(), glium::DrawError>
    {
        let context_matrix = mul_matrices(world_matrix, self.model_matrix);
        let uniforms = uniform! {
            model: context_matrix,
            view: settings.camera.get_view(),
            perspective: settings.perspective_matrix(&target),
            u_light: settings.light,
            tex: &*self.texture
        };
        let res = target.draw(
            (&self.data.positions, &self.data.normals), &self.data.indices,
            &settings.program, &uniforms, &settings.draw_params
        );
        if !self.children.is_empty() {
            for i in self.children.iter() {
                i.draw(settings, target, context_matrix).unwrap();
            }
        }
        return res;
    }
}
