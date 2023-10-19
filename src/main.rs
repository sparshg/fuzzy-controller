mod bezier;
mod drone;
mod funcs;
mod fuzzy;
mod mamdani;
mod pid;
mod rules;
mod state;
mod ui;
use drone::Drone;
use egui_macroquad::egui;
use funcs::{cliff, mount, tri};
use fuzzy::Fuzzy;
use macroquad::prelude::*;
use macroquad_particles::{Emitter, EmitterConfig};
use mamdani::Mamdani;
use std::{collections::HashMap};
use ui::smoke;


use rules::{InputType, Inputs, Output, Yv, Y};

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
        texture: Some(texture),
        ..smoke()
    });
    let e2 = Emitter::new(EmitterConfig {
        texture: Some(texture),
        ..smoke()
    });
    set_camera(&Camera2D {
        zoom: vec2(100. / screen_width(), 100. / screen_height()),
        ..Default::default()
    });
    let (yn, yz, yp) = (Inputs::Y(Y::N), Inputs::Y(Y::Z), Inputs::Y(Y::P));
    let (vn, vz, vp) = (Inputs::Yv(Yv::N), Inputs::Yv(Yv::Z), Inputs::Yv(Yv::P));
    let mut m = Mamdani {
        rules: vec![
            // (yp.into(), Output::None),
            // (yz.into(), Output::Small),
            // (yn.into(), Output::Large),
            (yz & vp | yp & vp | yz & vz, Output::None),
            (yz & vn | yp & vn | yn & vp, Output::Small),
            (yn & vn | yn & vz, Output::Large),
        ],
        inputs: HashMap::from([
            (
                InputType::Y,
                Fuzzy::new(
                    HashMap::from([
                        // (Inputs::Y(Y::Neg), zmf(0.25, 0.5)),
                        // (Inputs::Y(Y::Zero), gauss(0.4, 0.6)),
                        // (Inputs::Y(Y::Pos), smf(0.5, 0.75)),
                        (Inputs::Y(Y::N), cliff(0.25, 0.5)),
                        (Inputs::Y(Y::Z), tri(0.25, 0.5, 0.75)),
                        (Inputs::Y(Y::P), mount(0.5, 0.75)),
                    ]),
                    -5.0..5.,
                ),
            ),
            (
                InputType::Yv,
                Fuzzy::new(
                    HashMap::from([
                        // (Inputs::Y(Y::Neg), zmf(0.25, 0.5)),
                        // (Inputs::Y(Y::Zero), gauss(0.4, 0.6)),
                        // (Inputs::Y(Y::Pos), smf(0.5, 0.75)),
                        (Inputs::Yv(Yv::N), cliff(0.25, 0.5)),
                        (Inputs::Yv(Yv::Z), tri(0.25, 0.5, 0.75)),
                        (Inputs::Yv(Yv::P), mount(0.5, 0.75)),
                    ]),
                    -10.0..10.,
                ),
            ),
        ]),
        output: Fuzzy::new(
            HashMap::from([
                (Output::None, tri(0.0, 0.25, 0.5)),
                (Output::Small, tri(0.25, 0.5, 0.75)),
                (Output::Large, tri(0.5, 0.75, 1.0)),
            ]),
            0.0..11.,
        ),
    };
    // println!("{:?}", yz & vp | yp & vp);
    let mut drone = Drone::new(e1, e2);
    loop {
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        drone.update(&mut m, get_frame_time());
        drone.display(WHITE, 0.05);
        // draw_ui(1280., &gr, &gr2);
        egui_macroquad::ui(|ctx: &egui::Context| {
            m.inputs[&InputType::Y].draw(ctx, (10., 10.), (250., 200.), false);
            m.inputs[&InputType::Yv].draw(ctx, (10., 220.), (250., 200.), false);
            m.output.draw(ctx, (10., 430.), (250., 200.), true);
        });
        egui_macroquad::draw();
        next_frame().await;
    }
}
