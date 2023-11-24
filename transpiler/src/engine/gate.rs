use crate::system::*;
use crate::CONTEXT;

pub fn register_gate(engine: &mut rhai::Engine) {
    let _ = &mut engine
        .register_fn("set_gate", set_gate)
        .register_fn("set_gate", set_gate_col_ce);
}

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
