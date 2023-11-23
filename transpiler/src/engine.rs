use rhai::EvalAltResult;

use crate::util::convert_to_value;
use crate::{system::*, CONTEXT};

pub const DEFAULT_INSTANCE_COLUMN_NAME: &str = "defins";

pub trait EngineExt {
    fn register_plonk_script(&mut self);
}

impl EngineExt for rhai::Engine {
    #[warn(unused_must_use)]
    fn register_plonk_script(&mut self) {
        // when expression is complex, may occur ExprTooDeep error
        self.set_max_expr_depths(320, 320);

        let _ = &mut self
        .register_type_with_name::<Column>("Column")
        .register_indexer_get(Column::get_field)
        .register_indexer_set(Column::set_field)
        .register_type_with_name::<Cell>("Cell")
        .register_get("value",Cell::get_value)
        .register_fn("init_input", init_input)
        .register_fn("init_output", init_output)
        .register_fn("set_output", set_output)
        .register_fn("init_advice_column", init_advice_column)
        .register_fn("init_selector_column", init_selector_column)
        .register_fn("init_fixed_column", init_fixed_column)
        .register_fn("define_region", define_region)
        .register_fn("assign_constraint", assign_constraint)
        // .register_fn("assign_constraint", assign_constraint_int)
        .register_fn("assign_constraint", assign_constraint_cell_ce)
        .register_fn("assign_constraint", assign_constraint_string)
        .register_fn("assign_common", assign_common_string)
        .register_fn("assign_common", assign_common_ce)
        // .register_fn("assign_only", assign_only)
        // .register_fn("assign_only", assign_only_int)
        .register_fn("set_gate", set_gate)
        .register_fn("set_gate", set_gate_col_ce)
        .register_fn("enable_selector", enable_selector)
        .register_fn("set_parameter", set_parameter)
        .register_fn("set_parameter", set_parameter_i64)
        .register_fn("inspect", inspect)
        // .register_indexer_set(TestStruct::set_field)
        ;
        register_operator(self);

        define_region("default".to_string());
    }
}

impl Column {
    pub fn get_field(&mut self, index: i64) -> Cell {
        let name = self.get_field_name(index);
        unsafe {
            if CONTEXT.cells.contains_key(&name) {
                return CONTEXT.cells[&name].clone();
            }
        }

        Cell {
            name,
            index,
            value: Some("0".to_string()),
            column: self.clone(),
        }
    }
    pub fn set_field(&mut self, index: i64, value: Cell) {
        let name = self.get_field_name(index);
        let cell = Cell {
            name: name.clone(),
            index,
            value: value.value,
            ..value
        };
        unsafe {
            let entry = CONTEXT.cells.entry(name).or_insert(cell.clone()); //.and_modify(||cell);
            *entry = cell;
        }
    }

    fn get_field_name(&self, index: i64) -> String {
        let region = unsafe { CONTEXT.regions.last().unwrap().clone() };
        format!("{}[{}]_{}_{}", self.name, index, region.name, region.id)
    }
}

impl Cell {
    fn get_value(&mut self) -> String {
        self.value.clone().unwrap()
    }
}

fn get_lastest_instance_index() -> i64 {
    let a = unsafe { CONTEXT.instance_count };
    unsafe { CONTEXT.instance_count += 1 };
    a
}

fn init_input(v: &str) -> Cell {
    // println!("init_input({})", v);
    let cell = Cell {
        name: v.to_string(),
        index: get_lastest_instance_index(),
        value: unsafe { Some(CONTEXT.inputs[v].clone()) },
        column: Column {
            name: DEFAULT_INSTANCE_COLUMN_NAME.to_string(),
            ctype: ColumnType::Instance,
            stype: SpecialType::Input,
        },
    };
    unsafe {
        CONTEXT.signals.push(cell.clone());
    }
    cell
}
fn init_output(v: String) -> Cell {
    // println!("init_output({})", v);
    let cell = Cell {
        name: v.to_string(),
        index: get_lastest_instance_index(),
        value: None,
        column: Column {
            name: DEFAULT_INSTANCE_COLUMN_NAME.to_string(),
            ctype: ColumnType::Instance,
            stype: SpecialType::Output,
        },
    };
    unsafe {
        CONTEXT.signals.push(cell.clone());
    }
    cell
}
fn set_output(name: String, cell: Cell) {
    unsafe {
        if let Some(pos) = CONTEXT.signals.iter().position(|x| x.name == name) {
            CONTEXT.signals.splice(pos..(pos + 1), vec![cell]);
        }
    }
}
fn init_advice_column(v: String) -> Column {
    // println!("init_advice_column({})", v);
    let col = Column {
        name: v.to_string(),
        ctype: ColumnType::Advice,
        stype: SpecialType::None,
    };
    unsafe {
        CONTEXT.columns.push(col.clone());
    }
    col
}
fn init_selector_column(v: String) -> Column {
    // println!("init_selector_column({})", v);
    let col = Column {
        name: v.to_string(),
        ctype: ColumnType::Selector,
        stype: SpecialType::None,
    };
    unsafe {
        CONTEXT.columns.push(col.clone());
    }
    col
}
fn init_fixed_column(v: String) -> Column {
    // println!("init_selector_column({})", v);
    let col = Column {
        name: v.to_string(),
        ctype: ColumnType::Fixed,
        stype: SpecialType::None,
    };
    unsafe {
        CONTEXT.columns.push(col.clone());
    }
    col
}
fn define_region(v: String) {
    // println!("define_region({})", v);
    unsafe {
        CONTEXT.regions.push(Region {
            name: v,
            id: CONTEXT.regions.len() as i64,
            instructions: vec![],
        });
    }
    ()
}

