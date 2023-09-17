pub struct Bezier {
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
}

impl Bezier {
    pub fn new(p1: (f32, f32), p2: (f32, f32)) -> Self {
        Bezier {
            p0: (0., 0.),
            p1,
            p2,
            p3: (1., 1.),
        }
    }

    pub fn point(&self, t: f32) -> (f32, f32) {
        let t2 = t * t;
        let t3 = t2 * t;
        let mt = 1. - t;
        let mt2 = mt * mt;
        let mt3 = mt2 * mt;
        (
            self.p0.0 * mt3 + 3. * self.p1.0 * mt2 * t + 3. * self.p2.0 * mt * t2 + self.p3.0 * t3,
            self.p0.1 * mt3 + 3. * self.p1.1 * mt2 * t + 3. * self.p2.1 * mt * t2 + self.p3.1 * t3,
        )
    }

    pub fn get_n_points(&self, n: usize) -> Vec<(f32, f32)> {
        let mut points = Vec::with_capacity(n);
        for i in 0..n {
            points.push(self.point(i as f32 / (n - 1) as f32));
        }
        points
    }
}
