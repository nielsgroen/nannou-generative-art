use rand::{Rng, thread_rng};
use nannou::noise::{Perlin, Seedable};
use nannou::noise;
use nannou::noise::NoiseFn;
use nannou::math::num_traits::Float;
use nannou::prelude::*;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use nannou::color::Alpha;
use crate::particle::Particle;
use crate::scenes::perlin_flow::{PerlinFlowOptions, PerlinFlowScene};
use crate::scenes::Scene;

mod particle;
mod scenes;

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
    }
}

fn main() {
    // can't make a custom model yet: https://github.com/nannou-org/nannou/issues/793
    // incorporate this using lazy_static

    let args = Args::parse();

    let app;
    match args.scene {
        SceneArgs::PerlinFlow { .. } => {
            app = PerlinFlowScene::new_scene(&PerlinFlowOptions::from_args(&args)).app();
        }
    }

    app.run();
    // nannou::app(model).update(update).simple_window(view).size(1800, 1200).run();
}
