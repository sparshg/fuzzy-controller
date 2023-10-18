use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, Not},
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum InputType {
    Y,
    Yv,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Inputs {
    Y(Y),
    Yv(Yv),
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Output {
    None,
    Small,
    Large,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Y {
    N,
    Z,
    P,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Yv {
    N,
    Z,
    P,
}

impl Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Y::N => write!(f, "Y-"),
            Y::Z => write!(f, "Y0"),
            Y::P => write!(f, "Y+"),
        }
    }
}
impl Display for Yv {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Yv::N => write!(f, "Yv-"),
            Yv::Z => write!(f, "Yv0"),
            Yv::P => write!(f, "Yv+"),
        }
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inputs::Y(y) => write!(f, "{}", y),
            Inputs::Yv(y) => write!(f, "{}", y),
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
            InputType::Yv => write!(f, "Yv"),
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
pub enum RuleNode {
    Input(Inputs),
    Op(Op),
}

#[derive(Debug)]
pub struct Rule {
    pub val: RuleNode,
    pub left: Option<Box<Rule>>,
    pub right: Option<Box<Rule>>,
}

impl Rule {
    fn new(val: RuleNode, left: Option<Rule>, right: Option<Rule>) -> Self {
        Self {
            val,
            left: left.map(Box::new),
            right: right.map(Box::new),
        }
    }
}

impl Into<Rule> for Inputs {
    fn into(self) -> Rule {
        Rule::new(RuleNode::Input(self), None, None)
    }
}

impl BitAnd for Inputs {
    type Output = Rule;

    fn bitand(self, rhs: Inputs) -> Self::Output {
        Rule::new(
            RuleNode::Op(Op::And(f32::min)),
            Some(self.into()),
            Some(rhs.into()),
        )
    }
}

impl BitAnd<Rule> for Inputs {
    type Output = Rule;

    fn bitand(self, rhs: Rule) -> Self::Output {
        Rule::new(
            RuleNode::Op(Op::And(f32::min)),
            Some(self.into()),
            Some(rhs),
        )
    }
}

impl BitAnd<Inputs> for Rule {
    type Output = Rule;

    fn bitand(self, rhs: Inputs) -> Self::Output {
        Rule::new(
            RuleNode::Op(Op::And(f32::min)),
            Some(self),
            Some(rhs.into()),
        )
    }
}

impl BitAnd<Rule> for Rule {
    type Output = Rule;

    fn bitand(self, rhs: Rule) -> Self::Output {
        Rule::new(RuleNode::Op(Op::And(f32::min)), Some(self), Some(rhs))
    }
}

impl BitOr for Inputs {
    type Output = Rule;

    fn bitor(self, rhs: Inputs) -> Self::Output {
        Rule::new(
            RuleNode::Op(Op::Or(f32::max)),
            Some(self.into()),
            Some(rhs.into()),
        )
    }
}

impl BitOr<Rule> for Inputs {
    type Output = Rule;

    fn bitor(self, rhs: Rule) -> Self::Output {
        Rule::new(RuleNode::Op(Op::Or(f32::max)), Some(self.into()), Some(rhs))
    }
}

impl BitOr<Inputs> for Rule {
    type Output = Rule;

    fn bitor(self, rhs: Inputs) -> Self::Output {
        Rule::new(RuleNode::Op(Op::Or(f32::max)), Some(self), Some(rhs.into()))
    }
}

impl BitOr<Rule> for Rule {
    type Output = Rule;

    fn bitor(self, rhs: Rule) -> Self::Output {
        Rule::new(RuleNode::Op(Op::Or(f32::max)), Some(self), Some(rhs))
    }
}

impl Not for Inputs {
    type Output = Rule;

    fn not(self) -> Self::Output {
        Rule::new(RuleNode::Op(Op::Not(|x| 1. - x)), Some(self.into()), None)
    }
}

impl Not for Rule {
    type Output = Rule;

    fn not(self: Rule) -> Self::Output {
        Rule::new(RuleNode::Op(Op::Not(|x| 1. - x)), Some(self.into()), None)
    }
}
