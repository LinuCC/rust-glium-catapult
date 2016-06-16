#[macro_use]
extern crate glium;
extern crate image;

mod render;
mod teapot;

use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend;

fn main() {
    let display: glutin_backend::GlutinFacade = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();
    let mut settings: render::Settings = render::init(&display);
    // render::render(&display, &settings);
    let mut fullscreen = false;

    loop {
        render::render_new(&display, &mut settings);
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

