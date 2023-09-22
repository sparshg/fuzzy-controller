mod bezier;
mod drone;
mod funcs;
mod fuzzy;
mod mie;
mod pid;
mod state;
use std::{collections::HashMap, f32::consts::PI};

use bezier::Bezier;
use funcs::tri;
use fuzzy::Fuzzy;
use macroquad_particles::{self as particles, Emitter, EmitterConfig};

use drone::Drone;
use macroquad::prelude::*;
use mie::{InputType, Inputs, Mamdani, Output, Y};
use particles::{ColorCurve, Curve};
use pid::PID;

fn smoke() -> particles::EmitterConfig {
    particles::EmitterConfig {
        lifetime: 0.8,
        lifetime_randomness: 0.2,
        amount: 40,
        initial_direction_spread: 0.5,
        initial_direction: vec2(0.0, 1.),
        size_curve: Some(Curve {
            points: bezier::Bezier::new((0., 0.4), (0.4, 1.))
                .get_n_points(20)
                .into_iter()
                .map(|(x, y)| (x, 3. * y + 1.))
                .collect(),
            interpolation: particles::Interpolation::Linear,
            resolution: 20,
        }),
        linear_accel: -4.,
        initial_velocity: 15.,
        size: 0.3,
        size_randomness: 0.1,
        initial_rotation_randomness: PI,
        initial_angular_velocity: rand::gen_range(-1., 1.),
        angular_damping: 0.5,
        colors_curve: ColorCurve {
            start: Color::new(1., 1., 1., 0.4),
            mid: Color::new(1., 1., 1., 0.1),
            end: Color::new(1., 1., 1., 0.),
        },
        ..Default::default()
    }
}
fn window_conf() -> Conf {
    Conf {
        window_title: "Fuzzy Controller".to_string(),
        // fullscreen: true,
        // window_resizable: false,
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let texture = load_texture("smoke.png").await.unwrap();

    let e1 = Emitter::new(EmitterConfig {
        texture: Some(texture.clone()),
        ..smoke()
    });
    let e2 = Emitter::new(EmitterConfig {
        texture: Some(texture.clone()),
        ..smoke()
    });
    let mut drone = Drone::new(e1, e2);
    set_camera(&Camera2D {
        zoom: vec2(100. / screen_width(), 100. / screen_height()),
        ..Default::default()
    });
    let m = Mamdani {
        rules: HashMap::from([
            (Output::None, !Inputs::Y(Y::Pos)),
            (Output::Small, !Inputs::Y(Y::Neg)),
            (Output::Large, Inputs::Y(Y::Pos).into()),
        ]),
        inputs: HashMap::from([(
            InputType::Y,
            Fuzzy::new(HashMap::from([
                (Inputs::Y(Y::Neg), tri(0.0, 0.25, 0.5)),
                (Inputs::Y(Y::Zero), tri(0.25, 0.5, 0.75)),
                (Inputs::Y(Y::Pos), tri(0.5, 0.75, 1.0)),
            ])),
        )]),
        output: Fuzzy::new(HashMap::from([
            (Output::None, tri(0.0, 0.25, 0.5)),
            (Output::Small, tri(0.25, 0.5, 0.75)),
            (Output::Large, tri(0.5, 0.75, 1.0)),
        ])),
    };

    loop {
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        drone.update(get_frame_time());
        drone.display(WHITE, 0.05);
        next_frame().await;
    }
}
