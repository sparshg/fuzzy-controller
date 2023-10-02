use std::{collections::VecDeque, f32::INFINITY, rc::Rc};

use egui::{
    epaint::Shadow,
    plot::{CoordinatesFormatter, Corner, HLine, Legend, Line, Plot, PlotBounds, PlotPoints},
    Align, Align2, Color32, DragValue, Frame, Layout, Pos2, Slider, Vec2,
};
use egui_macroquad::egui;
use macroquad::prelude::*;

use crate::funcs;

pub struct Graph {
    title: &'static [&'static str],
    pos: Pos2,
    size: Vec2,
    functions: Vec<Rc<dyn Fn(f32) -> f32>>,
    colors: Vec<Color32>,
}

impl Graph {
    pub fn new(
        title: &'static [&'static str],
        pos: (f32, f32),
        size: (f32, f32),
        functions: Vec<Rc<dyn Fn(f32) -> f32>>,
        colors: Option<Vec<Color32>>,
    ) -> Self {
        Graph {
            title,
            pos: pos.into(),
            size: size.into(),
            colors: colors
                .unwrap_or_else(|| (0..functions.len()).map(|_| Color32::WHITE).collect()),
            functions,
        }
    }

    pub fn y(&mut self, y: f32) {
        self.pos.y = y;
    }

    // pub fn update(&mut self, track: Vec<f64>) {
    //     assert!(track.len() == self.history.len());
    //     for (i, &v) in track.iter().enumerate() {
    //         self.history[i].push_back(v as f32);
    //         if self.history[i].len() > self.hsize {
    //             self.history[i].pop_front();
    //         }
    //     }
    // }

    pub fn draw(&self, ctx: &egui::Context) {
        egui::Window::new(self.title[0])
            .frame(Frame {
                inner_margin: egui::Margin::same(0.),
                outer_margin: egui::Margin::same(0.),
                rounding: egui::Rounding::none(),
                fill: Color32::TRANSPARENT,
                shadow: Shadow::NONE,
                stroke: egui::Stroke::new(2., Color32::WHITE),
            })
            .current_pos(self.pos)
            .default_size(self.size)
            .resizable(false)
            .movable(false)
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                Plot::new("example")
                    .width(self.size.x)
                    .height(self.size.y)
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
                        plot_ui.hline(HLine::new(0.).color(Color32::WHITE).width(1.));
                        for (i, f) in self.functions.iter().enumerate() {
                            plot_ui.line(
                                Line::new(
                                    (0..=100)
                                        .map(|i| {
                                            use std::f64::consts::TAU;
                                            let x =
                                                egui::remap(i as f64, 0.0..=100 as f64, -0.0..=1.);
                                            [x, f(x as f32) as f64]
                                        })
                                        .collect::<PlotPoints>(),
                                )
                                .width(2.)
                                .color(self.colors[i]), // .name(self.title[i + 1]),
                            );
                        }
                    })
                    .response
            });
    }
}

pub fn draw_ui(w: f32, forceplt: &mut Graph, forceplt1: &mut Graph) {
    egui_macroquad::ui(|ctx: &egui::Context| {
        // ctx.set_debug_on_hover(true);
        // ctx.set_pixels_per_point(screen_width() / w);
        // forceplt.y(2.);
        // forceplt1.y(2.);
        forceplt.draw(ctx);
        forceplt1.draw(ctx);
    });
    egui_macroquad::draw();
}
