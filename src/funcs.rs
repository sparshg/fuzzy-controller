use std::rc::Rc;

pub fn tri(a: f32, p: f32, b: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| {
        if x < a {
            0.
        } else if x < p {
            (x - a) / (p - a)
        } else if x < b {
            (b - x) / (b - p)
        } else {
            0.
        }
    })
}

pub fn cliff(a: f32, b: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| {
        if x < a {
            1.
        } else if x < b {
            (b - x) / (b - a)
        } else {
            0.
        }
    })
}

pub fn mount(a: f32, b: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| {
        if x < a {
            0.
        } else if x < b {
            (x - a) / (b - a)
        } else {
            1.
        }
    })
}

pub fn zmf(a: f32, b: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| {
        if x < a {
            1.
        } else if x < (a + b) / 2. {
            1. - 2. * (x - a) * (x - a) / ((b - a) * (b - a))
        } else if x < b {
            2. * (x - b) * (x - b) / ((b - a) * (b - a))
        } else {
            0.
        }
    })
}

pub fn smf(a: f32, b: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| {
        if x < a {
            0.
        } else if x < (a + b) / 2. {
            2. * (x - a) * (x - a) / ((b - a) * (b - a))
        } else if x < b {
            1. - 2. * (x - b) * (x - b) / ((b - a) * (b - a))
        } else {
            1.
        }
    })
}

pub fn gauss(a: f32, b: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| {
        let c = (a + b) / 2.;
        let d = (b - a) / 2.;
        (-((x - c) / d).powi(2)).exp()
    })
}

pub fn gbell(a: f32, b: f32, c: f32) -> Rc<dyn Fn(f32) -> f32> {
    Rc::new(move |x| 1. / (1. + ((x - c) / a).abs().powi(2 * b as i32)))
}
