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
use ui::{draw_blue_grid, smoke};

use rules::{Amp, Diff, InputType, Inputs, Outputs, Th, Vx, Vy, W, X, Y};

use crate::{
    rules::Rule,
    ui::{draw_rules, draw_vingette},
};

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
    let (nl, nm, ns, z, ps, pm, pl) = (
        Outputs::Diff(Diff::NL),
        Outputs::Diff(Diff::NM),
        Outputs::Diff(Diff::NS),
        Outputs::Diff(Diff::Z),
        Outputs::Diff(Diff::PS),
        Outputs::Diff(Diff::PM),
        Outputs::Diff(Diff::PL),
    );
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
            (nl, r(&tn, 6) | r(&tn, 3)),
            (nm, r(&tn, 8) | r(&tp, 6) | r(&tp, 7) | r(&tn, 7)),
            (ns, r(&tp, 3) | r(&tp, 8) | r(&tn, 4) | wp),
            (z, xz & thz),
            (ps, r(&tp, 4) | r(&tn, 5) | r(&tn, 0) | wn),
            (pm, r(&tp, 0) | r(&tp, 1) | r(&tn, 2) | r(&tn, 1)),
            (pl, r(&tp, 5) | r(&tp, 2)),
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
    let vingette = Texture2D::from_file_with_format(include_bytes!("../vingette.png"), None);

    loop {
        if is_key_down(KeyCode::Escape) || is_key_down(KeyCode::Q) {
            break;
        }

        clear_background(BLACK);
        draw_blue_grid(0.075, DARKGRAY, 0.001, 6, 0.002);
        drone.update(&mut m, &mut m2, get_frame_time());
        drone.display(WHITE, 0.05);

        let mut fuzzied: HashMap<InputType, Vec<f32>> = HashMap::new();
        egui_macroquad::ui(|ctx: &egui::Context| {
            let H = 200.;
            let W = 250.;
            let gap = 10.;
            let title_gap = 0.;
            let f = (2. * W - gap) / (3. * W);
            let h = f * H;
            let w = f * W;
            let top = 10.;
            fuzzied.insert(
                InputType::Y,
                m.inputs[&InputType::Y].draw(ctx, (gap, top), (w, h), false),
            );
            fuzzied.insert(
                InputType::X,
                m2.inputs[&InputType::X].draw(ctx, (w + 2. * gap, top), (w, h), false),
            );
            fuzzied.insert(
                InputType::Th,
                m2.inputs[&InputType::Th].draw(ctx, (2. * w + 3. * gap, top), (w, h), false),
            );
            fuzzied.insert(
                InputType::Vy,
                m.inputs[&InputType::Vy].draw(ctx, (gap, top + h + gap + title_gap), (w, h), false),
            );
            fuzzied.insert(
                InputType::Vx,
                m2.inputs[&InputType::Vx].draw(
                    ctx,
                    (w + 2. * gap, top + h + gap + title_gap),
                    (w, h),
                    false,
                ),
            );
            fuzzied.insert(
                InputType::W,
                m2.inputs[&InputType::W].draw(
                    ctx,
                    (2. * w + 3. * gap, top + h + gap + title_gap),
                    (w, h),
                    false,
                ),
            );
            m.output
                .draw(ctx, (gap, top + 2. * (h + gap + title_gap)), (W, H), true);
            m2.output.draw(
                ctx,
                (W + 2. * gap, top + 2. * (h + gap + title_gap)),
                (W, H),
                true,
            );
        });
        // println!("{:?}", fuzzied);
        // panic!("");
        egui_macroquad::draw();

        push_camera_state();
        set_default_camera();
        draw_rules(
            50.,
            (40., 525.),
            (3, 3),
            (
                &[xn, xz, xp],
                &[thn, thz, thp],
                &[pm, pm, pl, ns, ps, pl, nm, nm, ns],
                &[vxp],
            ),
            (
                &fuzzied[&InputType::X],
                &fuzzied[&InputType::Th],
                Some(fuzzied[&InputType::Vx][1]),
            ),
        );
        draw_rules(
            50.,
            (230., 525.),
            (3, 3),
            (
                &[xn, xz, xp],
                &[thn, thz, thp],
                &[ps, pm, pm, nl, ns, ps, nl, nm, nm],
                &[vxn],
            ),
            (
                &fuzzied[&InputType::X],
                &fuzzied[&InputType::Th],
                Some(fuzzied[&InputType::Vx][0]),
            ),
        );
        draw_rules(
            50.,
            (420., 520.),
            (2, 2),
            (
                &[vyn, vyp],
                &[yn, yp],
                &[
                    Outputs::Amp(Amp::L),
                    Outputs::Amp(Amp::S),
                    Outputs::Amp(Amp::S),
                    Outputs::Amp(Amp::Z),
                ],
                &[""],
            ),
            (&fuzzied[&InputType::Vy], &fuzzied[&InputType::Y], None),
        );

        draw_rules(
            50.,
            (420., 630.),
            (2, 1),
            (&[""], &[""], &[ns, ps], &[wp, wn]),
            (&fuzzied[&InputType::W], &[1.], None),
        );

        // draw_rules(50., (200., 500.), (3, 3));
        // draw_rules(50., (390., 500.), (2, 2));
        // if is_key_down(KeyCode::Space) {
        draw_vingette(vingette);
        // }
        pop_camera_state();
        next_frame().await;
    }
}
