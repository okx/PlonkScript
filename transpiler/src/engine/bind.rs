use rhai::EvalAltResult;

use crate::system::cell_expression::ToValueString;
use crate::system::*;
use crate::CONTEXT;

pub fn register_bind(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_fn("assign_constraint", assign_constraint)
        .register_fn("assign_constraint", assign_constraint_cell_ce)
        .register_fn("assign_constraint", assign_constraint_string)
        .register_fn("assign_common", assign_common_string)
        .register_fn("assign_common", assign_common_ce)
        .register_fn("enable_selector", enable_selector)
        ;
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
    a.value = b.to_value_string();
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), b)]);
    a.clone()
}

fn assign_constraint_string(a: &mut Cell, b: String) -> Cell {
    // println!("assign_constraint({:?}, {:?})", a, b);
    let cb = CellExpression::Constant(b);
    a.value = cb.to_value_string();
    push_instruction_to_last_region(vec![Instruction::AssignAdvice(a.clone(), cb)]);
    a.clone()
}

fn assign_common_string(a: &mut Cell, b: String) -> Cell {
    match a.column.ctype {
        ColumnType::Fixed => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), cb)]);
            a.clone()
        }
        ColumnType::Instance => {
            let cb = CellExpression::Constant(b);
            a.value = cb.to_value_string();
            a.clone()
            //warning
        }
        o => todo!("{:?}", o),
    }
}

fn assign_common_ce(a: &mut Cell, b: CellExpression) -> Result<Cell, Box<EvalAltResult>> {
    match a.column.ctype {
        ColumnType::Fixed => {
            a.value = b.to_value_string();
            push_instruction_to_last_region(vec![Instruction::AssignFixed(a.clone(), b)]);
            Ok(a.clone())
        }
        ColumnType::Advice => {
            a.value = b.to_value_string();
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

fn enable_selector(a: &mut Cell) {
    // println!("enable_selector({:?})", a);
    a.value = Some("1".to_string());
    if let Some(region) = unsafe { CONTEXT.regions.last_mut() } {
        region
            .instructions
            .push(Instruction::EnableSelector(a.clone()));
    }
}
