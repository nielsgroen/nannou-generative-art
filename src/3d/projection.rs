use nannou::glam::Vec4Swizzles;
use nannou::prelude::*;


const DEFAULT_Z_NEAR: f32 = 0.1;
const DEFAULT_Z_FAR: f32 = 1000.0;
/// The object that turns camera space to screen space in the range of `[-1, 1]`
pub trait Projection {
    fn apply_projection(&self, point: Point3) -> Point2 {
        let projection_matrix = self.get_transformation_matrix();

        let vec = Vec4::from((point, 1.0));

        let result = projection_matrix * vec;
        result.xy()
    }

    fn get_transformation_matrix(&self) -> Mat4;

    fn get_z_near(&self) -> f32;
    fn get_z_far(&self) -> f32;

    fn z_near(&mut self, z_near: f32);
    fn z_far(&mut self, z_far: f32);

    fn get_aspect_ratio(&self) -> f32;
    fn aspect_ratio(&mut self, aspect_ratio: f32);
}


/// For making an Orthographic projection
#[derive(Copy, Clone, Debug)]
pub struct OrthographicProjection {
    /// Size determines what coordinate in camera space gets mapped to 1.0 (top of the screen).
    /// So this will map the y-coordinate in camera space to `[-1.0, 1.0]`
    pub size: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub aspect_ratio: f32,
}


impl OrthographicProjection {
    pub fn new(size: f32, aspect_ratio: f32) -> Self {
        Self {
            size,
            z_near: 0.1,
            z_far: 1000.0,
            aspect_ratio,
        }
    }
}

impl Projection for OrthographicProjection {
    fn get_transformation_matrix(&self) -> Mat4 {
        Mat4::orthographic_rh_gl(
            -self.size * self.aspect_ratio,
            self.size * self.aspect_ratio,
            -self.size,
            self.size,
            self.z_near,
            self.z_far,
        )
    }

    fn get_z_near(&self) -> f32 {
        self.z_near
    }

    fn get_z_far(&self) -> f32 {
        self.z_far
    }

    fn z_near(&mut self, z_near: f32) {
        self.z_near = z_near;
    }

    fn z_far(&mut self, z_far: f32) {
        self.z_far = z_far;
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    fn aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}

pub struct PerspectiveProjection {
    pub fov_y_radians: f32,
    pub z_near: f32,
    pub z_far: f32,
    pub aspect_ratio: f32,
}

impl PerspectiveProjection {
    pub fn new(fov: f32, aspect_ratio: f32) -> Self {
        Self {
            fov_y_radians: fov,
            z_near: 0.1,
            z_far: 1000.0,
            aspect_ratio,
        }
    }
}

impl Projection for PerspectiveProjection {
    fn get_transformation_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(
            self.fov_y_radians,
            self.aspect_ratio,
            self.z_near,
            self.z_far,
        )
    }

    fn get_z_near(&self) -> f32 {
        self.z_near
    }

    fn get_z_far(&self) -> f32 {
        self.z_far
    }

    fn z_near(&mut self, z_near: f32) {
        self.z_near = z_near;
    }

    fn z_far(&mut self, z_far: f32) {
        self.z_far = z_far;
    }

    fn get_aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    fn aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }
}