// a <== b
fn assign_constraint(a: &mut Cell, b: Cell) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    a.value = b.value.clone();
    push_instruction_to_last_region(match (a.column.ctype, b.column.ctype) {
        (ColumnType::Advice, ColumnType::Instance) => {
            vec![Instruction::AssignAdviceFromInstance(a.clone(), b.clone())]
        }
        (ColumnType::Instance, ColumnType::Advice) => {
            vec![Instruction::AssignAdviceFromInstance(b.clone(), a.clone())]
        }
        (_, _) => {
            vec![
                Instruction::AssignAdvice(a.clone(), CellExpression::CellValue(b.clone())),
                Instruction::ConstrainEqual(a.clone(), b.clone()),
            ]
        }
    });
    a.clone()
}

// a <== b (b is expresion, e.g. b1 + b2)
fn assign_constraint_cell_ce(a: &mut Cell, b: CellExpression) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    a.value = convert_to_value(b.clone());
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), b)]);
    a.clone()
}
fn assign_constraint_string(a: &mut Cell, b: String) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    let cb = CellExpression::Constant(b);
    a.value = convert_to_value(cb.clone());
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), cb)]);
    a.clone()
}
fn assign_common_string(a: &mut Cell, b: String) -> Cell {
    match a.column.ctype {
        ColumnType::Fixed => {
            let cb = CellExpression::Constant(b);
            a.value = convert_to_value(cb.clone());
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), cb)]);
            a.clone()
        }
        ColumnType::Instance => {
            let cb = CellExpression::Constant(b);
            a.value = convert_to_value(cb.clone());
            a.clone()
            //warning
        }
        o => todo!("{:?}", o),
    }
}
// fn assign_common_ce(a: &mut Cell, b: CellExpression) -> Cell {
fn assign_common_ce(a: &mut Cell, b: CellExpression) -> Result<Cell, Box<EvalAltResult>> {
    match a.column.ctype {
        ColumnType::Fixed => {
            a.value = convert_to_value(b.clone());
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), b)]);
            Ok(a.clone())
        }
        ColumnType::Advice => {
            a.value = convert_to_value(b.clone());
            push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), b)]);
            Ok(a.clone())
        }
        o => todo!("{:?}", o),
    }
}
fn push_instruction_to_last_region(a: Vec<Instruction>) {
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        for i in a {
            region.instructions.push(i);
        }
    }
}
// fn assign_constraint_int(a: Column, b: i64) {
//     println!("assign_constraint({:?}, {})", a, b);
//     ()
// }
// fn assign_constraint_int(a: Cell, b: i64) {
//     println!("assign_constraint({:?}, {})", a, b);
//     push_instruction_to_last_region(Instruction::AssignAdvice(
//         a.column.name,
//         a.index,
//         CellExpression::Constant(b),
//     ));
// }
// fn assign_only(a: Column, b: Column) {
//     println!("assign_only({:?}, {:?})", a, b);
//     ()
// }
// fn assign_only_int(a: Column, b: i64) {
//     println!("assign_only({:?}, {})", a, b);
//     ()
// }
// fn assign_only(a: Cell, b: Cell) {
//     println!("assign_only({:?}, {:?})", a, b);
//     ()
// }
fn enable_selector(a: &mut Cell) {
    // println!("enable_selector({:?})", a);
    a.value = Some("1".to_string());
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        region
            .instructions
            .push(Instruction::EnableSelector(a.clone()));
    }
}
// fn assign_only_int(a: Cell, b: i64) {
//     println!("assign_only({:?}, {})", a, b);
//     ()
// }
// fn set_gate(advices: Array, selectors: Array, exp: String) {
//     println!("set_gate({:?}, {:?}, {})", advices, selectors, exp);
//     ()
// }
// fn set_gate(exp: &dyn Fn(Dynamic) -> Column) {
//     println!("set_gate()");
//     ()
// }
fn set_gate_col_ce(name: String, selector: Column, exp: CellExpression) {
    // println!("set_gate({:#?})", exp);
    unsafe {
        CONTEXT.gates.push((
            name,
            CellExpression::Product(
                Box::new(CellExpression::CellValue(selector.clone().get_field(0))),
                Box::new(exp),
            ),
        ));
    }
}
fn set_gate(name: String, selector: CellExpression, exp: CellExpression) {
    // println!("set_gate({:#?})", exp);
    unsafe {
        CONTEXT.gates.push((
            name,
            CellExpression::Product(Box::new(selector), Box::new(exp)),
        ));
    }
}

