use crate::Args;

pub mod perlin_flow;
pub mod lorenz;


pub trait Scene {
    type SceneOptions;
    type Model;

    fn new_scene(options: &Self::SceneOptions) -> Self;

    fn app(&self) -> nannou::app::Builder<Self::Model>;
}
