extern crate glium;
use glium::glutin;
use std::f32::consts::PI;
use quaternion;
use vecmath;

pub struct CameraState {
    aspect_ratio: f32,
    position: (f32, f32, f32),
    direction: (f32, f32, f32),
    up: (f32, f32, f32),
    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
    moving_speed: f32,
    rotating_up: bool,
    rotating_left: bool,
    rotating_down: bool,
    rotating_right: bool,
    rotating_speed: f32,
}

fn normalize(vec: [f32; 3]) -> [f32; 3] {
    let len = vec[0] * vec[0] + vec[1] * vec[1] + vec[2] * vec[2];
    let len = len.sqrt();
    [vec[0] / len, vec[1] / len, vec[2] / len]
}

// fn rotate(vec: (f32, f32, f32), rot: (f32, f32, f32)) -> (f32, f32, f32) {
    // let vec_vec: vecmath::Vector3<f32> = normalize(vec);
    // let vec_rot: vecmath::Vector3<f32> = normalize(rot);
    // let q = quaternion::rotation_from_to(vec_vec, vec_rot);
    // let rotated_vec = quaternion::rotate_vector(q, vec_vec);
    // (rotated_vec[0], rotated_vec[1], rotated_vec[2])

    // Use Rodrigues formula
    // let x: f32 = 0.0 * vec.0 + -rot.2 * vec.1 + rot.1 * vec.2;
    // let y: f32 = rot.2 * vec.0 + 0.0 * vec.1 + -rot.0 * vec.2;
    // let z: f32 = -rot.1 * vec.0 + rot.0 * vec.1 + 0.0 * vec.2;
    // (x, y, z)

