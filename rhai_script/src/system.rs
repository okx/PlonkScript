use crate::engine::*;

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SimplifiedConstraitSystem {
    pub signals: Vec<Cell>,
    pub columns: Vec<Column>,
    pub regions: Vec<Region>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct Region {
    pub name: String,
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Instruction {
    // CopyAdvice(),
    EnableSelector(),                             // selector, row(offset)
    AssignAdvice(i64, i64, CellExpression),       // advice, adv_row(offset), value expression
    AssignAdviceFromConstant(i64, i64, i64),      // advice, adv_row(offset), constant
    AssignAdviceFromInstance(i64, i64, i64, i64), // instance, ins_row, advice, adv_row(offset)
    // AssignFixed(i64, i64, CellExpression),       // fixed, fix_row(offset), value expression
    ConstrainEqual(),    //
    ConstrainConstant(), //
}

impl SimplifiedConstraitSystem {
    // pub fn new() -> SimplifiedConstraitSystem {
    //     SimplifiedConstraitSystem { columns: vec![] }
    // }
}
