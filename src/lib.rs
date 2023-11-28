use async_std::task::block_on;
use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use wasm_bindgen::prelude::wasm_bindgen;
use scenes::perlin_flow::{PerlinFlowOptions, PerlinFlowScene};
use scenes::Scene;
use crate::scenes::base3d::Base3DScene;
use crate::scenes::lorenz::{LorenzOptions, LorenzScene};

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
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = Base3DScene::new_scene(&()).app().await;
    app.run();
}

#[wasm_bindgen]
pub async fn main_perlin_flow(seed: u32) {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = PerlinFlowScene::new_scene(&PerlinFlowOptions {
        show_vectors: false,
        hide_dots: false,
        seed: Some(seed),
    }).app().await;
    app.run();
}

#[wasm_bindgen]
pub async fn main_lorenz() {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let app = LorenzScene::new_scene(&LorenzOptions {
        rho: 28.0,
        sigma: 10.0,
        beta: 2.66667,
    }).app().await;
    app.run();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
