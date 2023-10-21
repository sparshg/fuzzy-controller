use std::{f32::consts::PI, fmt::Display, ops::Range, rc::Rc};

use egui::{
    epaint::Shadow,
    plot::{HLine, Legend, Line, Plot, PlotPoints},
    Color32, Frame,
};
use egui_macroquad::egui::{
    self,
    plot::{Points, Text, VLine},
    FontId, RichText,
};
use macroquad::prelude::*;
use macroquad_particles::{ColorCurve, Curve};

use crate::bezier;
pub struct Graph {
    title: String,
    pub funcs: Vec<(String, Rc<dyn Fn(f32) -> f32>)>,
    colors: Vec<Color32>,
    range: Range<f32>,
    lines: Vec<Vec<[f64; 2]>>,
}

impl Graph {
    pub fn new(
        title: String,
        funcs: Vec<(String, Rc<dyn Fn(f32) -> f32>)>,
        colors: Option<Vec<Color32>>,
        range: Option<Range<f32>>,
    ) -> Self {
        Graph {
            title,
            colors: colors.unwrap_or_else(|| {
                [
                    Color32::LIGHT_BLUE,
                    Color32::LIGHT_RED,
                    Color32::from_rgb(220, 211, 39),
                    Color32::LIGHT_GREEN,
                ]
                .iter()
                .copied()
                .cycle()
                .take(funcs.len())
                .collect()
            }),
            range: range.unwrap_or(0.0..1.0),
            lines: funcs
                .iter()
                .map(|(_, f)| {
                    (0..=100)
                        .map(|i| {
                            let x = egui::remap(i as f64, 0.0..=100f64, -0.0..=1.);
                            [x, f(x as f32) as f64]
                        })
                        .collect()
                })
                .collect(),
            funcs,
        }
    }

    pub fn draw(
        &self,
        ctx: &egui::Context,
        pos: (f32, f32),
        size: (f32, f32),
        inp: Option<f32>,
        legends: bool,
        out: Option<&Vec<(f32, f32)>>,
    ) -> Vec<f32> {
        let mut memberships = vec![0.; self.funcs.len()];
        egui::Window::new(&self.title)
            .frame(Frame {
                inner_margin: egui::Margin::same(0.),
                outer_margin: egui::Margin::same(0.),
                rounding: egui::Rounding::none(),
                shadow: Shadow::NONE,
                ..Default::default()
            })
            .current_pos(pos)
            .default_size(size)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                let _r = self.range.clone();
                let mut p = Plot::new("plot")
                    .width(size.0)
                    .height(size.1)
                    .show_axes([false, false])
                    // .show_background(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show_x(true)
                    .show_y(true);
                if legends {
                    p = p.legend(
                        Legend::default()
                            .background_alpha(0.)
                            .position(egui::plot::Corner::RightTop),
                    );
                }
                p.show(ui, |plot_ui| {
                    plot_ui.text(
                        Text::new(
                            [0.02, 0.05].into(),
                            RichText::new(self.range.start.to_string())
                                .color(Color32::WHITE)
                                .font(FontId::proportional(12.0)),
                        )
                        .color(Color32::WHITE),
                    );
                    plot_ui.text(
                        Text::new(
                            [0.98, 0.05].into(),
                            RichText::new(self.range.end.to_string())
                                .color(Color32::WHITE)
                                .font(FontId::proportional(12.0)),
                        )
                        .color(Color32::WHITE),
                    );
                    if let Some(x) = inp {
                        plot_ui.vline(VLine::new(x.clamp(0., 1.)).color(Color32::GREEN).width(1.5));
                    }
                    for (i, (_, f)) in self.funcs.iter().enumerate() {
                        plot_ui.line(
                            Line::new(self.lines[i].clone())
                                .width(2.)
                                .color(self.colors[i])
                                .name(&self.funcs[i].0),
                        );
                        if let Some(x) = inp {
                            memberships[i] = f(x);
                            plot_ui.points(
                                Points::new([x.clamp(0., 1.) as f64, f(x) as f64])
                                    // .name(format!("Hello"))
                                    .filled(true)
                                    .radius(4.)
                                    .color(self.colors[i]),
                            )
                        }
                    }
                    if let Some(out) = out {
                        plot_ui.line(
                            Line::new(
                                out.iter()
                                    .skip(1)
                                    .map(|(x, y)| [*x as f64, *y as f64])
                                    .collect::<PlotPoints>(),
                            )
                            .fill(0.)
                            .width(4.)
                            .color(Color32::GREEN), // .name(&self.title[i]),
                        );
                        plot_ui.hline(HLine::new(out[0].1 as f64).width(1.5).color(Color32::GREEN));
                        plot_ui.vline(VLine::new(out[0].0 as f64).width(1.5).color(Color32::GREEN));
                        plot_ui.points(
                            Points::new([out[0].0 as f64, out[0].1 as f64])
                                // .name(format!("Hello"))
                                .filled(true)
                                .radius(6.)
                                .color(Color32::GREEN)
                                .shape(egui::plot::MarkerShape::Cross),
                        );
                    }
                })
                .response
            });
        memberships
    }
}

