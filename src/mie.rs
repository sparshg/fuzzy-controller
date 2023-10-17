use std::{
    collections::HashMap,
    fmt::Display,
    ops::{BitAnd, BitOr, Not},
};

use crate::fuzzy::Fuzzy;
#[derive(Debug)]
enum Op {
    And(fn(f32, f32) -> f32),
    Or(fn(f32, f32) -> f32),
    Not(fn(f32) -> f32),
}

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
pub struct Rule(Vec<Inputs>, Vec<Op>);

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

pub struct Mamdani {
    pub rules: HashMap<Output, Rule>,
    pub inputs: HashMap<InputType, Fuzzy<Inputs>>,
    pub output: Fuzzy<Output>,
}

impl Mamdani {
    pub fn fuzzify(&mut self, crisp: &[(InputType, f32)]) -> HashMap<Inputs, f32> {
        crisp
            .iter()
            .flat_map(|(i, x)| self.inputs.get_mut(i).unwrap().fuzzify(*x))
            .collect()
    }

    pub fn infer(&mut self, inputs: &[(InputType, f32)]) -> f32 {
        let finputs = self.fuzzify(inputs);
        // println!("{:?}", finputs);
        let mut outputs = HashMap::new();
        for (&out, rule) in self.rules.iter() {
            let (ins, ops) = (&rule.0, &rule.1);
            let mut result = finputs[&ins[0]];
            let mut j = 1;
            for op in ops {
                result = match op {
                    Op::And(f) => f(result, finputs[&ins[j]]),
                    Op::Or(f) => f(result, finputs[&ins[j]]),
                    Op::Not(f) => {
                        j -= 1;
                        f(result)
                    }
                };
                j += 1;
            }
            outputs.insert(out, result);
        }
        // println!("{:?}", outputs);
        self.output.defuzzify(outputs)
    }
}
