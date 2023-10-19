use rand::{Rng, thread_rng};
use nannou::noise::{Perlin, Seedable};
use nannou::noise;
use nannou::noise::NoiseFn;
use nannou::math::num_traits::Float;
use nannou::prelude::*;

use clap::Parser;
use lazy_static::lazy_static;
use nannou::color::Alpha;
use crate::particle::Particle;

mod particle;

lazy_static! {
    static ref ARGS: Args = Args::parse();
}


#[derive(Parser)]
struct Args {
    /// Toggles whether to show the direction vectors
    #[arg(long, default_value_t = false)]
    show_vectors: bool,

    /// Toggles whether to show the moving dot
    #[arg(long, default_value_t = false)]
    hide_dots: bool
}


struct Model<T>
where
    T: NoiseFn<[f64; 2]>
{
    pub seed: u32,
    pub noise_fn: T,
    pub noise_scale: f64,
    pub particles: Vec<Particle>,
    // pub particle_position: Point2,
    // pub particle_velocity: Vec2,
}


fn main() {
    // can't make a custom model yet: https://github.com/nannou-org/nannou/issues/793
    // incorporate this using lazy_static

    nannou::app(model).update(update).simple_window(view).size(1800, 1200).run();
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
            Particle::new(
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
        // particle_position: pt2(0.0, 0.0),
        // particle_velocity: vec2(0.0, 0.0),
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
    // let grid_size_x = 20;
    // let grid_size_y = 20;
    let grid_size = (20, 20);
    // let x_step_size = win.x.len() / grid_size_x as f32;
    // let y_step_size = win.y.len() / grid_size_y as f32;
    let step_size: Vec2 = win.wh() / vec2(grid_size.0 as f32, grid_size.1 as f32);
    // let step_size = vec2(win.x.len() / grid_size_x as f32, win.y.len() / grid_size_y as f32);

    let line_length = 12.0;

    if ARGS.show_vectors {
        for grid_x in 0..grid_size.0 {
            for grid_y in 0..grid_size.1 {
                // Drawing a circle at every grid point.
                let grid_num = pt2(grid_x as f32, grid_y as f32);
                let xy = grid_num * step_size + win.bottom_left() + step_size / 2.0;
                // draw.ellipse().xy(pt2(x, y)).radius(2.0).color(STEELBLUE);

                let new_xy = xy + force_vector(&model.noise_fn, model.noise_scale, scale_coords(app, xy)) * 10.0;
                draw.line().start(xy).end(new_xy).color(DARKRED).stroke_weight(1.0);
            }
        }
    }

    if !ARGS.hide_dots {
        for particle in model.particles.iter() {
            draw.ellipse().radius(1.0).color(Alpha { color: particle.color.color, alpha: particle.color.alpha * time_passed * 1000.0 }).xy(particle.position);
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

// fn force_vector<T>(model: &Model<T>, scaled_coords: Point2) -> Vec2
fn force_vector<T>(noise_fn: &T, noise_scale: f64, scaled_coords: Point2) -> Vec2
where
    T: NoiseFn<[f64; 2]>
{
    let move_length = 10.0;

    let number: f64 = noise_fn.get((scaled_coords.as_f64() * noise_scale).to_array());
    let angle = number as f32 * TAU;

    let unit_vector = vec2(angle.cos(), angle.sin());

    // unit_vector
    unit_vector * move_length
}

fn scale_coords(app: &App, coords: Point2) -> Point2 {
    let win = app.window_rect();

    (coords - win.bottom_left()) / win.wh()
}
