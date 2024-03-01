use std::collections::HashMap;

use crate::parser::BorgerType;

pub struct BorgerEnvironment {
    pub data: BorgerMap,
    pub outer: BorgerNode,
}

pub type BorgerNode = Option<Box<BorgerEnvironment>>;
pub type BorgerMap = HashMap<String, Box<BorgerType>>;

impl BorgerEnvironment {
    pub fn find(&self, symbol: String) -> &BorgerType {
        let value = self.data.get(&symbol);

        match value {
            Some(v) => v,
            None => match &self.outer {
                Some(outer) => outer.find(symbol),
                None => &BorgerType::Null,
            },
        }
    }

    pub fn set(&mut self, symbol: String, value: BorgerType) {
        self.data.insert(symbol, Box::new(value));
    }
}

pub fn eval_ast<'a>(ast: &BorgerType, environment: &'a mut BorgerEnvironment) -> &'a BorgerType {
    let evaluated = match ast {
        BorgerType::Symbol(ast) => environment.find(ast.to_owned()),
        a => panic!("Unknown node type {:?}", a),
    };

    evaluated
}
