use rand::{Rng, thread_rng};
use nannou::noise::{Perlin, Seedable};
use nannou::noise;
use nannou::noise::NoiseFn;
use nannou::math::num_traits::Float;
use nannou::prelude::*;

use async_trait::async_trait;
use clap::Parser;
use lazy_static::lazy_static;
use nannou::app::Builder;
use nannou::color::Alpha;
use nannou::wgpu::{DeviceDescriptor, Limits};
use crate::{Args, SceneArgs};
use crate::particle::{Particle2, scale_coords};
use crate::scenes::Scene;


// Can't inject parameters into the nannou model yet, so we need to use lazy_static
// to access them as global variables.
// https://github.com/nannou-org/nannou/issues/793
lazy_static! {
    static ref OPTIONS: PerlinFlowOptions = PerlinFlowOptions::from_args(&Args::parse());
}

#[derive(Copy, Clone, Debug)]
pub struct PerlinFlowScene {
    // _app_builder: Builder<Model<Perlin>>,
    model_fn: fn(app: &App) -> Model<Perlin>,
    update_fn: fn(app: &App, model: &mut Model<Perlin>, _update: Update),
    view_fn: fn(app: &App, model: &Model<Perlin>, frame: Frame),
}

#[derive(Copy, Clone, Debug)]
pub struct PerlinFlowOptions {
    pub show_vectors: bool,
    pub hide_dots: bool,
}

impl PerlinFlowOptions {
    pub fn from_args(cli_args: &Args) -> Self {
        match cli_args.scene {
            SceneArgs::PerlinFlow { show_vectors, hide_dots } => {
                Self {
                    show_vectors,
                    hide_dots,
                }
            },
            _ => panic!("Can't construct PerlinFlowOptions from this scene type"),
        }
    }
}

#[async_trait]
impl Scene for PerlinFlowScene {
    type SceneOptions = PerlinFlowOptions;
    type Model = Model<Perlin>;


    fn new_scene(options: &Self::SceneOptions) -> Self {
        Self {
            model_fn: model,
            update_fn: update,
            view_fn: view,
        }
    }

    async fn app(&self) -> Builder<Self::Model> {
        // let model = Model {
        //     seed: 0,
        //     noise_fn: (),
        //     noise_scale: 0.0,
        //     particles: vec![],
        // };
        // thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());
        //
        // let builder = app::Builder::new_async(|app| {
        //     Box::new(async move {
        //         let device_descriptor = DeviceDescriptor {
        //             limits: Limits {
        //                 max_texture_dimension_2d: 8192,
        //                 ..Limits::downlevel_webgl2_defaults()
        //             },
        //             ..Default::default()
        //         };
        //
        //         app.new_window()
        //             .device_descriptor(device_descriptor)
        //             .view(self.view_fn)
        //             .title("n0ls Base3D")
        //             .build_async()
        //             .await
        //             .unwrap();
        //
        //         (self.model_fn)(app)
        //     })
        // });


        // todo!();

        nannou::app(self.model_fn)
            .update(self.update_fn)
            .simple_window(self.view_fn)
            .size(1800, 1200)
    }
}




pub struct Model<T>
    where
        T: NoiseFn<[f64; 2]>
{
    pub seed: u32,
    pub noise_fn: T,
    pub noise_scale: f64,
    pub particles: Vec<Particle2>,
}


fn model(app: &App) -> Model<Perlin> {
    let seed: u32 = thread_rng().gen();
    let noise_fn = noise::Perlin::new().set_seed(seed);

    let win = app.window_rect();

    Model {
        seed,
        noise_fn,
        noise_scale: 4.0,
        particles: vec![0; 1000].iter().map(|_|
            Particle2::new(
                pt2(
                    thread_rng().gen_range(win.x.start as i32 / 3..win.x.end as i32 / 3) as f32,
                    thread_rng().gen_range(win.y.start as i32 / 3..win.y.end as i32 / 3) as f32,
                ),
                Alpha {
                    color: WHITE.into_format(),
                    alpha: 0.0003,
                }
            )
        ).collect(),
    }
}

fn view<T>(app: &App, model: &Model<T>, frame: Frame)
    where
        T: NoiseFn<[f64; 2]>
{
    // draw stuff
    let win = app.window_rect();
    let draw = app.draw();
    let time_passed = app.duration.since_prev_update.as_secs_f32();

    // if app.elapsed_frames() < 2 {
    //     draw.background().color(BLANCHEDALMOND);
    // }
    //
    // draw.background().color(BLANCHEDALMOND);
    // draw.background().color(Alpha { color: DARKBLUE, alpha: 0.90000001 });
    // draw.rect().wh(win.wh()).xy(win.xy()).color(Alpha { color: BLANCHEDALMOND, alpha: 0.009 });

    // add circles in a grid
    let grid_size = (20, 20);
    let step_size: Vec2 = win.wh() / vec2(grid_size.0 as f32, grid_size.1 as f32);

    let line_length = 12.0;

    if OPTIONS.show_vectors {
        for grid_x in 0..grid_size.0 {
            for grid_y in 0..grid_size.1 {
                // Drawing a circle at every grid point.
                let grid_num = pt2(grid_x as f32, grid_y as f32);
                let xy = grid_num * step_size + win.bottom_left() + step_size / 2.0;

                let new_xy = xy + force_vector(&model.noise_fn, model.noise_scale, scale_coords(app, xy)) * 10.0;
                draw.line().start(xy).end(new_xy).color(DARKRED).stroke_weight(1.0);
            }
        }
    }

    if !OPTIONS.hide_dots {
        for particle in model.particles.iter() {
            draw.ellipse().radius(particle.radius).color(Alpha { color: particle.color.color, alpha: particle.color.alpha * time_passed * 1000.0 }).xy(particle.position);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update<T>(app: &App, model: &mut Model<T>, _update: Update)
    where
        T: NoiseFn<[f64; 2]> + Clone
{
    let time_passed = app.duration.since_prev_update.as_secs_f32();

    let noise_fn = model.noise_fn.clone();
    model.particles.iter_mut().for_each(|mut x| {
        let force = force_vector(&noise_fn, model.noise_scale, x.position_scaled(app));
        x.update(force, time_passed);
    });
}

fn force_vector<T>(noise_fn: &T, noise_scale: f64, scaled_coords: Point2) -> Vec2
    where
        T: NoiseFn<[f64; 2]>
{
    let move_length = 22.0;

    let number: f64 = noise_fn.get((scaled_coords.as_f64() * noise_scale).to_array());
    let angle = number as f32 * TAU;

    let unit_vector = vec2(angle.cos(), angle.sin());

    // unit_vector
    unit_vector * move_length
}

