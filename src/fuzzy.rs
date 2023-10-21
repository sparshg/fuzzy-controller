use std::{collections::HashMap, fmt::Display, hash::Hash, ops::Range, rc::Rc};

use egui_macroquad::egui::Context;

use crate::ui::Graph;

pub struct Fuzzy<V>
where
    V: Eq + Hash + Copy + Display,
{
    members: usize,
    pub functions: HashMap<V, Rc<dyn Fn(f32) -> f32>>,
    range: Range<f32>,
    graph: Graph,
    last_input: f32,
    last_output: Vec<(f32, f32)>,
    resolution: usize,
}

impl<V> Fuzzy<V>
where
    V: Eq + Hash + Copy + Display,
{
    pub fn new(
        input_type: impl Display,
        functions: HashMap<V, Rc<dyn Fn(f32) -> f32>>,
        range: Range<f32>,
    ) -> Fuzzy<V> {
        // let f = Rc::new(functions);
        let mut titles: Vec<(String, Rc<dyn Fn(f32) -> f32>)> = functions
            .iter()
            .map(|(&x, y)| (x.to_string(), Rc::clone(y)))
            .collect();
        titles.sort_unstable_by_key(|(s, _)| {
            let order = "NZLPSM-0+";
            s.chars()
                .map(|c| order.find(c).unwrap_or(0) as u8)
                .collect::<Vec<_>>()
        });
        Fuzzy {
            members: functions.len(),
            graph: Graph::new(input_type.to_string(), titles, None, Some(range.clone())),
            range,
            functions,
            last_input: 0.,
            resolution: 100,
            last_output: vec![(0., 0.); 101],
        }
    }
    pub fn fuzzify(&mut self, x: f32) -> Vec<(V, f32)> {
        self.last_input = (x - self.range.start) / (self.range.end - self.range.start);
        let mut result = Vec::with_capacity(self.members);
        for (&l, f) in self.functions.iter() {
            result.push((l, f(self.last_input)));
        }
        result
    }

    pub fn defuzzify(&mut self, acuts: HashMap<V, f32>) -> f32 {
        if acuts.len() != self.members {
            panic!(
                "Length of alpha cuts ({}) != Length of membership functions ({})",
                acuts.len(),
                self.members
            );
        }
        let (mut mx, mut my, mut m) = (0., 0., 0.);
        for i in 0..self.resolution {
            let x = i as f32 / (self.resolution - 1) as f32;
            let y = acuts
                .iter()
                .fold(0f32, |acc, (l, &a)| acc.max(self.functions[l](x).min(a)));
            self.last_output[i + 1] = (x, y);
            mx += y * x;
            my += y * y;
            m += y;
            // println!("{} {} {}", mx, my, m);
        }
        mx /= m;
        my /= 2. * m;
        self.last_output[0] = (mx, my);
        mx * (self.range.end - self.range.start) + self.range.start
    }

    pub fn draw(
        &self,
        ctx: &Context,
        pos: (f32, f32),
        size: (f32, f32),
        is_output: bool,
    ) -> Vec<f32> {
        self.graph.draw(
            ctx,
            pos,
            size,
            if is_output {
                None
            } else {
                Some(self.last_input)
            },
            if is_output {
                Some(&self.last_output)
            } else {
                None
            },
        )
    }
}
