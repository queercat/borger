use crate::parser::BorgerType;
use std::collections::HashMap;

pub type BorgerNode = Option<Box<BorgerEnvironment>>;
pub type BorgerMap = HashMap<String, BorgerType>;

pub struct BorgerEnvironment {
    pub value: BorgerMap,
}

impl Default for BorgerEnvironment {
    fn default() -> Self {
        let mut environment = BorgerEnvironment {
            value: BorgerMap::new(),
        };

        environment.value.insert(
            String::from("+"),
            BorgerType::Function(|value| -> BorgerType {
                let values = match value {
                    BorgerType::List(value) => value,
                    _ => panic!(""),
                };

                let mut accumulator = 0f64;

                values
                    .iter()
                    .map(|v| -> f64 {
                        let value = match v {
                            BorgerType::Number(v) => v,
                            _ => panic!("Invalid value."),
                        };

                        return *value;
                    })
                    .for_each(|v| accumulator += v);

                BorgerType::Number(accumulator)
            }),
        );

        environment.value.insert(
            String::from("-"),
            BorgerType::Function(|value| -> BorgerType {
                let values = match value {
                    BorgerType::List(value) => value,
                    _ => panic!(""),
                };

                let mut accumulator = 0f64;

                values
                    .iter()
                    .map(|v| -> f64 {
                        let value = match v {
                            BorgerType::Number(v) => v,
                            _ => panic!("Invalid value."),
                        };

                        return *value;
                    })
                    .for_each(|v| accumulator -= v);

                BorgerType::Number(accumulator)
            }),
        );

        environment.value.insert(
            String::from("/"),
            BorgerType::Function(|value| -> BorgerType {
                let values = match value {
                    BorgerType::List(value) => value,
                    _ => panic!(""),
                };

                let mut accumulator = 0f64;

                values
                    .iter()
                    .map(|v| -> f64 {
                        let value = match v {
                            BorgerType::Number(v) => v,
                            _ => panic!("Invalid value."),
                        };

                        return *value;
                    })
                    .for_each(|v| accumulator /= v);

                BorgerType::Number(accumulator)
            }),
        );

        environment.value.insert(
            String::from("*"),
            BorgerType::Function(|value| -> BorgerType {
                let values = match value {
                    BorgerType::List(value) => value,
                    _ => panic!(""),
                };

                let mut accumulator = 0f64;

                values
                    .iter()
                    .map(|v| -> f64 {
                        let value = match v {
                            BorgerType::Number(v) => v,
                            _ => panic!("Invalid value."),
                        };

                        return *value;
                    })
                    .for_each(|v| accumulator *= v);

                BorgerType::Number(accumulator)
            }),
        );

        environment
    }
}

pub fn eval_ast(ast: BorgerType, environment: &mut BorgerEnvironment) -> Box<BorgerType> {
    if let BorgerType::List(ast) = ast {
        let (first, rest) = ast
            .split_first()
            .expect("Did you pass the right amount of arguments?");

        let symbol = match first {
            BorgerType::Symbol(first) => first,
            _ => panic!("Expected a symbol but instead found something else."),
        };

        if *symbol == "\'" {
            if rest.len() > 1 {
                panic!("Too many arguments for quoting.")
            }

            return Box::from(
                rest.first()
                    .expect("Expected a value to quote but found nothing instead")
                    .to_owned(),
            );
        }

        let arguments: BorgerType = BorgerType::List(
            rest.to_vec()
                .iter()
                .map(|v| *eval_ast(v.to_owned(), environment))
                .collect(),
        );

        let value = environment
            .value
            .get(symbol)
            .expect("{symbol} does not exist within this scope");

        if let BorgerType::Function(value) = value {
            return Box::from(value(arguments));
        } else {
            return Box::from(value.to_owned());
        }
    }

    Box::from(ast)
}
