use crate::{system::*, CONTEXT};

pub const DEFAULT_INSTANCE_COLUMN_NAME: &str = "defins";

pub trait EngineExt {
    fn register_plonk_script(&mut self);
}

impl EngineExt for rhai::Engine {
    #[warn(unused_must_use)]
    fn register_plonk_script(&mut self) {
        let _ = &mut self
        .register_type_with_name::<Column>("Column")
        .register_fn("init_input", init_input)
        .register_fn("init_output", init_output)
        .register_fn("init_advice_column", init_advice_column)
        .register_fn("init_selector_column", init_selector_column)
        .register_fn("define_region", define_region)
        .register_fn("assign_constraint", assign_constraint)
        // .register_fn("assign_constraint", assign_constraint_int)
        .register_fn("assign_constraint", assign_constraint_cell_ce)
        // .register_fn("assign_only", assign_only)
        // .register_fn("assign_only", assign_only_int)
        .register_fn("set_gate", set_gate)
        .register_fn("set_gate", set_gate_col_ce)
        .register_fn("enable_selector", enable_selector)
        .register_fn("+", operator_add)
        .register_fn("+", operator_add_column)
        .register_fn("+", operator_add_cell_column)
        .register_fn("-", operator_minus_cell_column)
        .register_fn("*", operator_mul_column_cell)
        .register_indexer_get(Column::get_field)
        // .register_indexer_set(TestStruct::set_field)
        ;
    }
}

impl Column {
    fn get_field(&mut self, index: i64) -> Cell {
        let name = format!("{}[{}]", self.name, index);
        Cell {
            name,
            index,
            column: self.clone(),
        }
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
fn define_region(v: String) {
    // println!("define_region({})", v);
    unsafe {
        CONTEXT.regions.push(Region {
            name: v,
            instructions: vec![],
        });
    }
    ()
}
// fn assign_constraint(a: Column, b: Column) {
//     println!("assign_constraint({:?}, {:?})", a, b);
//     ()
// }

// a <== b
fn assign_constraint(a: Cell, b: Cell) {
    // println!("assign_constraint({:?}, {:?})", a, b);
    push_instruction_to_last_region(match (a.column.ctype, b.column.ctype) {
        (ColumnType::Advice, ColumnType::Instance) => {
            vec![Instruction::AssignAdviceFromInstance(a, b)]
        }
        (ColumnType::Instance, ColumnType::Advice) => {
            vec![Instruction::AssignAdviceFromInstance(b, a)]
        }
        (_, _) => vec![
            Instruction::AssignAdvice(a.clone(), CellExpression::CellValue(b.clone())),
            Instruction::ConstrainEqual(a, b),
        ],
    });
}

// a <== b (b is expresion, e.g. b1 + b2)
fn assign_constraint_cell_ce(a: Cell, b: CellExpression) {
    // println!("assign_constraint({:?}, {:?})", a, b);
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a, b)]);
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
fn enable_selector(a: Cell) {
    // println!("enable_selector({:?})", a);
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        region.instructions.push(Instruction::EnableSelector(a));
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
fn operator_add(a: Cell, b: Cell) -> CellExpression {
    // println!("operator: {:?} + {:?}", a, b);
    CellExpression::Sum(
        Box::new(CellExpression::CellValue(a)),
        Box::new(CellExpression::CellValue(b)),
    )
}
fn operator_add_column(a: Column, b: Column) -> CellExpression {
    // println!("operator: {:?} + {:?}", a, b);
    CellExpression::Sum(
        Box::new(CellExpression::CellValue(a.clone().get_field(0))),
        Box::new(CellExpression::CellValue(b.clone().get_field(0))),
    )
}
fn operator_add_cell_column(a: CellExpression, b: Column) -> CellExpression {
    // println!("operator: {:?} + {:?}", a, b);
    CellExpression::Sum(
        Box::new(a),
        Box::new(CellExpression::CellValue(b.clone().get_field(0))),
    )
}
fn operator_minus_cell_column(a: CellExpression, b: Column) -> CellExpression {
    // println!("operator: {:?} - {:?}", a, b);
    CellExpression::Sum(
        Box::new(a),
        Box::new(CellExpression::Negated(Box::new(
            CellExpression::CellValue(b.clone().get_field(0)),
        ))),
    )
}
fn operator_mul_column_cell(a: Column, b: CellExpression) -> CellExpression {
    // println!("operator: {:?} * {:?}", a, b);
    CellExpression::Product(
        Box::new(CellExpression::CellValue(a.clone().get_field(0))),
        Box::new(b),
    )
}
