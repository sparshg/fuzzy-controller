use macroquad::prelude::Vec2;

#[derive(Clone, PartialEq, Debug)]

pub struct State {
    pub x: Vec2,
    pub v: Vec2,
    pub w: f32,
    pub th: f32,
}

impl Default for State {
    fn default() -> Self {
        Self::from(Vec2::ZERO, Vec2::ZERO, 0., 0.0)
    }
}

impl State {
    pub fn from(x: Vec2, v: Vec2, th: f32, w: f32) -> Self {
        State { x, v, th, w }
    }

    pub fn update(&mut self, (v, vdot, w, wdot): (Vec2, Vec2, f32, f32), dt: f32) {
        self.w += wdot * dt;
        self.th += w * dt;
        // self.th = (self.th % (2. * PI) + 2. * PI) % (2. * PI);
        self.v += vdot * dt;
        self.x += v * dt;
    }

    pub fn after(&self, (v, vdot, w, wdot): (Vec2, Vec2, f32, f32), dt: f32) -> State {
        let mut new_state = self.clone();
        new_state.update((v, vdot, wdot, w), dt);
        println!("{:?}", new_state);
        new_state
    }

    pub fn unpack(&self) -> (Vec2, Vec2, f32, f32) {
        (self.x, self.v, self.th, self.w)
    }
}