pub fn smoke() -> macroquad_particles::EmitterConfig {
    macroquad_particles::EmitterConfig {
        lifetime: 0.8,
        lifetime_randomness: 0.1,
        amount: 50,
        initial_direction_spread: 0.4,
        initial_direction: vec2(0.0, 1.),
        size_curve: Some(Curve {
            points: bezier::Bezier::new((0., 0.4), (0.4, 1.))
                .get_n_points(20)
                .into_iter()
                .map(|(x, y)| (x, 3. * y + 1.))
                .collect(),
            interpolation: macroquad_particles::Interpolation::Linear,
            resolution: 20,
        }),
        linear_accel: -3.,
        initial_velocity: 12.,
        size: 0.35,
        size_randomness: 0.05,
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

pub fn draw_blue_grid(grid: f32, color: Color, thickness: f32, bold_every: i32, bold_thick: f32) {
    push_camera_state();
    set_camera(&Camera2D {
        zoom: vec2(1., screen_width() / screen_height()),
        ..Default::default()
    });
    draw_line(0., -1., 0., 1., bold_thick, color);
    draw_line(-1., 0., 1., 0., bold_thick, color);
    for i in 1..=(1. / grid) as i32 {
        let thickness = if i % bold_every == 0 {
            bold_thick
        } else {
            thickness
        };
        draw_line(i as f32 * grid, -1., i as f32 * grid, 1., thickness, color);
        draw_line(
            -i as f32 * grid,
            -1.,
            -i as f32 * grid,
            1.,
            thickness,
            color,
        );
        draw_line(-1., i as f32 * grid, 1., i as f32 * grid, thickness, color);
        draw_line(
            -1.,
            -i as f32 * grid,
            1.,
            -i as f32 * grid,
            thickness,
            color,
        );
    }
    pop_camera_state();
}

pub fn draw_rules(
    spacing: f32,
    (x, y): (f32, f32),
    size: (usize, usize),
    labels: (
        &[impl Display],
        &[impl Display],
        &[impl Display],
        &[impl Display],
    ),
    fuzzied: (&[f32], &[f32], Option<f32>),
) {
    // make a grid table of size.0 * size.1 with each square of edge spacing

    let (w, h) = (spacing * size.0 as f32, spacing * size.1 as f32);
    for i in 0..labels.0.len() {
        let m = measure_text(&format!("{}", labels.0[i]), None, 16, 1.);
        draw_text(
            &format!("{}", labels.0[i]),
            (x + i as f32 * spacing + spacing / 2. - m.width / 2.).round(),
            (y - m.height).round(),
            16.,
            WHITE,
        );
    }
    for i in 0..labels.1.len() {
        let m = measure_text(&format!("{}", labels.1[i]), None, 16, 1.);
        draw_text(
            &format!("{}", labels.1[i]),
            (x - m.width - 8.).round(),
            (y + i as f32 * spacing + spacing / 2.).round(),
            16.,
            WHITE,
        );
    }
    for i in 0..size.0 {
        for j in 0..size.1 {
            let m = measure_text(&format!("{}", labels.2[i * size.1 + j]), None, 24, 1.);
            let font = Font::default();
            font.set_filter(FilterMode::Nearest);
            let c = (fuzzied.0[i]
                .min(fuzzied.1[j])
                .min(fuzzied.2.unwrap_or_else(|| 1.))
                * 255.) as u8;

            draw_text_ex(
                &format!("{}", labels.2[i * size.1 + j]),
                (x + (i as f32 + 0.5) * spacing - m.width / 2.).round(),
                (y + (j as f32 + 0.5) * spacing + m.height / 2.).round(),
                TextParams {
                    font_size: 24_u16,
                    font_scale: 1.0,
                    font,
                    color: Color::from_rgba(0, c, c, 255),
                    ..Default::default()
                },
            );
        }
    }
    for i in 0..labels.3.len() {
        let m = measure_text(&format!("{}", labels.3[i]), None, 16, 1.);
        draw_text(
            &format!("{}", labels.3[i]),
            (x + (w - m.width - (labels.3.len() - 1) as f32 * spacing) / 2. + i as f32 * spacing)
                .round(),
            (y + h + m.height + 8.).round(),
            16.,
            WHITE,
        );
    }
    for i in 0..=size.0 {
        draw_line(
            x + i as f32 * spacing,
            y,
            x + i as f32 * spacing,
            y + h,
            2.,
            WHITE,
        );
    }
    for i in 0..=size.1 {
        draw_line(
            x,
            y + i as f32 * spacing,
            x + w,
            y + i as f32 * spacing,
            2.,
            WHITE,
        );
    }
}

pub fn draw_vingette(tex: Texture2D) {
    draw_texture_ex(
        tex,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        },
    );
    // pop_camera_state();
}

pub fn draw_title(ctx: &egui::Context) {
    egui::Window::new("Fuzzy Controller")
        .frame(Frame {
            inner_margin: egui::Margin::same(0.),
            outer_margin: egui::Margin::same(0.),
            rounding: egui::Rounding::none(),
            fill: Color32::TRANSPARENT,
            shadow: Shadow::NONE,
            stroke: egui::Stroke::NONE,
        })
        .current_pos((45., 15.))
        .default_size((200., 50.))
        .resizable(false)
        .movable(false)
        .collapsible(false)
        .title_bar(false)
        .show(ctx, |ui| {
            ui.label(
                RichText::new("Fuzzy Controller")
                    .color(Color32::WHITE)
                    .font(FontId::proportional(24.0)),
            )
        });
}
