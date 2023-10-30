use lazy_static::lazy_static;
use nannou::prelude::*;
use nannou::{App, Frame};
use nannou::app::Builder;
use nannou::color::Alpha;
use nannou::event::Update;
use nannou::geom::pt3;
use rand::{Rng, thread_rng};
use clap::Parser;
use nannou::glam::Vec4Swizzles;
use crate::{Args, SceneArgs};
use crate::particle::Particle3;
use crate::scenes::Scene;


const BACKGROUND_COLOR: Srgb<u8> = BLANCHEDALMOND;

// Can't inject parameters into the nannou model yet, so we need to use lazy_static
// to access them as global variables.
// https://github.com/nannou-org/nannou/issues/793
lazy_static! {
    static ref OPTIONS: LorenzOptions = LorenzOptions::from_args(&Args::parse());
}


pub struct LorenzScene {
    model_fn: fn(app: &App) -> Model,
    update_fn: fn(app: &App, model: &mut Model, _update: Update),
    view_fn: fn(app: &App, model: &Model, frame: Frame),
}

#[derive(Copy, Clone, Debug)]
pub struct LorenzOptions {
    pub rho: f32,  // https://en.wikipedia.org/wiki/Lorenz_system
    pub sigma: f32,
    pub beta: f32,
}

impl LorenzOptions {
    pub fn from_args(cli_args: &Args) -> Self {
        match cli_args.scene {
            SceneArgs::Lorenz { rho, sigma, beta } => {
                Self {
                    rho,
                    sigma,
                    beta,
                }
            },
            _ => panic!("Can't construct LorenzOptions from this scene type"),
        }
    }
}


impl Scene for LorenzScene {
    type SceneOptions = LorenzOptions;
    type Model = Model;


    fn new_scene(options: &Self::SceneOptions) -> Self {
        Self {
            model_fn: model,
            update_fn: update,
            view_fn: view,
        }
    }

    fn app(&self) -> Builder<Self::Model> {
        nannou::app(self.model_fn).update(self.update_fn).simple_window(self.view_fn).size(1800, 1200)
    }
}


pub struct Model {
    pub particles: Vec<Particle3>,
    pub rho: f32,  // https://en.wikipedia.org/wiki/Lorenz_system
    pub sigma: f32,
    pub beta: f32,
    pub camera_angle: f32,
    // TODO: add camera
}


fn model(app: &App) -> Model {
    let win = app.window_rect();

    let particles = vec![0; 1000].into_iter().map(|_| {
        Particle3::new(
            pt3(
                thread_rng().gen_range(-10..10) as f32,
                thread_rng().gen_range(-10..10) as f32,
                thread_rng().gen_range(-10..10) as f32,
            ),
            Alpha {
                color: rgb(0.0, 0.0, 0.0),
                alpha: 0.99,
            },
            4.0,
        )
    }).collect::<Vec<_>>();

    Model {
        particles,
        rho: OPTIONS.rho,
        sigma: OPTIONS.sigma,
        beta: OPTIONS.beta,
        camera_angle: 0.0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    // let time_passed = app.duration.since_prev_update.as_secs_f32();

    if app.elapsed_frames() < 2 {
        draw.background().color(BACKGROUND_COLOR);
    }

    draw.rect().wh(win.wh()).xy(win.xy()).color(Alpha { color: BACKGROUND_COLOR, alpha: 0.02 });
    let y_rotation = mat3(
        vec3( model.camera_angle.cos(), 0.0, model.camera_angle.sin()),
        vec3( 0.0,                      1.0, 0.0                     ),
        vec3(-model.camera_angle.sin(), 0.0, model.camera_angle.cos()),
    );
    let x_rotation = mat3(
        vec3(1.0,                      0.0, 0.0                     ),
        vec3(0.0, model.camera_angle.cos(), model.camera_angle.sin()),
        vec3(0.0, -model.camera_angle.sin(), model.camera_angle.cos()),
    );
    let z_rotation = mat3(
        vec3( model.camera_angle.cos(), model.camera_angle.sin(), 0.0),
        vec3(-model.camera_angle.sin(), model.camera_angle.cos(), 0.0),
        vec3( 0.0,                      0.0, 1.0                     ),
    );

    if app.elapsed_frames() < 200000 {
        for particle in model.particles.iter() {
            let rotated_position = x_rotation * y_rotation * z_rotation * particle.position;
            let x = rotated_position.x;
            let y = rotated_position.y;
            let z = rotated_position.z + 30.0;
            draw.ellipse()
                .radius(particle.radius / z * 20.0)
                .color(Alpha { color: particle.color.color, alpha: particle.color.alpha })
                .xy(pt2((x / z) * win.w() / 2.0, (y / z) * win.h() / 2.0));
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let time_passed = app.duration.since_prev_update.as_secs_f32();

    let sigma = model.sigma;
    let rho = model.rho;
    let beta = model.beta;

    for particle in model.particles.iter_mut() {
        let x = particle.position.x;
        let y = particle.position.y;
        let z = particle.position.z;

        // The Lorenz equations
        let dx = sigma * (y - x);
        let dy = x * (rho - z) - y;
        let dz = x * y - beta * z;

        let velocity = vec3(dx, dy, dz);
        particle.position += velocity * time_passed / 10.0;
    }

    model.camera_angle += time_passed * 0.1;
}
