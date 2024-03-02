use std::collections::HashMap;

use crate::parser::BorgerType;

pub struct BorgerEnvironment {
    pub data: BorgerMap,
    pub outer: BorgerNode,
}

pub type BorgerNode = Option<Box<BorgerEnvironment>>;
pub type BorgerMap = HashMap<String, Box<BorgerType>>;
