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

use crate::rules::Rule;

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
    let (xn, xz, xp) = (Inputs::X(X::N), Inputs::X(X::Z), Inputs::X(X::P));
    let (yn, yp) = (Inputs::Y(Y::N), Inputs::Y(Y::P));
    let (vyn, vyp) = (Inputs::Vy(Vy::N), Inputs::Vy(Vy::P));
    let (vxn, vxp) = (Inputs::Vx(Vx::N), Inputs::Vx(Vx::P));
    let (thn, thz, thp) = (Inputs::Th(Th::N), Inputs::Th(Th::Z), Inputs::Th(Th::P));
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
                    InputType::Y,
                    HashMap::from([(yn, zmf(0., 1.)), (yp, smf(0., 1.))]),
                    -7.0..7.,
                ),
            ),
            (
                InputType::Vy,
                Fuzzy::new(
                    InputType::Vy,
                    HashMap::from([(vyn, zmf(0.25, 0.75)), (vyp, smf(0.25, 0.75))]),
                    -8.0..8.,
                ),
            ),
        ]),
        output: Fuzzy::new(
            "Amp",
            HashMap::from([
                (Outputs::Amp(Amp::Z), gbell(0.3, 3.5, 0.)),
                (Outputs::Amp(Amp::S), gbell(0.2, 3., 0.5)),
                (Outputs::Amp(Amp::L), gbell(0.2, 3., 1.)),
            ]),
            0.0..10.,
        ),
    };

    let tp = vec![
        vxp & thn & xn,
        vxp & thn & xz,
        vxp & thn & xp,
        vxp & thz & xn,
        vxp & thz & xz,
        vxp & thz & xp,
        vxp & thp & xn,
        vxp & thp & xz,
        vxp & thp & xp,
    ];
    let tn = vec![
        vxn & thn & xn,
        vxn & thn & xz,
        vxn & thn & xp,
        vxn & thz & xn,
        vxn & thz & xz,
        vxn & thz & xp,
        vxn & thp & xn,
        vxn & thp & xz,
        vxn & thp & xp,
    ];

    fn r(v: &Vec<Rule>, i: usize) -> Rule {
        v[i].clone()
    }

    let mut m2 = Mamdani {
        rules: vec![
            // (Outputs::Diff(Diff::NL), thp & xn & wp & vxn),
            // (
            //     Outputs::Diff(Diff::NM),
            //     xn & (wp & (thn & vxn | thp & vxp) | wn & thp & vxn),
            // ),
            // (
            //     Outputs::Diff(Diff::NS),
            //     xn & (wn & (thn & vxn | thp & vxp) | wp & thn & vxp) | xp & thp & wp & vxn,
            // ),
            // (
            //     Outputs::Diff(Diff::PS),
            //     xp & (wp & (thp & vxp | thn & vxn) | wn & thp & vxn) | xn & thn & wn & vxp,
            // ),
            // (
            //     Outputs::Diff(Diff::PM),
            //     xp & (wn & (thp & vxp | thn & vxn) | wp & thn & vxp),
            // ),
            // (Outputs::Diff(Diff::PL), thn & xp & wn & vxp),
            (Outputs::Diff(Diff::NL), r(&tn, 6) | r(&tn, 3)),
            (
                Outputs::Diff(Diff::NM),
                r(&tn, 8) | r(&tp, 6) | r(&tp, 7) | r(&tn, 7),
            ),
            (
                Outputs::Diff(Diff::NS),
                r(&tp, 3) | r(&tp, 8) | r(&tn, 4) | wp,
            ),
            (Outputs::Diff(Diff::Z), xz & thz),
            (
                Outputs::Diff(Diff::PS),
                r(&tp, 4) | r(&tn, 5) | r(&tn, 0) | wn,
            ),
            (
                Outputs::Diff(Diff::PM),
                r(&tp, 0) | r(&tp, 1) | r(&tn, 2) | r(&tn, 1),
            ),
            (Outputs::Diff(Diff::PL), r(&tp, 5) | r(&tp, 2)),
            // (Outputs::Diff(Diff::NL), vxn & thp & xn),
            // (Outputs::Diff(Diff::NM), vxp & thp | vxn & thp & (xz | xp)),
            // (Outputs::Diff(Diff::NS), thz & xn | vxn & thz & xz | wp),
            // (Outputs::Diff(Diff::Z), xz & thz),
            // (Outputs::Diff(Diff::PS), thz & xp | vxp & thz & xz | wn),
            // (Outputs::Diff(Diff::PM), vxn & thn | vxp & thn & (xz | xn)),
            // (Outputs::Diff(Diff::PL), vxp & thn & xp),

            // (Outputs::Diff(Diff::NL), xn & thp),
            // (Outputs::Diff(Diff::NM), xz & thp),
            // (Outputs::Diff(Diff::NS), xn & (thn | thz)),
            // (Outputs::Diff(Diff::Z), xz & thz),
            // (Outputs::Diff(Diff::PS), xp & (thp | thz)),
            // (Outputs::Diff(Diff::PM), xz & thn),
            // (Outputs::Diff(Diff::PL), xp & thn),

            // (Outputs::Diff(Diff::NL), thp & vxn & wp & (xp | xn)),
            // (Outputs::Diff(Diff::NM), wp & (vxp & thp | vxn & thn & xp)),
            // (Outputs::Diff(Diff::NS), xn & vxn & (thn & wp | thp & wn)),
            // (Outputs::Diff(Diff::PS), xp & vxp & (thn & wp | thp & wn)),
            // (Outputs::Diff(Diff::PM), wn & (vxn & thn | vxp & thp & xn)),
            // (Outputs::Diff(Diff::PL), thn & vxp & wn & (xp | xn)),
        ],
        inputs: HashMap::from([
            (
                InputType::X,
                Fuzzy::new(
                    InputType::X,
                    HashMap::from([
                        (xn, zmf(0., 0.9)),
                        (xz, gbell(0.2, 1.5, 0.5)),
                        (xp, smf(0.1, 1.)),
                    ]),
                    -10.0..10.,
                ),
            ),
            (
                InputType::Vx,
                Fuzzy::new(
                    InputType::Vx,
                    HashMap::from([(vxn, zmf(0., 1.)), (vxp, smf(0., 1.))]),
                    -4.0..4.,
                ),
            ),
            (
                InputType::Th,
                Fuzzy::new(
                    InputType::Th,
                    HashMap::from([
                        (thn, zmf(0., 0.9)),
                        (thz, gbell(0.15, 2., 0.5)),
                        (thp, smf(0.1, 1.)),
                    ]),
                    -0.5..0.5,
                ),
            ),
            (
                InputType::W,
                Fuzzy::new(
                    InputType::W,
                    HashMap::from([(wn, zmf(0., 1.)), (wp, smf(0., 1.))]),
                    -0.6..0.6,
                ),
            ),
        ]),
        output: Fuzzy::new(
            "Diff",
            HashMap::from([
                (Outputs::Diff(Diff::NL), gbell(0.1, 3., 0.)),
                (Outputs::Diff(Diff::NM), gbell(0.1, 3., 0.3)),
                (Outputs::Diff(Diff::NS), gbell(0.08, 3., 0.4)),
                (Outputs::Diff(Diff::Z), gbell(0.02, 3., 0.5)),
                (Outputs::Diff(Diff::PS), gbell(0.08, 3., 0.6)),
                (Outputs::Diff(Diff::PM), gbell(0.1, 3., 0.7)),
                (Outputs::Diff(Diff::PL), gbell(0.1, 3., 1.)),
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
        draw_blue_grid(0.075, DARKGRAY, 0.001, 6, 0.003);
        drone.update(&mut m, &mut m2, get_frame_time());
        drone.display(WHITE, 0.05);
        // draw_ui(1280., &gr, &gr2);
        egui_macroquad::ui(|ctx: &egui::Context| {
            let H = 200.;
            let W = 250.;
            let gap = 10.;
            let title_gap = 0.;
            let f = (2. * W - gap) / (3. * W);
            let h = f * H;
            let w = f * W;
            let top = screen_height() * 0.5 - (gap + h + H * 0.5);
            m.inputs[&InputType::Y].draw(ctx, (gap, top), (w, h), false);
            m2.inputs[&InputType::X].draw(ctx, (w + 2. * gap, top), (w, h), false);
            m2.inputs[&InputType::Th].draw(ctx, (2. * w + 3. * gap, top), (w, h), false);
            m.inputs[&InputType::Vy].draw(ctx, (gap, top + h + gap + title_gap), (w, h), false);
            m2.inputs[&InputType::Vx].draw(
                ctx,
                (w + 2. * gap, top + h + gap + title_gap),
                (w, h),
                false,
            );
            m2.inputs[&InputType::W].draw(
                ctx,
                (2. * w + 3. * gap, top + h + gap + title_gap),
                (w, h),
                false,
            );
            m.output
                .draw(ctx, (gap, top + 2. * (h + gap + title_gap)), (W, H), true);
            m2.output.draw(
                ctx,
                (W + 2. * gap, top + 2. * (h + gap + title_gap)),
                (W, H),
                true,
            );
            draw_title(ctx);
            // .ui(ctx);
        });
        egui_macroquad::draw();
        // draw_vingette(vingette);
        next_frame().await;
    }
}
