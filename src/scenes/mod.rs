use async_trait::async_trait;

pub mod base3d;
pub mod perlin_flow;
pub mod lorenz;


#[async_trait]
pub trait Scene {
    type SceneOptions;
    type Model;

    fn new_scene(options: &Self::SceneOptions) -> Self;

    async fn app(&self) -> nannou::app::Builder<Self::Model>;
}
