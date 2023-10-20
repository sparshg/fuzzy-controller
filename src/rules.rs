use std::{
    fmt::Display,
    ops::{BitAnd, BitOr, Not},
};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum InputType {
    X,
    Y,
    W,
    Vy,
    Vx,
    Th,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Inputs {
    Y(Y),
    Vy(Vy),
    Vx(Vx),
    X(X),
    Th(Th),
    W(W),
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Outputs {
    Amp(Amp),
    Diff(Diff),
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Amp {
    Z,
    S,
    L,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Diff {
    NL,
    NM,
    NS,
    PS,
    PM,
    PL,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum X {
    N,
    P,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Y {
    N,
    P,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Vy {
    N,
    P,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Vx {
    N,
    P,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Th {
    N,
    P,
}
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum W {
    N,
    P,
}

impl Display for X {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            X::N => write!(f, "X-"),
            X::P => write!(f, "X+"),
        }
    }
}
impl Display for Y {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Y::N => write!(f, "Y-"),
            Y::P => write!(f, "Y+"),
        }
    }
}
impl Display for Vy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vy::N => write!(f, "Vy-"),
            Vy::P => write!(f, "Vy+"),
        }
    }
}
impl Display for Vx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vx::N => write!(f, "Vx-"),
            Vx::P => write!(f, "Vx+"),
        }
    }
}
impl Display for Th {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Th::N => write!(f, "Th-"),
            Th::P => write!(f, "Th+"),
        }
    }
}
impl Display for W {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            W::N => write!(f, "W-"),
            W::P => write!(f, "W+"),
        }
    }
}

impl Display for Inputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Inputs::Y(y) => write!(f, "{}", y),
            Inputs::Vy(y) => write!(f, "{}", y),
            Inputs::Vx(y) => write!(f, "{}", y),
            Inputs::X(y) => write!(f, "{}", y),
            Inputs::Th(y) => write!(f, "{}", y),
            Inputs::W(y) => write!(f, "{}", y),
        }
    }
}

impl Display for Amp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Amp::Z => write!(f, "None"),
            Amp::S => write!(f, "Small"),
            Amp::L => write!(f, "Large"),
        }
    }
}
impl Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Diff::NL => write!(f, "NL"),
            Diff::NM => write!(f, "NM"),
            Diff::NS => write!(f, "NS"),
            Diff::PS => write!(f, "PS"),
            Diff::PM => write!(f, "PM"),
            Diff::PL => write!(f, "PL"),
        }
    }
}

impl Display for Outputs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Outputs::Amp(y) => write!(f, "{}", y),
            Outputs::Diff(y) => write!(f, "{}", y),
        }
    }
}

impl Display for InputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputType::Y => write!(f, "Y"),
            InputType::Vy => write!(f, "Vy"),
            InputType::Vx => write!(f, "Vx"),
            InputType::X => write!(f, "X"),
            InputType::Th => write!(f, "Th"),
            InputType::W => write!(f, "W"),
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

impl From<Inputs> for Rule {
    fn from(val: Inputs) -> Self {
        Rule::new(RuleNode::Input(val), None, None)
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
        Rule::new(RuleNode::Op(Op::Not(|x| 1. - x)), Some(self), None)
    }
}
