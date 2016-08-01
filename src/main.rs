#[macro_use]
extern crate glium;
extern crate image;
extern crate quaternion;
extern crate vecmath;

mod render;
mod catapult;
mod camera;
mod matrix;
mod state;
mod drawable;

use glium::glutin;
use glium::DisplayBuild;
use glium::backend::glutin_backend;

/*
 *  Remember, remember: matrices rows are columns when defined as an array!
 */

/**
 * Renders something of a catapult in yo' face.
 *
 * Use WASD, Arrow keys and Q and E to control the camera.
 * Use Space and Backspace to see some animations. Yay!
 */
fn main() {
    let display: glutin_backend::GlutinFacade = glutin::WindowBuilder::new()
        .with_depth_buffer(24)
        .build_glium().unwrap();
    let mut settings: state::Settings = render::init(&display);
    let mut fullscreen = false;

    loop {
        render::render(&display, &settings);
        let keyboard_events = display.poll_events().collect::<Vec<_>>();
        for renderable in settings.objects.iter_mut() {
            renderable.update(&keyboard_events);
        }
        settings.camera.update();
        for ev in keyboard_events {
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

