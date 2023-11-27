use async_std::task::block_on;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;
use scenes::perlin_flow::{PerlinFlowOptions, PerlinFlowScene};
use scenes::Scene;
use crate::scenes::base3d::Base3DScene;

mod particle;
mod scenes;
#[path="3d/mod.rs"]
mod math_3d;

lazy_static! {
    static ref ARGS: Args = Args::parse();
}


#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    scene: SceneArgs,
}

#[derive(Subcommand)]
enum SceneArgs {
    /// Selects the Perlin Flow Model scene
    PerlinFlow {
        /// Toggles whether to show the direction vectors
        #[arg(long, default_value_t = false)]
        show_vectors: bool,

        /// Toggles whether to show the moving dot
        #[arg(long, default_value_t = false)]
        hide_dots: bool
    },
    /// A 3D simulation of a Lorenz attractor
    Lorenz {
        /// The rho from the Lorenz equation, see wikipedia
        #[arg(long, default_value_t = 28.0)]
        rho: f32,
        /// The sigma from the Lorenz equation, see wikipedia
        #[arg(long, default_value_t = 10.0)]
        sigma: f32,
        /// The beta from the Lorenz equation, see wikipedia
        #[arg(long, default_value_t = 2.66667)]
        beta: f32,
    },
    Base3d {},
}

#[wasm_bindgen]
pub async fn main_base3d() {
    let app = Base3DScene::new_scene(&()).app().await;
    app.run();
}

