use std::io;

use rhai::EvalAltResult;

use crate::system::*;
use crate::CONTEXT;

pub fn register_gate(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_fn("set_gate", set_gate)
        .register_fn("set_gate", set_gate_col_ce);
}

fn set_gate_col_ce(name: String, selector: Column, exp: CellExpression) -> Result<(), Box<EvalAltResult>>{
    set_gate(
        name,
        CellExpression::CellValue(selector.clone().get_field(0)),
        exp,
    )
}
fn set_gate(
    name: String,
    selector: CellExpression,
    exp: CellExpression,
) -> Result<(), Box<EvalAltResult>> {
    // println!("set_gate({:#?})", exp);
    check_gate_ce(&exp)?;
    unsafe {
        CONTEXT.gates.push((
            name,
            CellExpression::Product(Box::new(selector), Box::new(exp)),
        ));
    }
    Ok(())
}

fn check_gate_ce(exp: &CellExpression) -> Result<(), Box<EvalAltResult>> {
    match exp {
        CellExpression::Constant(_) => Ok(()),
        CellExpression::CellValue(c) => match c.column.ctype {
            crate::system::ColumnType::Selector => create_error(
                io::ErrorKind::Unsupported,
                "Instance cannot be used in gate",
            ),
            crate::system::ColumnType::Advice => Ok(()),
            crate::system::ColumnType::Fixed => match c.index {
                0 => Ok(()),
                _ => create_error(
                    io::ErrorKind::Unsupported,
                    "Fixed column cannot have rotation in gate, refer to https://github.com/zcash/halo2/issues/585",
                ),
            },
                
            crate::system::ColumnType::Instance => create_error(
                io::ErrorKind::Unsupported,
                "Instance cannot be used in gate",
            ),
        },
        CellExpression::Negated(n) => check_gate_ce(&*n),
        CellExpression::Product(a, b) => check_gate_ce(&*a).and(check_gate_ce(&*b)),
        CellExpression::Sum(a, b) => check_gate_ce(&*a).and(check_gate_ce(&*b)),
        CellExpression::Scaled(a, _) => check_gate_ce(&*a),
    }
}

fn create_error(kind: io::ErrorKind, err: &str) -> Result<(), Box<EvalAltResult>> {
    Err(Box::new(EvalAltResult::ErrorSystem(
        "gate check error".to_string(),
        Box::new(io::Error::new(kind, err)),
    )))
}
