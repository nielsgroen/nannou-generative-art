use std::fmt::Debug;
use nannou::glam::Vec4Swizzles;
use nannou::prelude::*;
use crate::math_3d::projection::{OrthographicProjection, PerspectiveProjection, Projection};

pub mod projection;

/// For turning 3D coordinates to screen space
/// The screen space is defined as the area of (x, y) values with range `[-1, 1]`
pub struct Camera {
    pub position: Point3,
    pub view_direction: Vec3,
    pub up: Vec3,
    pub projection: Box<dyn Projection>,
}


impl Camera {
    /// Creates a new orthographic camera.
    pub fn new_orthographic(position: Point3, view_direction: Vec3, up: Vec3, size: f32, aspect_ratio: f32) -> Self {
        let projection = OrthographicProjection::new(size, aspect_ratio);

        Self {
            position,
            view_direction,
            up,
            projection: Box::new(projection),
        }
    }

    /// Creates a new perspective camera.
    /// A good default for the fov in radians is `0.25`.
    pub fn new_perspective(position: Point3, view_direction: Vec3, up: Vec3, fov: f32, aspect_ratio: f32) -> Self {
        let projection = PerspectiveProjection::new(fov, aspect_ratio);

        Self {
            position,
            view_direction,
            up,
            projection: Box::new(projection),
        }
    }

    /// Turns the camera to look at the given coordinates.
    pub fn look_at(&mut self, target_position: Point3) {
        self.view_direction = (target_position - self.position).normalize();
    }

    /// Returns the matrix transformation that centers the world around the camera.
    pub fn get_view_matrix(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.view_direction, self.up)
    }

    /// Turns camera coordinates (where the world is assumed to be centered around the camera)
    /// into screen coordinates in the range of `[-1, 1]`.
    pub fn get_projection_matrix(&self) -> Mat4 {
        self.projection.get_transformation_matrix()
    }

    /// Turns world coordinates into screen coordinates.
    pub fn get_transformation_matrix(&self) -> Mat4 {
        let view_matrix = self.get_view_matrix();
        let projection_matrix = self.get_projection_matrix();

        projection_matrix * view_matrix
        // view_matrix.transpose() * projection_matrix.transpose()  // TODO: remove
    }

    pub fn get_z_near(&self) -> f32 {
        self.projection.get_z_near()
    }

    pub fn get_z_far(&self) -> f32 {
        self.projection.get_z_far()
    }

    pub fn z_near(&mut self, z_near: f32) {
        self.projection.z_near(z_near);
    }

    pub fn z_far(&mut self, z_far: f32) {
        self.projection.z_far(z_far);
    }

    pub fn get_aspect_ratio(&self) -> f32 {
        self.projection.get_aspect_ratio()
    }

    pub fn aspect_ratio(&mut self, aspect_ratio: f32) {
        self.projection.aspect_ratio(aspect_ratio);
    }
}
