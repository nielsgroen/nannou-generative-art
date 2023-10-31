use rand::{Rng, thread_rng};
use nannou::noise::{Perlin, Seedable};
use nannou::noise;
use nannou::noise::NoiseFn;
use nannou::math::num_traits::Float;
use nannou::prelude::*;

use clap::{Parser, Subcommand};
use lazy_static::lazy_static;
use nannou::app::Builder;
use nannou::color::Alpha;
use crate::particle::Particle2;
use crate::scenes::base3d::Base3DScene;
use crate::scenes::lorenz::{LorenzOptions, LorenzScene};
use crate::scenes::perlin_flow::{PerlinFlowOptions, PerlinFlowScene};
use crate::scenes::Scene;

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

fn main() {
    // can't make a custom model yet: https://github.com/nannou-org/nannou/issues/793
    // incorporate this using lazy_static

    let args = Args::parse();

    match args.scene {
        SceneArgs::PerlinFlow { .. } => {
            PerlinFlowScene::new_scene(&PerlinFlowOptions::from_args(&args)).app().run();
        }
        SceneArgs::Lorenz { .. } => {
            LorenzScene::new_scene(&LorenzOptions::from_args(&args)).app().run();
        }
        SceneArgs::Base3d { .. } => {
            Base3DScene::new_scene(&()).app().run();
        }
    }

    // nannou::app(model).update(update).simple_window(view).size(1800, 1200).run();
}
