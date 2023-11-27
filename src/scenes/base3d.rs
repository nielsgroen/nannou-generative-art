use std::cell::RefCell;

use nannou::prelude::*;
use nannou::{App, Frame};
use nannou::app::Builder;
use nannou::color::Alpha;
use nannou::glam::Vec4Swizzles;
use async_trait::async_trait;
use nannou::wgpu::{DeviceDescriptor, Limits};
use crate::math_3d::Camera;
use crate::math_3d::controls::{CameraControls, CenteredCameraControls, MouseBasedCenteredCameraControls};
use crate::particle::Particle3;
use crate::scenes::Scene;


const BACKGROUND_COLOR: Srgb<u8> = BLANCHEDALMOND;

pub struct Base3DScene {
    model_fn: fn(app: &App) -> Model,
    update_fn: fn(app: &App, model: &mut Model, _update: Update),
    view_fn: fn(app: &App, model: &Model, frame: Frame),
    event_fn: fn(app: &App, model: &mut Model, event: Event),
}

pub struct Model {
    camera: Camera,
    camera_controls: Box<dyn CameraControls>,
    points: Vec<Particle3>
}

impl Model {
    pub fn new() -> Self {
        Self {
            camera: Camera::new_perspective(
                vec3(3.0, 3.0, 3.0),
                vec3(-1.0, -1.0, -1.0).normalize(),
                vec3(0.0, 1.0, 0.0),
                0.25 * PI,
                1.0,
            ),
            camera_controls: Box::new(MouseBasedCenteredCameraControls::new(
                vec3(0.0, 0.0, 0.0),
                1.0,
            )),
            points: vec![
                Particle3::new(
                    pt3(1.0, 1.0, 1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(1.0, 1.0, -1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(1.0, -1.0, 1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(1.0, -1.0, -1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(-1.0, 1.0, 1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(-1.0, 1.0, -1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(-1.0, -1.0, 1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),
                Particle3::new(
                    pt3(-1.0, -1.0, -1.0),
                    Alpha {
                        color: rgb(0.0, 0.0, 0.0),
                        alpha: 1.0,
                    },
                    1.0
                ),

            ],
        }
    }
}


#[async_trait]
impl Scene for Base3DScene {
    type SceneOptions = ();
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
                    .title("n0ls Base3D")
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


fn model(_app: &App) -> Model {
    Model::new()
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();
    let aspect_ratio = win.x.len() / win.y.len();

    model.camera.aspect_ratio(aspect_ratio);

    model.camera_controls.apply_to_camera(&mut model.camera, app);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let win = app.window_rect();
    let draw = app.draw();

    draw.background().color(BACKGROUND_COLOR);
    let transformation_matrix = model.camera.get_transformation_matrix();

    for particle in model.points.iter() {
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

fn event(app: &App, model: &mut Model, event: Event) {
    model.camera_controls.event(app, event);
}
