#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum ColumnType {
    Selector,
    Advice,
    Fixed,
    Instance,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum SpecialType {
    Input,
    Output,
    Field,
    None,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Column {
    pub name: String,
    pub ctype: ColumnType,
    pub stype: SpecialType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Cell {
    pub column: Column,
    pub name: String,
    pub index: i64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum CellExpression {
    Constant(i64),
    CellValue(Cell),
    Negated(Box<CellExpression>),
    Product(Box<CellExpression>, Box<CellExpression>),
    Sum(Box<CellExpression>, Box<CellExpression>),
    Scaled(Box<CellExpression>, i64),
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SimplifiedConstraitSystem {
    pub signals: Vec<Cell>,
    pub columns: Vec<Column>,
    pub regions: Vec<Region>,
    pub instance_count: i64,
    pub gates: Vec<CellExpression>,
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
    // AssignFixed(i64, i64, CellExpression),       // fixed, fix_row(offset), value expression
    EnableSelector(String, i64),               // selector, row(offset)
    AssignAdvice(String, i64, CellExpression), // advice, adv_row(offset), value expression
    AssignAdviceFromConstant(i64, i64, i64),   // advice, adv_row(offset), constant
    AssignAdviceFromInstance(String, i64, String, i64), // advice, adv_row(offset), instance, ins_row
    ConstrainEqual(String, i64, String, i64), // advice, adv_row(offset), advice, adv_row(offset)
    ConstrainConstant(),                      //
}

impl SimplifiedConstraitSystem {
    // pub fn new() -> SimplifiedConstraitSystem {
    //     SimplifiedConstraitSystem { columns: vec![] }
    // }
}
