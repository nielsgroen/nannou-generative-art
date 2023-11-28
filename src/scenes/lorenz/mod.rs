use std::cell::RefCell;

use lazy_static::lazy_static;
use nannou::prelude::*;
use nannou::{App, Frame};
use nannou::app::Builder;
use nannou::color::Alpha;
use nannou::event::Update;
use nannou::geom::pt3;
use rand::{Rng, thread_rng};
use clap::Parser;
use async_trait::async_trait;
use nannou::glam::Vec4Swizzles;
use nannou::wgpu::{DeviceDescriptor, Limits};
use crate::{Args, SceneArgs};
use crate::math_3d::Camera;
use crate::math_3d::controls::{CameraControls, CenteredCameraControls};
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
    event_fn: fn(app: &App, model: &mut Model, event: Event),
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


#[async_trait]
impl Scene for LorenzScene {
    type SceneOptions = LorenzOptions;
    type Model = Model;


    fn new_scene(_options: &Self::SceneOptions) -> Self {
        Self {
            model_fn: model,
            update_fn: update,
            view_fn: view,
            event_fn: event,
        }
    }

    async fn app(&self) -> Builder<Self::Model> {
        let model = Model::new();

        thread_local!(static MODEL: RefCell<Option<Model>> = Default::default());
        MODEL.with(|m| m.borrow_mut().replace(model));

        let builder = app::Builder::new_async(|app| {
            Box::new(async move {
                let device_descriptor = DeviceDescriptor {
                    limits: Limits {
                        max_texture_dimension_2d: 8192,
                        ..Limits::downlevel_webgl2_defaults()
                    },
                    ..Default::default()
                };

                app.new_window()
                    .device_descriptor(device_descriptor)
                    .view(view)
                    .title("n0ls Lorenz")
                    .build_async()
                    .await
                    .unwrap();

                MODEL.with(|m| m.borrow_mut().take().unwrap())
            })
        });

        builder
            .update(self.update_fn)
            .event(self.event_fn)

        // Old code
        // nannou::app(self.model_fn)
        //     .update(self.update_fn)
        //     .event(self.event_fn)
        //     .simple_window(self.view_fn)
        //     .size(1800, 1200)
    }
}


pub struct Model {
    pub particles: Vec<Particle3>,
    pub rho: f32,  // https://en.wikipedia.org/wiki/Lorenz_system
    pub sigma: f32,
    pub beta: f32,
    // pub camera_angle: f32,
    pub camera: Camera,
    pub camera_controls: Box<dyn CameraControls>,
}

impl Model {
    pub fn new() -> Self {
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

        // let cam_position = pt3(300.0, 10.0, 10.0);
        let cam_position = pt3(10.926121, 7.850386, 232.43593);
        let view_center = vec3(0.0, 0.0, 0.0);
        let view_direction = (view_center - vec3(10.0, 10.0, 10.0)).normalize();
        let up = vec3(0.0, 1.0, 0.0);
        let right = view_direction.cross(up).normalize();
        let up = right.cross(view_direction).normalize(); // calculate the real up vector

        #[cfg(not(target_family = "wasm"))]
        return Model {
            particles,
            rho: OPTIONS.rho,
            sigma: OPTIONS.sigma,
            beta: OPTIONS.beta,
            camera: Camera::new_perspective(
                cam_position,
                view_direction,
                up,
                0.25,
                // win.w() / win.h(),
                1.0, // so that WASM works
            ),
            camera_controls: Box::new(CenteredCameraControls::new(
                view_center,
                100.0,
                100.0,
            )),
        };

        #[cfg(target_family = "wasm")]
        Model {
            particles,
            rho: 28.0,
            sigma: 10.0,
            beta: 2.66667,
            camera: Camera::new_perspective(
                cam_position,
                view_direction,
                up,
                0.25,
                1.0, // so that WASM works
            ),
            camera_controls: Box::new(CenteredCameraControls::new(
                view_center,
                100.0,
                100.0,
            )),
        }
    }
}


fn model(app: &App) -> Model {
    // let win = app.window_rect();
    Model::new()
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();
    // let time_passed = app.duration.since_prev_update.as_secs_f32();

    if app.elapsed_frames() < 2 {
        draw.background().color(BACKGROUND_COLOR);
    }

    draw.rect().wh(win.wh()).xy(win.xy()).color(Alpha { color: BACKGROUND_COLOR, alpha: 0.02 });

    let transformation_matrix = model.camera.get_transformation_matrix();

    for particle in model.particles.iter() {
        let new_particle_position = particle.transform_position(transformation_matrix);
        let nannou_coordinate_transformation = Mat4::from_diagonal(vec4(win.x.end, win.y.end, 1.0, 1.0));
        let new_particle_position = nannou_coordinate_transformation * new_particle_position;
        draw.ellipse()
            .radius(particle.radius / new_particle_position.w * 50.0)
            .color(particle.color)
            .xy(new_particle_position.xy() / new_particle_position.w);
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

    // model.camera_angle += time_passed * 0.1;
    model.camera_controls.apply_to_camera(&mut model.camera, app);
}

fn event(app: &App, model: &mut Model, event: Event) {
    model.camera_controls.event(app, event);

    // #[cfg(target_family = "wasm")]
    // {
    //     use crate::log;
    //
    //     log(&format!("Camera position: {:?}", model.camera.position));
    //     log(&format!("Camera direction: {:?}", model.camera.view_direction));
    //     log(&format!("Camera up: {:?}", model.camera.up));
    // }
}