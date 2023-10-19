use std::{collections::HashMap, f32::consts::PI, hash::Hash, rc::Rc};

use egui::{
    epaint::Shadow,
    plot::{CoordinatesFormatter, Corner, HLine, Legend, Line, Plot, PlotPoints},
    Color32, Frame,
};
use egui_macroquad::egui::{
    self,
    plot::{Points, VLine},
};
use macroquad::prelude::*;
use macroquad_particles::{ColorCurve, Curve};

use crate::bezier;
pub struct Graph<V>
where
    V: Eq + Hash + Copy,
{
    title: Vec<String>,
    functions: Rc<HashMap<V, Box<dyn Fn(f32) -> f32>>>,
    colors: Vec<Color32>,
}

impl<V> Graph<V>
where
    V: Eq + Hash + Copy,
{
    pub fn new(
        title: Vec<String>,
        functions: Rc<HashMap<V, Box<dyn Fn(f32) -> f32>>>,
        colors: Option<Vec<Color32>>,
    ) -> Self {
        Graph {
            title,
            colors: colors
                .unwrap_or_else(|| (0..functions.len()).map(|_| Color32::WHITE).collect()),
            functions,
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
        egui::Window::new(&self.title[0])
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
                Plot::new("plot")
                    .width(size.0)
                    .height(size.1)
                    .show_axes([false, false])
                    .show_background(false)
                    .allow_drag(false)
                    .allow_zoom(false)
                    .allow_scroll(false)
                    .allow_boxed_zoom(false)
                    .show_x(false)
                    .show_y(false)
                    .coordinates_formatter(
                        Corner::LeftBottom,
                        CoordinatesFormatter::new(|&point, _| {
                            format!("x: {:.3} y: {:.3}", point.x, point.y)
                        }),
                    )
                    .legend(Legend::default().position(egui::plot::Corner::RightBottom))
                    .show(ui, |plot_ui| {
                        // plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                        //     [0., -clamp * 1.1],
                        //     [self.hsize as f64, clamp * 1.1],
                        // ));
                        if let Some(x) = inp {
                            plot_ui
                                .vline(VLine::new(x.clamp(0., 1.)).color(Color32::GREEN).width(1.));
                        }
                        for (i, (_, f)) in self.functions.iter().enumerate() {
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
                                .color(self.colors[i]), // .name(&self.title[i]),
                            );
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
                            plot_ui.hline(HLine::new(out[0].1 as f64).color(Color32::GREEN));
                            plot_ui.vline(VLine::new(out[0].0 as f64).color(Color32::GREEN));
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

// pub fn draw_ui<V>(_w: f32, forceplt: &Graph<V>, forceplt1: &Graph<V>)
// where
//     V: Eq + Hash + Copy,
// {
//     egui_macroquad::ui(|ctx: &egui::Context| {
//         // ctx.set_debug_on_hover(true);
//         // ctx.set_pixels_per_point(screen_width() / w);
//         // forceplt.y(2.);
//         // forceplt1.y(2.);
//         // forceplt.draw(ctx);
//         // forceplt1.draw(ctx);
//     });
//     egui_macroquad::draw();
// }

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
