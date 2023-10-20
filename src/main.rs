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
use egui_macroquad::egui::{self};
use funcs::*;
use fuzzy::Fuzzy;
use macroquad::prelude::*;
use macroquad_particles::{Emitter, EmitterConfig};
use mamdani::Mamdani;
use std::collections::HashMap;
use ui::{draw_blue_grid, draw_title, smoke};

use rules::{Amp, Diff, InputType, Inputs, Outputs, Th, Vx, Vy, W, X, Y};

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
    let (xn, xp) = (Inputs::X(X::N), Inputs::X(X::P));
    let (yn, yp) = (Inputs::Y(Y::N), Inputs::Y(Y::P));
    let (vyn, vyp) = (Inputs::Vy(Vy::N), Inputs::Vy(Vy::P));
    let (vxn, vxp) = (Inputs::Vx(Vx::N), Inputs::Vx(Vx::P));
    let (thn, thp) = (Inputs::Th(Th::N), Inputs::Th(Th::P));
    let (wn, wp) = (Inputs::W(W::N), Inputs::W(W::P));
    let mut m = Mamdani {
        rules: vec![
            (Outputs::Amp(Amp::Z), yp & vyp),
            (Outputs::Amp(Amp::S), yp & vyn | yn & vyp),
            (Outputs::Amp(Amp::L), yn & vyn),
        ],
        inputs: HashMap::from([
            (
                InputType::Y,
                Fuzzy::new(
                    HashMap::from([(yn, zmf(0., 1.)), (yp, smf(0., 1.))]),
                    -7.0..7.,
                ),
            ),
            (
                InputType::Vy,
                Fuzzy::new(
                    HashMap::from([(vyn, zmf(0.25, 0.75)), (vyp, smf(0.25, 0.75))]),
                    -8.0..8.,
                ),
            ),
        ]),
        output: Fuzzy::new(
            HashMap::from([
                (Outputs::Amp(Amp::Z), gbell(0.3, 3.5, 0.)),
                (Outputs::Amp(Amp::S), gbell(0.2, 3., 0.5)),
                (Outputs::Amp(Amp::L), gbell(0.2, 3., 1.)),
            ]),
            0.0..10.,
        ),
    };

    let mut m2 = Mamdani {
        rules: vec![
            (Outputs::Diff(Diff::NL), thp & vxn & wp),
            (Outputs::Diff(Diff::NM), wp & (vxp & thp | vxn & thn & xp)),
            (Outputs::Diff(Diff::NS), xn & vxn & (thn & wp | thp & wn)),
            (Outputs::Diff(Diff::PS), xp & vxp & (thn & wp | thp & wn)),
            (Outputs::Diff(Diff::PM), wn & (vxn & thn | vxp & thp & xn)),
            (Outputs::Diff(Diff::PL), thn & vxp & wn),
        ],
        inputs: HashMap::from([
            (
                InputType::X,
                Fuzzy::new(
                    HashMap::from([(xn, zmf(0.1, 1.)), (xp, smf(0., 0.9))]),
                    -9.0..9.,
                ),
            ),
            (
                InputType::Vx,
                Fuzzy::new(
                    HashMap::from([(vxn, zmf(0., 1.)), (vxp, smf(0., 1.))]),
                    -4.0..4.,
                ),
            ),
            (
                InputType::Th,
                Fuzzy::new(
                    HashMap::from([(thn, zmf(0., 1.)), (thp, smf(0., 1.))]),
                    -1.0..1.,
                ),
            ),
            (
                InputType::W,
                Fuzzy::new(
                    HashMap::from([(wn, zmf(0., 1.)), (wp, smf(0., 1.))]),
                    -0.6..0.6,
                ),
            ),
        ]),
        output: Fuzzy::new(
            HashMap::from([
                (Outputs::Diff(Diff::NL), gbell(0.2, 2.5, 0.)),
                (Outputs::Diff(Diff::NM), gbell(0.1, 3., 0.1)),
                (Outputs::Diff(Diff::NS), gbell(0.1, 3., 0.3)),
                (Outputs::Diff(Diff::PS), gbell(0.1, 3., 0.7)),
                (Outputs::Diff(Diff::PM), gbell(0.1, 3., 0.9)),
                (Outputs::Diff(Diff::PL), gbell(0.2, 2.5, 1.)),
            ]),
            -10.0..10.,
        ),
    };

    let mut drone = Drone::new(e1, e2);
    let _vingette = Texture2D::from_file_with_format(include_bytes!("../vingette.png"), None);

    loop {
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        draw_blue_grid(0.15, DARKGRAY, 0.001, 3, 0.003);
        drone.update(&mut m, &mut m2, get_frame_time());
        drone.display(WHITE, 0.05);
        // draw_ui(1280., &gr, &gr2);
        egui_macroquad::ui(|ctx: &egui::Context| {
            m.inputs[&InputType::Y].draw(ctx, (10., 60.), (250., 200.), false);
            m.inputs[&InputType::Vy].draw(ctx, (10., 270.), (250., 200.), false);
            m.output.draw(ctx, (10., 480.), (250., 200.), true);
            m2.inputs[&InputType::X].draw(ctx, (screen_width() - 260., 60.), (250., 200.), false);
            m2.inputs[&InputType::Th].draw(ctx, (screen_width() - 260., 270.), (250., 200.), false);
            m2.inputs[&InputType::W].draw(ctx, (screen_width() - 520., 480.), (250., 200.), false);
            m2.inputs[&InputType::Vx].draw(ctx, (screen_width() - 520., 270.), (250., 200.), false);
            m2.output
                .draw(ctx, (screen_width() - 260., 480.), (250., 200.), true);
            draw_title(ctx);
            // .ui(ctx);
        });
        egui_macroquad::draw();
        // draw_vingette(vingette);
        next_frame().await;
    }
}
