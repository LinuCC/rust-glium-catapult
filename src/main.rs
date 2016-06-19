#[macro_use]
extern crate glium;
extern crate image;

mod render;
mod teapot;
mod catapult;
mod camera;

use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend;

fn main() {
    let display: glutin_backend::GlutinFacade = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();
    let mut settings: render::Settings = render::init(&display);
    let mut fullscreen = false;

    loop {
        render::render(&display, &mut settings);
        settings.camera.update();
        for ev in display.poll_events() {
            settings.camera.process_input(&ev);
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

