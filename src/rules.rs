use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, Not},
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum InputType {
    Y,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Inputs {
    Y(Y),
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Output {
    None,
    Small,
    Large,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Y {
    Neg,
    Zero,
    Pos,
}

impl Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Y::Neg => write!(f, "Y-"),
            Y::Zero => write!(f, "Y0"),
            Y::Pos => write!(f, "Y+"),
        }
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inputs::Y(y) => write!(f, "{}", y),
        }
    }
}

impl Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Output::None => write!(f, "None"),
            Output::Small => write!(f, "Small"),
            Output::Large => write!(f, "Large"),
        }
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Y => write!(f, "Y"),
        }
    }
}

#[derive(Debug)]
pub enum Op {
    And(fn(f32, f32) -> f32),
    Or(fn(f32, f32) -> f32),
    Not(fn(f32) -> f32),
}

#[derive(Debug)]
pub struct Rule(pub Vec<Inputs>, pub Vec<Op>);

impl Into<Rule> for Inputs {
    fn into(self) -> Rule {
        Rule(vec![self], vec![])
    }
}

impl BitAnd for Inputs {
    type Output = Rule;

    fn bitand(self, rhs: Inputs) -> Self::Output {
        Rule(vec![self, rhs], vec![Op::And(f32::min)])
    }
}

impl BitAnd<Rule> for Inputs {
    type Output = Rule;

    fn bitand(self, mut rhs: Self::Output) -> Self::Output {
        rhs.0.push(self);
        rhs.1.push(Op::And(f32::min));
        rhs
    }
}

impl BitAnd<Inputs> for Rule {
    type Output = Rule;

    fn bitand(mut self, rhs: Inputs) -> Self::Output {
        self.0.push(rhs);
        self.1.push(Op::And(f32::min));
        self
    }
}

impl BitOr for Inputs {
    type Output = Rule;

    fn bitor(self, rhs: Inputs) -> Self::Output {
        Rule(vec![self, rhs], vec![Op::Or(f32::max)])
    }
}

impl BitOr<Rule> for Inputs {
    type Output = Rule;

    fn bitor(self, mut rhs: Self::Output) -> Self::Output {
        rhs.0.push(self);
        rhs.1.push(Op::Or(f32::max));
        rhs
    }
}

impl BitOr<Inputs> for Rule {
    type Output = Rule;

    fn bitor(mut self, rhs: Inputs) -> Self::Output {
        self.0.push(rhs);
        self.1.push(Op::Or(f32::min));
        self
    }
}

impl Not for Inputs {
    type Output = Rule;

    fn not(self) -> Self::Output {
        Rule(vec![self], vec![Op::Not(|x| 1. - x)])
    }
}

impl Not for Rule {
    type Output = Rule;

    fn not(mut self: Rule) -> Self::Output {
        self.1.push(Op::Not(|x| 1. - x));
        self
    }
}