// operators
// Cell/CellExpression/Column/String/i64
macro_rules! engine_register_ops {
    ($eng: expr, $op: tt, $func: ident, $a:ty, $b:ty) => {
        $eng.register_fn(stringify!($op), $func::<$a, $b>);
    };
}

macro_rules! engine_register_ops_types {
    ($eng: expr, $op: tt, $func: ident) => {
        engine_register_ops!($eng, $op, $func, Cell, Cell);
        engine_register_ops!($eng, $op, $func, Cell, CellExpression);
        engine_register_ops!($eng, $op, $func, Cell, String);
        engine_register_ops!($eng, $op, $func, Cell, Column);
        engine_register_ops!($eng, $op, $func, Cell, i64);

        engine_register_ops!($eng, $op, $func, CellExpression, Cell);
        engine_register_ops!($eng, $op, $func, CellExpression, CellExpression);
        engine_register_ops!($eng, $op, $func, CellExpression, String);
        engine_register_ops!($eng, $op, $func, CellExpression, Column);
        engine_register_ops!($eng, $op, $func, CellExpression, i64);

        engine_register_ops!($eng, $op, $func, String, Cell);
        engine_register_ops!($eng, $op, $func, String, CellExpression);
        engine_register_ops!($eng, $op, $func, String, String);
        engine_register_ops!($eng, $op, $func, String, Column);
        engine_register_ops!($eng, $op, $func, String, i64);

        engine_register_ops!($eng, $op, $func, Column, Cell);
        engine_register_ops!($eng, $op, $func, Column, CellExpression);
        engine_register_ops!($eng, $op, $func, Column, String);
        engine_register_ops!($eng, $op, $func, Column, Column);
        engine_register_ops!($eng, $op, $func, Column, i64);

        engine_register_ops!($eng, $op, $func, i64, Cell);
        engine_register_ops!($eng, $op, $func, i64, CellExpression);
        engine_register_ops!($eng, $op, $func, i64, String);
        engine_register_ops!($eng, $op, $func, i64, Column);
        engine_register_ops!($eng, $op, $func, i64, i64);
    };
}

fn register_operator(engine: &mut rhai::Engine) {
    engine_register_ops_types!(engine, +, operator_plus);
    engine_register_ops_types!(engine, -, operator_minus);
    engine_register_ops_types!(engine, *, operator_mul);
}

fn operator_plus<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    CellExpression::Sum(
        Box::new(a.to_cell_expression()),
        Box::new(b.to_cell_expression()),
    )
}

fn operator_minus<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    CellExpression::Sum(
        Box::new(a.to_cell_expression()),
        Box::new(CellExpression::Negated(Box::new(b.to_cell_expression()))),
    )
}

fn operator_mul<T1: ToCellExpression, T2: ToCellExpression>(a: T1, b: T2) -> CellExpression {
    match ((a.to_cell_expression()), (b.to_cell_expression())) {
        (CellExpression::Constant(a), b) => CellExpression::Scaled(Box::new(b), a),
        (b, CellExpression::Constant(a)) => CellExpression::Scaled(Box::new(b), a),
        (a, b) => CellExpression::Product(Box::new(a), Box::new(b)),
    }
}

fn set_parameter_i64(name: String, v: i64) {
    set_parameter(name, v.to_string())
}

fn set_parameter(name: String, v: String) {
    unsafe {
        CONTEXT.inputs.insert(name, v);
    }
}

fn inspect(obj: Cell) {
    println!("{:#?}", obj);
}
