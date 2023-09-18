#![allow(non_snake_case)]

use std::f32::consts::PI;

use macroquad::prelude::*;
use macroquad_particles::Emitter;

use crate::{pid::PID, state::State};

pub struct Drone {
    pub enable: bool,
    pub state: State,
    pub steps: i32,
    m: f32,
    M: f32,
    t_m: f32,
    l: f32,
    g: f32,
    Tl: f32,
    Tr: f32,
    smoke1: Emitter,
    smoke2: Emitter,
    pid1: PID,
    pid2: PID,
    pid3: PID,
}

// impl Default for Drone {
//     fn default() -> Self {
//         Drone {
//             // ..Default::default()
//         }
//     }
// }

impl Drone {
    pub fn new(e1: Emitter, e2: Emitter) -> Self {
        let (m, M) = (4., 2.);
        Drone {
            m,
            M,
            t_m: 2. * m + M,
            l: 1.5,
            g: 9.80665,
            Tl: 0.,
            Tr: 0.,
            state: State::default(),
            steps: 5,
            enable: true,
            smoke1: e1,
            smoke2: e2,
            pid1: PID::new(4., 2., 2.),
            pid2: PID::new(0.1, 0., 0.15),
            pid3: PID::new(5., 0., 2.),
            // ..Default::default()
        }
    }

    pub fn update(&mut self, dt: f32) {
        let steps = if dt > 0.02 {
            ((self.steps * 60) as f32 * dt) as i32
        } else {
            self.steps
        };
        let dt = dt / steps as f32;
        for _ in 0..steps {
            // self.error = PI - self.state.th;
            // self.int += self.error * dt;
            // self.F = 0.;
            if self.enable {
                //     self.F = (10.
                //         * (self.error * self.pid.0 + self.int * self.pid.1
                //             - self.state.w * self.pid.2))
                //         .clamp(-self.Fclamp, self.Fclamp);
            }
            (self.Tl, self.Tr) = (0., 0.);
            if is_key_down(KeyCode::Left) {
                //     self.int = 0.
                self.Tl = self.t_m * 12.;
            }
            if is_key_down(KeyCode::Right) {
                self.Tr = self.t_m * 12.;
                //     self.F = self.Finp;
                //     self.int = 0.
            }
            let (a, b) = (
                mouse_position_local().x * screen_width() * 0.01,
                -mouse_position_local().y * screen_height() * 0.01,
                // 5., 4.,
            );
            let amp = self.pid1.output(self.state.x.y + b, dt).clamp(0., 20.);
            let o1 = self.pid2.output(self.state.x.x - a, dt).clamp(-0.8, 0.8);
            // self.state.th = o1;
            let diff = self.pid3.output(o1 - self.state.th, dt).clamp(-10., 10.);
            println!("{}\t{}", o1, diff);
            // let diff = 0. as f32;
            self.Tl = self.t_m * (amp - diff).max(0.);
            self.Tr = self.t_m * (amp + diff).max(0.);
            self.smoke1.config.emitting = self.Tl > 0.;
            self.smoke2.config.emitting = self.Tr > 0.;
            let k1 = self.process_state(self.state);
            let k2 = self.process_state(self.state.after(k1, dt * 0.5));
            let k3 = self.process_state(self.state.after(k2, dt * 0.5));
            let k4 = self.process_state(self.state.after(k3, dt));

            let k_avg = (
                (k1.0 + 2.0 * k2.0 + 2.0 * k3.0 + k4.0) / 6.0,
                (k1.1 + 2.0 * k2.1 + 2.0 * k3.1 + k4.1) / 6.0,
                (k1.2 + 2.0 * k2.2 + 2.0 * k3.2 + k4.2) / 6.0,
                (k1.3 + 2.0 * k2.3 + 2.0 * k3.3 + k4.3) / 6.0,
            );
            self.state.update(k_avg, dt);
        }
    }

    pub fn process_state(&self, state: State) -> (Vec2, Vec2, f32, f32) {
        let (_, v, th, w) = state.unpack();
        // returns (v, vdot, w, wdot)
        (
            v,
            vec2(
                -(self.Tl + self.Tr) * th.sin() / self.t_m,
                -(self.Tl + self.Tr) * th.cos() / self.t_m + self.g,
            ),
            w,
            (self.Tr - self.Tl) / (self.l * (2. * self.m + self.M / 12.)),
        )
    }

    // pub fn get_potential_energy(&self) -> f32 {
    //     // with respect to ground
    //     -self.m3 * self.g * self.l * self.state.th.cos()
    // }
    // pub fn get_kinetic_energy(&self) -> f32 {
    //     0.5 * self.m1 * self.state.v * self.state.v
    //         + 0.5 * self.m2 * self.state.w * self.state.w * self.l * self.l
    //         + self.m3 * self.state.v * self.state.w * self.l * self.state.th.cos()
    // }
    // pub fn get_total_energy(&self) -> f32 {
    //     self.get_potential_energy() + self.get_kinetic_energy()
    // }

    pub fn display(&mut self, color: Color, thickness: f32) {
        // draw a 2d drone with arm length of l
        let (x, y) = self.state.x.into();
        let th = self.state.th;
        let (dx, dy) = (self.l * th.cos(), -self.l * th.sin());

        self.smoke1.config.initial_direction = vec2(-dy, dx);
        self.smoke2.config.initial_direction = vec2(-dy, dx);
        // self.smoke1.config.emitting = is_key_down(KeyCode::Left);
        // self.smoke2.config.emitting = is_key_down(KeyCode::Right);
        self.smoke1
            .draw(vec2(self.state.x.x - dx, self.state.x.y - dy));
        self.smoke2
            .draw(vec2(self.state.x.x + dx, self.state.x.y + dy));
        // draw_circle(x - dx - 0.1, y - dy + 0.1, 0.1, color);
        // draw_circle(x + dx + 0.1, y + dy + 0.1, 0.1, color);

        draw_circle(x, y, 0.1, color);
        draw_line(x, y, x + dx, y + dy, thickness, color);
        draw_circle(x + dx, y + dy, 0.1, color);

        draw_line(x, y, x - dx, y - dy, thickness, color);
        draw_circle(x - dx, y - dy, 0.1, color);
        draw_circle(
            // 5., -4.,
            mouse_position_local().x * screen_width() * 0.01,
            mouse_position_local().y * screen_height() * 0.01,
            0.1,
            color,
        );
        // println!("x: {}, y: {}, th: {}, w: {}", x, y, th, self.state.w);
    }
}
