use std::collections::HashMap;

use crate::{
    fuzzy::Fuzzy,
    rules::{InputType, Inputs, Op, Output, Rule, RuleNode},
};

pub struct Mamdani {
    pub rules: Vec<(Rule, Output)>,
    pub inputs: HashMap<InputType, Fuzzy<Inputs>>,
    pub output: Fuzzy<Output>,
}

impl Mamdani {
    pub fn fuzzify(&mut self, crisp: &[(InputType, f32)]) -> HashMap<Inputs, f32> {
        // println!("{:?}", crisp);
        crisp
            .iter()
            .flat_map(|(i, x)| self.inputs.get_mut(i).unwrap().fuzzify(*x))
            .collect()
    }

    fn resolve(&self, rule: &Rule, finputs: &HashMap<Inputs, f32>) -> f32 {
        match &rule.val {
            RuleNode::Input(i) => finputs[&i],
            RuleNode::Op(o) => {
                let left = self.resolve(rule.left.as_ref().expect("Op at end of tree"), finputs);
                if let Op::Not(f) = o {
                    if rule.right.is_some() {
                        panic!("Not op must have only one (left) child");
                    }
                    return f(left);
                }
                let right = self.resolve(rule.right.as_ref().expect("Op at end of tree"), finputs);
                match o {
                    Op::And(f) => f(left, right),
                    Op::Or(f) => f(left, right),
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn infer(&mut self, inputs: &[(InputType, f32)]) -> f32 {
        let finputs = self.fuzzify(inputs);
        // println!("{:?}", finputs);
        let mut outputs = HashMap::new();
        // println!("{:?}", finputs);
        for (rule, out) in self.rules.iter() {
            let a = self.resolve(rule, &finputs);
            // println!("{:?}", out);
            println!("{:?}", a);
            outputs.insert(*out, a);
        }
        // println!("{:?}", outputs);
        self.output.defuzzify(outputs)
    }
}
