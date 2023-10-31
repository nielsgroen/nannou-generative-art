use nannou::color::Alpha;
use nannou::prelude::*;


pub const MAX_VELOCITY: f32 = 40.0;

#[derive(Copy, Clone, Debug, Default)]
pub struct Particle2 {
    pub position: Point2,
    pub velocity: Vec2,
    pub radius: f32,
    pub color: Alpha<Rgb, f32>,
}

impl Particle2 {
    pub fn new(position: Point2, color: Alpha<Rgb, f32>) -> Self {
        Self {
            position,
            velocity: vec2(0.0, 0.0),
            radius: 1.0,
            color,
        }
    }


    /// Updates its position and velocity given a force vector, and the amount of time passed.
    pub fn process_force(&mut self, force: Vec2, time_passed: f32) {
        self.velocity += force * time_passed;
        self.velocity = self.velocity.clamp_length_max(MAX_VELOCITY);
    }

    pub fn move_at_velocity(&mut self, time_passed: f32) {
        self.position += self.velocity * time_passed;
    }

    pub fn update(&mut self, force: Vec2, time_passed: f32) {
        self.process_force(force, time_passed);
        self.move_at_velocity(time_passed);
    }

    pub fn position_scaled(&self, app: &App) -> Point2 {
        scale_coords(app, self.position)
    }
}


#[derive(Copy, Clone, Debug, Default)]
pub struct Particle3 {
    pub position: Point3,
    pub velocity: Vec3,
    pub radius: f32,
    pub color: Alpha<Rgb, f32>,
}

impl Particle3 {
    pub fn new(position: Point3, color: Alpha<Rgb, f32>, radius: f32) -> Self {
        Self {
            position,
            velocity: vec3(0.0, 0.0, 0.0),
            radius,
            color,
        }
    }


    /// Updates its position and velocity given a force vector, and the amount of time passed.
    pub fn process_force(&mut self, force: Vec3, time_passed: f32) {
        self.velocity += force * time_passed;
        self.velocity = self.velocity.clamp_length_max(MAX_VELOCITY);
    }

    pub fn move_at_velocity(&mut self, time_passed: f32) {
        self.position += self.velocity * time_passed;
    }

    pub fn update(&mut self, force: Vec3, time_passed: f32) {
        self.process_force(force, time_passed);
        self.move_at_velocity(time_passed);
    }

    /// Returns a transformed particle position
    pub fn transform_position(&self, transformation_matrix: Mat4) -> Vec4 {
        transformation_matrix * Vec4::from((self.position, 1.0))
    }
}


pub fn scale_coords(app: &App, coords: Point2) -> Point2 {
    let win = app.window_rect();

    (coords - win.bottom_left()) / win.wh()
}
