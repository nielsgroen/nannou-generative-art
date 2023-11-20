/// The controls for the camera

use nannou::prelude::*;
use crate::math_3d::Camera;


/// The controls for the camera
/// Keeps track of input and updates the camera accordingly
pub trait CameraControls {
    fn event(&mut self, app: &App, event: Event);
    fn apply_to_camera(&self, camera: &mut Camera, app: &App);
}

/// Camera Controls that will always make the camera look at the origin
pub struct CenteredCameraControls {
    pub center: Point3,
    pub speed: f32,
    pub zoom_speed: f32,
    pub left_pressed: bool,
    pub right_pressed: bool,
    pub up_pressed: bool,
    pub down_pressed: bool,
    pub zoom_in_pressed: bool,
    pub zoom_out_pressed: bool,
}

impl CenteredCameraControls {
    pub fn new(center: Point3, speed: f32, zoom_speed: f32) -> Self {
        Self {
            center,
            speed,
            zoom_speed,
            left_pressed: false,
            right_pressed: false,
            up_pressed: false,
            down_pressed: false,
            zoom_in_pressed: false,
            zoom_out_pressed: false,
        }
    }
}

impl CameraControls for CenteredCameraControls {
    fn event(&mut self, _app: &App, event: Event) {
        match event {
            Event::WindowEvent { simple: Some(inner_event), .. } => {
                match inner_event {
                    KeyPressed(key) => {
                        match key {
                            Key::Left => self.left_pressed = true,
                            Key::Right => self.right_pressed = true,
                            Key::Up => self.up_pressed = true,
                            Key::Down => self.down_pressed = true,
                            Key::PageUp => self.zoom_in_pressed = true,
                            Key::PageDown => self.zoom_out_pressed = true,
                            Key::A => self.left_pressed = true,
                            Key::D => self.right_pressed = true,
                            Key::W => self.up_pressed = true,
                            Key::S => self.down_pressed = true,
                            Key::Q => self.zoom_in_pressed = true,
                            Key::E => self.zoom_out_pressed = true,
                            _ => {}
                        }
                    },
                    KeyReleased(key) => {
                        match key {
                            Key::Left => self.left_pressed = false,
                            Key::Right => self.right_pressed = false,
                            Key::Up => self.up_pressed = false,
                            Key::Down => self.down_pressed = false,
                            Key::PageUp => self.zoom_in_pressed = false,
                            Key::PageDown => self.zoom_out_pressed = false,
                            Key::A => self.left_pressed = false,
                            Key::D => self.right_pressed = false,
                            Key::W => self.up_pressed = false,
                            Key::S => self.down_pressed = false,
                            Key::Q => self.zoom_in_pressed = false,
                            Key::E => self.zoom_out_pressed = false,
                            _ => {}
                        }
                    },
                    _ => {},
                }
            },
            _ => {},
        }
    }

    fn apply_to_camera(&self, camera: &mut Camera, app: &App) {
        let mut direction = vec3(0.0, 0.0, 0.0);

        if self.left_pressed {
            direction += vec3(-1.0, 0.0, 0.0);
        }
        if self.right_pressed {
            direction += vec3(1.0, 0.0, 0.0);
        }
        if self.up_pressed {
            direction += vec3(0.0, 1.0, 0.0);
        }
        if self.down_pressed {
            direction += vec3(0.0, -1.0, 0.0);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }
        direction = self.speed * direction;

        let mut zoom_direction = vec3(0.0, 0.0, 0.0);

        if self.zoom_in_pressed {
            zoom_direction += vec3(0.0, 0.0, 1.0);
        }
        if self.zoom_out_pressed {
            zoom_direction += vec3(0.0, 0.0, -1.0);
        }

        if zoom_direction.length() > 0.0 {
            zoom_direction = zoom_direction.normalize();
        }
        zoom_direction = self.zoom_speed * zoom_direction;

        let velocity = direction + zoom_direction;

        let mut camera_velocity = vec3(0.0, 0.0, 0.0);
        camera_velocity += camera.view_direction * velocity.z;
        camera_velocity += camera.up * velocity.y;
        camera_velocity += camera.view_direction.cross(camera.up) * velocity.x;

        camera.position += camera_velocity * app.duration.since_prev_update.as_secs_f32();
        camera.look_at(self.center);
    }
}

pub struct MouseBasedCenteredCameraControls {
    pub center: Point3,
    pub speed: f32,
    // pub zoom_speed: f32,
    // pub left_pressed: bool,
    // pub right_pressed: bool,
    // pub up_pressed: bool,
    // pub down_pressed: bool,
    // pub zoom_in_pressed: bool,
    // pub zoom_out_pressed: bool,
    // pub mouse_pressed: bool,
    pub mouse_position: Point2,
    pub mouse_position_prev: Point2,
}

impl MouseBasedCenteredCameraControls {
    pub fn new(center: Point3, speed: f32) -> Self {
        Self {
            center,
            speed,
            // zoom_speed,
            // left_pressed: false,
            // right_pressed: false,
            // up_pressed: false,
            // down_pressed: false,
            // zoom_in_pressed: false,
            // zoom_out_pressed: false,
            // mouse_pressed: false,
            mouse_position: pt2(0.0, 0.0),
            mouse_position_prev: pt2(0.0, 0.0),
        }
    }
}

impl CameraControls for MouseBasedCenteredCameraControls {
    fn event(&mut self, app: &App, event: Event) {
        match event {
            Event::Update(update) => {
                self.mouse_position = app.mouse.position();
                self.mouse_position_prev = self.mouse_position_prev + (self.mouse_position - self.mouse_position_prev) * update.since_last.as_secs_f32();
                self.mouse_position_prev = (self.mouse_position_prev - self.mouse_position).clamp_length_max(2.0) + self.mouse_position;
            },
            _ => {},
        }
    }

    fn apply_to_camera(&self, camera: &mut Camera, app: &App) {
        let mut direction = vec3(0.0, 0.0, 0.0);

        let mouse_delta = self.mouse_position - self.mouse_position_prev;
        direction += vec3(mouse_delta.x, mouse_delta.y, 0.0);

        direction = self.speed * direction;

        // let mut zoom_direction = vec3(0.0, 0.0, 0.0);
        //
        // if self.zoom_in_pressed {
        //     zoom_direction += vec3(0.0, 0.0, 1.0);
        // }
        // if self.zoom_out_pressed {
        //     zoom_direction += vec3(0.0, 0.0, -1.0);
        // }
        //
        // if zoom_direction.length() > 0.0 {
        //     zoom_direction = zoom_direction.normalize();
        // }
        // zoom_direction = self.zoom_speed * zoom_direction;

        let velocity = direction; // + zoom_direction;

        let mut camera_velocity = vec3(0.0, 0.0, 0.0);
        camera_velocity += camera.view_direction * velocity.z;
        camera_velocity += camera.up * velocity.y;
        camera_velocity += camera.view_direction.cross(camera.up) * velocity.x;

        camera.position += camera_velocity * app.duration.since_prev_update.as_secs_f32();
        camera.look_at(self.center);
    }
}
