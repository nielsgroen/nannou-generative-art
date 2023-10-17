use rand::{Rng, thread_rng};
use nannou::noise::{Perlin, Seedable};
use nannou::{color, noise};
use nannou::noise::NoiseFn;
use nannou::math::num_traits::Float;
use nannou::prelude::*;

use clap::Parser;
use nannou::color::Alpha;


#[derive(Parser)]
struct Cli {
    /// Toggles whether to show the direction vectors
    #[arg(long, default_value_t = false)]
    show_vectors: bool,

    /// Toggles whether to show the moving dot
    #[arg(long, default_value_t = false)]
    show_dot: bool
}


struct Model<T>
where
    T: NoiseFn<[f64; 2]>
{
    pub seed: u32,
    pub noise_fn: T,
    pub noise_scale: f64,
    pub particle_position: Point2,
}


fn main() {
    // can't make a custom model yet: https://github.com/nannou-org/nannou/issues/793
    // incorporate this using lazy_static
    let cli = Cli::parse();

    nannou::app(model).update(update).simple_window(view).size(600, 600).run();
}

fn model(_app: &App) -> Model<Perlin> {
    let seed: u32 = thread_rng().gen();
    let noise_fn = noise::Perlin::new().set_seed(seed);
    Model {
        seed,
        noise_fn,
        noise_scale: 2.0,
        particle_position: pt2(0.0, 0.0),
    }
}

fn view<T>(app: &App, model: &Model<T>, frame: Frame)
where
    T: NoiseFn<[f64; 2]>
{
    // draw stuff
    let win = app.window_rect();
    let draw = app.draw();


    draw.background().color(BLANCHEDALMOND);

    // add circles in a grid
    let grid_size_x = 20;
    let grid_size_y = 20;
    let x_step_size = win.x.len() / grid_size_x as f32;
    let y_step_size = win.y.len() / grid_size_y as f32;

    let line_length = 12.0;

    for grid_x in 0..grid_size_x {
        for grid_y in 0..grid_size_y {
            // Drawing a circle at every grid point.
            let x = grid_x as f32 * x_step_size + win.x.start + x_step_size / 2.0;
            let y = grid_y as f32 * y_step_size + win.y.start + y_step_size / 2.0;
            // draw.ellipse().xy(pt2(x, y)).radius(2.0).color(STEELBLUE);


            // Now to draw a vector at every for every grid point according to the noise value
            let x_scaled = (x - win.x.start) / win.x.len();
            let y_scaled = (y - win.y.start) / win.y.len();
            let number: f64 = model.noise_fn.get([x_scaled as f64 * model.noise_scale, y_scaled as f64 * model.noise_scale]);
            let angle = number as f32 * TAU;

            let new_x = x + angle.cos() * line_length;
            let new_y = y + angle.sin() * line_length;
            draw.line().start(pt2(x, y)).end(pt2(new_x, new_y)).color(DARKRED).stroke_weight(1.0);
        }
    }

    draw.ellipse().radius(4.0).color(BLACK).xy(model.particle_position);
    draw.to_frame(app, &frame).unwrap();
}

fn update<T>(app: &App, model: &mut Model<T>, _update: Update)
where
    T: NoiseFn<[f64; 2]>
{
    let win = app.window_rect();

    let x = model.particle_position.x;
    let y = model.particle_position.y;

    let move_length = 30.0;

    let x_scaled = (x - win.x.start) / win.x.len();
    let y_scaled = (y - win.y.start) / win.y.len();
    let number: f64 = model.noise_fn.get([x_scaled as f64 * model.noise_scale, y_scaled as f64 * model.noise_scale]);
    let angle = number as f32 * TAU;

    let new_x = x + angle.cos() * move_length * app.duration.since_prev_update.as_secs_f32();
    let new_y = y + angle.sin() * move_length * app.duration.since_prev_update.as_secs_f32();;

    model.particle_position = pt2(new_x, new_y);
}

