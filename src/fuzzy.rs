use std::{collections::HashMap, hash::Hash, ops::Range};

pub struct Fuzzy<T, V>
where
    T: Fn(f32) -> f32,
    V: Eq + Hash + Copy,
{
    members: usize,
    functions: HashMap<V, T>,
    range: Range<f32>,
}

impl<T, V> Fuzzy<T, V>
where
    T: Fn(f32) -> f32,
    V: Eq + Hash + Copy,
{
    pub fn new(functions: HashMap<V, T>) -> Fuzzy<T, V> {
        Fuzzy {
            members: functions.len(),
            functions,
            range: 0f32..1f32,
        }
    }
    pub fn fuzzify(&self, x: f32) -> HashMap<V, f32> {
        if x < self.range.start || x > self.range.end {
            panic!(
                "Value {} is out of range [{}, {}]",
                x, self.range.start, self.range.end
            );
        }
        let mut result = HashMap::with_capacity(self.members);
        for (&l, f) in self.functions.iter() {
            result.insert(l, f(x));
        }
        result
    }

    pub fn defuzzify(&self, acuts: HashMap<V, f32>, resolution: usize) -> (f32, f32) {
        if acuts.len() != self.members {
            panic!(
                "Length of alpha cuts ({}) != Length of membership functions ({})",
                acuts.len(),
                self.members
            );
        }
        let (mut mx, mut my, mut m) = (0., 0., 0.);
        for i in 0..=resolution {
            let x = (i as f32 / resolution as f32) * (self.range.end - self.range.start)
                + self.range.start;
            let y = acuts
                .iter()
                .fold(0f32, |acc, (l, &a)| acc.max(self.functions[l](x).min(a)));
            mx += y * x;
            my += y * y;
            m += y;
        }
        mx /= m;
        my /= m;
        (mx, my)
    }
}