// }

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 1024.0 / 768.0,
            position: (-5.0, 1.0, 1.0),
            direction: (2.0, 0.0, 0.0),
            up: (0.0, 1.0, 0.0),
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
            moving_speed: 0.05,
            rotating_up: false,
            rotating_left: false,
            rotating_down: false,
            rotating_right: false,
            rotating_speed: 0.05,
        }
    }

    pub fn set_position(&mut self, pos: (f32, f32, f32)) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: (f32, f32, f32)) {
        self.direction = dir;
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn get_perspective(&self) -> [[f32; 4]; 4] {
        let fov: f32 = 3.141592 / 2.0;
        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (fov / 2.0).tan();

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [f / self.aspect_ratio,    0.0,              0.0              ,   0.0],
            [         0.0         ,     f ,              0.0              ,   0.0],
            [         0.0         ,    0.0,  (zfar+znear)/(zfar-znear)    ,   1.0],
            [         0.0         ,    0.0, -(2.0*zfar*znear)/(zfar-znear),   0.0],
        ]
    }

    pub fn get_view(&self) -> [[f32; 4]; 4] {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };


        let s = (f.1 * self.up.2 - f.2 * self.up.1,
                 f.2 * self.up.0 - f.0 * self.up.2,
                 f.0 * self.up.1 - f.1 * self.up.0);

        let s_norm = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s_norm.1 * f.2 - s_norm.2 * f.1,
                 s_norm.2 * f.0 - s_norm.0 * f.2,
                 s_norm.0 * f.1 - s_norm.1 * f.0);

        let p = (-self.position.0 * s.0 - self.position.1 * s.1 - self.position.2 * s.2,
                 -self.position.0 * u.0 - self.position.1 * u.1 - self.position.2 * u.2,
                 -self.position.0 * f.0 - self.position.1 * f.1 - self.position.2 * f.2);

        // note: remember that this is column-major, so the lines of code are actually columns
        [
            [s_norm.0, u.0, f.0, 0.0],
            [s_norm.1, u.1, f.1, 0.0],
            [s_norm.2, u.2, f.2, 0.0],
            [p.0, p.1,  p.2, 1.0],
        ]
    }

    pub fn update(&mut self) {
        let f = {
            let f = self.direction;
            let len = f.0 * f.0 + f.1 * f.1 + f.2 * f.2;
            let len = len.sqrt();
            (f.0 / len, f.1 / len, f.2 / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (f.1 * up.2 - f.2 * up.1,
                 f.2 * up.0 - f.0 * up.2,
                 f.0 * up.1 - f.1 * up.0);

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (s.1 * f.2 - s.2 * f.1,
                 s.2 * f.0 - s.0 * f.2,
                 s.0 * f.1 - s.1 * f.0);

        if self.moving_up {
            self.position.0 += u.0 * self.moving_speed;
            self.position.1 += u.1 * self.moving_speed;
            self.position.2 += u.2 * self.moving_speed;
        }

        if self.moving_left {
            self.position.0 -= s.0 * self.moving_speed;
            self.position.1 -= s.1 * self.moving_speed;
            self.position.2 -= s.2 * self.moving_speed;
        }

        if self.moving_down {
            self.position.0 -= u.0 * self.moving_speed;
            self.position.1 -= u.1 * self.moving_speed;
            self.position.2 -= u.2 * self.moving_speed;
        }

        if self.moving_right {
            self.position.0 += s.0 * self.moving_speed;
            self.position.1 += s.1 * self.moving_speed;
            self.position.2 += s.2 * self.moving_speed;
        }

        if self.moving_forward {
            self.position.0 += f.0 * self.moving_speed;
            self.position.1 += f.1 * self.moving_speed;
            self.position.2 += f.2 * self.moving_speed;
        }

        if self.moving_backward {
            self.position.0 -= f.0 * self.moving_speed;
            self.position.1 -= f.1 * self.moving_speed;
            self.position.2 -= f.2 * self.moving_speed;
        }

        if self.rotating_up {

            let null_pos: vecmath::Vector3<f32> = [1.0, 0.0, 0.0];
            let change: vecmath::Vector3<f32> = normalize([
                null_pos[0],
                null_pos[1] + self.rotating_speed,
                null_pos[2],
            ]);
            // let q = quaternion::rotation_from_to(null_pos, change);
            let direction_q = quaternion::rotation_from_to([1.0, 0.0, 0.0], normalize([self.direction.0, self.direction.1, self.direction.2]));
            let local_axis: vecmath::Vector3<f32> = [0.0, 0.0, 1.0];
            let world_axis: vecmath::Vector3<f32> = quaternion::rotate_vector(direction_q, local_axis);
            let world_rotation = quaternion::axis_angle(world_axis, self.rotating_speed);
            let new_direction = quaternion::mul(world_rotation, direction_q).1;
            println!("{:?}", direction_q);
            // let modifier = quaternion::euler_angles(0.0, PI / 2.0, 0.0);
            // let q = quaternion::mul(coord_sys, modifier);
            // let q = quaternion::axis_angle([self.direction.0, self.direction.1, self.direction.2], -self.rotating_speed);
            // let dir: vecmath::Vector3<f32> = normalize([
            //     self.direction.0, self.direction.1, self.direction.2
            // ]);
            // let new_direction = quaternion::rotate_vector(q, dir);
            // println!("{:?}", new_direction);
            self.direction = (new_direction[0], new_direction[1], new_direction[2])
            // let vec_vec: vecmath::Vector3<f32> = normalize(vec);
            // let vec_rot: vecmath::Vector3<f32> = normalize(rot);
            // let q = quaternion::rotation_from_to(vec_vec, vec_rot);
            // let rotated_vec = quaternion::rotate_vector(q, vec_vec);
            // (rotated_vec[0], rotated_vec[1], rotated_vec[2])


            // self.direction = rotate(
            //     self.direction,
            //     ((f.0 * (-self.rotating_speed).cos()) -
            //     (f.1 * (-self.rotating_speed).sin()),
            //     (f.0 * (-self.rotating_speed).sin()) +
            //     (f.1 * (-self.rotating_speed).cos()),
            //     self.direction.2)
            // )
            // self.direction.0 = {
            //     (f.0 * (-self.rotating_speed).cos()) -
            //     (f.1 * (-self.rotating_speed).sin())
            // };
            // self.direction.1 = {
            //     (f.0 * (-self.rotating_speed).sin()) +
            //     (f.1 * (-self.rotating_speed).cos())
            // };
        }
        if self.rotating_down {
            self.direction.0 = {
                (f.0 * self.rotating_speed.cos()) -
                (f.1 * self.rotating_speed.sin())
            };
            self.direction.1 = {
                (f.0 * self.rotating_speed.sin()) +
                (f.1 * self.rotating_speed.cos())
            };
        }
        if self.rotating_left {
            self.direction.0 = {
                (f.0 * self.rotating_speed.cos()) +
                (f.2 * self.rotating_speed.sin())
            };
            self.direction.2 = {
                (-f.0 * self.rotating_speed.sin()) +
                (f.2 * self.rotating_speed.cos())
            };
        }
        if self.rotating_right {
            self.direction.0 = {
                (f.0 * (-self.rotating_speed).cos()) +
                (f.2 * (-self.rotating_speed).sin())
            };
            self.direction.2 = {
                (- f.0 * (-self.rotating_speed).sin()) +
                (f.2 * (-self.rotating_speed).cos())
            };
        }
    }

    pub fn process_input(&mut self, event: &glutin::Event) {
        match event {
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Q)) => {
                self.moving_up = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Q)) => {
                self.moving_up = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::E)) => {
                self.moving_down = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::E)) => {
                self.moving_down = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::A)) => {
                self.moving_left = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::D)) => {
                self.moving_right = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::W)) => {
                self.moving_forward = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::S)) => {
                self.moving_backward = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Up)) => {
                self.rotating_up = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Up)) => {
                self.rotating_up = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.rotating_down = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Down)) => {
                self.rotating_down = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Left)) => {
                self.rotating_left = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Left)) => {
                self.rotating_left = false;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Pressed, _, Some(glutin::VirtualKeyCode::Right)) => {
                self.rotating_right = true;
            },
            &glutin::Event::KeyboardInput(glutin::ElementState::Released, _, Some(glutin::VirtualKeyCode::Right)) => {
                self.rotating_right = false;
            },
            _ => {}
        }
    }
}
