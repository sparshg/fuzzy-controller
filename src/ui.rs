use std::{collections::HashMap, f32::consts::PI, hash::Hash, ops::Range, rc::Rc};

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
    funcs: Vec<(String, Rc<dyn Fn(f32) -> f32>)>,
    colors: Vec<Color32>,
    range: Range<f32>,
}

impl Graph {
    pub fn new(
        funcs: Vec<(String, Rc<dyn Fn(f32) -> f32>)>,
        colors: Option<Vec<Color32>>,
        range: Option<Range<f32>>,
    ) -> Self {
        Graph {
            colors: colors.unwrap_or_else(|| {
                [
                    Color32::LIGHT_BLUE,
                    Color32::LIGHT_RED,
                    Color32::from_rgb(220, 211, 39),
                ]
                .iter()
                .copied()
                .cycle()
                .take(funcs.len())
                .collect()
            }),
            funcs,
            range: range.unwrap_or(0.0..1.0),
        }
    }

    // pub fn y(&mut self, y: f32) {
    //     self.pos.y = y;
    // }

    // pub fn update(&mut self, track: Vec<f64>) {
    //     assert!(track.len() == self.history.len());
    //     for (i, &v) in track.iter().enumerate() {
    //         self.history[i].push_back(v as f32);
    //         if self.history[i].len() > self.hsize {
    //             self.history[i].pop_front();
    //         }
    //     }
    // }

    pub fn draw(
        &self,
        ctx: &egui::Context,
        pos: (f32, f32),
        size: (f32, f32),
        inp: Option<f32>,
        out: Option<&Vec<(f32, f32)>>,
    ) {
        egui::Window::new(&self.funcs[0].0)
            .frame(Frame {
                inner_margin: egui::Margin::same(0.),
                outer_margin: egui::Margin::same(0.),
                rounding: egui::Rounding::none(),
                fill: Color32::TRANSPARENT,
                shadow: Shadow::NONE,
                stroke: egui::Stroke::new(2., Color32::WHITE),
            })
            .current_pos(pos)
            .default_size(size)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                let _r = self.range.clone();
                Plot::new("plot")
                    .width(size.0)
                    .height(size.1)
                    .show_axes([false, false])
                    .show_background(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show_x(true)
                    .show_y(true)
                    .legend(Legend::default().position(egui::plot::Corner::RightTop))
                    .show(ui, |plot_ui| {
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
                            plot_ui.vline(
                                VLine::new(x.clamp(0., 1.)).color(Color32::GREEN).width(1.5),
                            );
                        }
                        for (i, (_, f)) in self.funcs.iter().enumerate() {
                            plot_ui.line(
                                Line::new(
                                    (0..=100)
                                        .map(|i| {
                                            let x = egui::remap(i as f64, 0.0..=100f64, -0.0..=1.);
                                            [x, f(x as f32) as f64]
                                        })
                                        .collect::<PlotPoints>(),
                                )
                                .width(2.)
                                .color(self.colors[i])
                                .name(&self.funcs[i].0),
                            );
                            if let Some(x) = inp {
                                plot_ui.points(
                                    Points::new([x.clamp(0., 1.) as f64, f(x as f32) as f64])
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
                            plot_ui.hline(
                                HLine::new(out[0].1 as f64).width(1.5).color(Color32::GREEN),
                            );
                            plot_ui.vline(
                                VLine::new(out[0].0 as f64).width(1.5).color(Color32::GREEN),
                            );
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
    }
}

pub fn smoke() -> macroquad_particles::EmitterConfig {
    macroquad_particles::EmitterConfig {
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
            interpolation: macroquad_particles::Interpolation::Linear,
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

pub fn draw_blue_grid(grid: f32, color: Color, thickness: f32, bold_every: i32, bold_thick: f32) {
    push_camera_state();
    set_camera(&Camera2D {
        zoom: vec2(1., screen_width() / screen_height()),
        ..Default::default()
    });
    draw_line(0., -1., 0., 1., bold_thick, color);
    draw_line(-1., 0., 1., 0., bold_thick, color);
    for i in 1..=(1. / grid as f32) as i32 {
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

pub fn draw_vingette(tex: Texture2D) {
    push_camera_state();
    set_default_camera();
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
    pop_camera_state();
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
