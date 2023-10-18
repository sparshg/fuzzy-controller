use std::collections::HashMap;

use crate::{
    fuzzy::Fuzzy,
    rules::{InputType, Inputs, Op, Output, Rule},
};

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
