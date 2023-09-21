use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use circuit::MyCircuit;
use regex::Regex;
use rhai::{ASTNode, Array, Engine, EvalAltResult, Position};

// use super::circuit;
mod circuit;

// use rhai::{Dynamic, ImmutableString};

// // Normal function that returns a standard type
// // Remember to use 'ImmutableString' and not 'String'
// fn add_len(x: i64, s: ImmutableString) -> i64 {
//     x + s.len() as i64
// }
// // Alternatively, '&str' maps directly to 'ImmutableString'
// fn add_len_count(x: i64, s: &str, c: i64) -> i64 {
//     x + (s.len() as i64) * c
// }
// // Function that returns a 'Dynamic' value
// fn get_any_value() -> Dynamic {
//     42_i64.into() // standard types can use '.into()'
// }

static mut CONTEXT: SimplifiedConstraitSystem = SimplifiedConstraitSystem {
    columns: vec![],
    signals: vec![],
    regions: vec![],
};

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>>
//                          ^^^^^^^^^^^^^^^^^^
//                          Rhai API error type
{
    // Create an 'Engine'
    let engine = Engine::new();

    // Your first Rhai Script
    let script = "print(40 + 2);";

    // Run the script - prints "42"
    engine.run(script)?;

    let result = engine.eval::<i64>("40 + 2")?;
    //                      ^^^^^^^ required: cast the result to a type

    println!("Answer: {result}"); // prints 42

    let result = engine.eval_file::<i64>("rhai_script/hello_world.rhai".into())?;
    //                                   ^^^^^^^^^^^^^^^^^^^^^^^^^
    //                                   a 'PathBuf' is needed
    println!("Answer: {result}"); // prints 42

    // Running a script file also works in a similar manner
    engine.run_file("rhai_script/hello_world.rhai".into())?;

    let re = Regex::new(
        r"(?x)
    (?P<year>\d{4})  # the year
    -
    (?P<month>\d{2}) # the month
    -
    (?P<day>\d{2})   # the day
    ",
    )
    .unwrap();

    let caps = re.captures("2010-03-14").unwrap();
    assert_eq!("2010", &caps["year"]);
    assert_eq!("03", &caps["month"]);
    assert_eq!("14", &caps["day"]);

    let result = re.replace_all("2023-09-07,2023-09-08", "$year:$month:$day");
    println!("{}", result);

    let mut engine = Engine::new();

    // // Notice that all three functions are overloaded into the same name with
    // // different number of parameters and/or parameter types.
    // engine
    //     .register_fn("add", add_len)
    //     .register_fn("add", add_len_count)
    //     .register_fn("add", get_any_value)
    //     .register_fn("inc", |x: i64| {
    //         // closure is also OK!
    //         x + 1
    //     })
    //     .register_fn("log", |label: &str, x: i64| {
    //         println!("{label} = {x}");
    //     });

    // let result = engine.eval::<i64>(r#"add(40, "xxx")"#)?;

    // println!("Answer: {result}"); // prints 42

    // let result = engine.eval::<i64>(r#"add(40, "xxx", 2)"#)?;

    // println!("Answer: {result}"); // prints 42

    // let result = engine.eval::<i64>("add()")?;

    // println!("Answer: {result}"); // prints 42

    // let result = engine.eval::<i64>("inc(40)")?;

    // println!("Answer: {result}"); // prints 42

    // engine.run(r#"log("value", 42)"#)?; // prints "value = 42"

    // Compile to an AST and store it for later evaluations
    // let ast = engine.compile("40 + 2")?;
    // println!("{:#?}", ast);

    // let ast = engine.compile("a[0] + b")?;
    // println!("{:#?}", ast);

    // ast.walk(&mut transcode_node);

    // ============================ here ===============================
    // engine.register_type_with_name::<TestStruct>("TestStruct")
    // .register_fn("new_ts", TestStruct::new)

    engine
        .register_type_with_name::<Column>("Column")
        .register_fn("init_input", init_input)
        .register_fn("init_output", init_output)
        .register_fn("init_advice_column", init_advice_column)
        .register_fn("init_selector_column", init_selector_column)
        .register_fn("define_region", define_region)
        .register_fn("assign_constraint", assign_constraint)
        .register_fn("assign_constraint", assign_constraint_int)
        .register_fn("assign_constraint", assign_constraint_cell_ce)
        .register_fn("assign_only", assign_only)
        .register_fn("assign_only", assign_only_int)
        .register_fn("set_gate", set_gate)
        .register_fn("+", operator_add)
        .register_fn("+", operator_add_column)
        .register_fn("+", operator_add_cell_column)
        .register_fn("-", operator_minus_cell_column)
        .register_fn("*", operator_mul_column_cell)
        .register_indexer_get(Column::get_field)
        // .register_indexer_set(TestStruct::set_field)
        ;

    engine.run_file("rhai_script/fibonacci.rhai".into())?;

    unsafe {
        println!("{:#?}", CONTEXT);
    }

    return Ok(());

    let mut scripts = Vec::<String>::new();
    // scripts.push("let N = 10;".to_string());
    if let Ok(lines) = read_lines("rhai_script/fibonacci.plonk") {
        // Consumes the iterator, returns an (Optional) String
        for line_result in lines {
            if let Ok(line) = line_result {
                // println!("{}", line);
                scripts.push(format_code(line));
            }
        }
    }

    let script = scripts.join("\n");
    println!("{}", script);

    engine.run(script.as_str())?;

    run_prover(
        4,
        unsafe { CONTEXT.clone() },
        vec![
            halo2_proofs::pasta::Fp::from(1),
            halo2_proofs::pasta::Fp::from(1),
            halo2_proofs::pasta::Fp::from(54),
        ],
    );

    Ok(())
}

fn run_prover(k: u32, scs: SimplifiedConstraitSystem, public_input: Vec<halo2_proofs::pasta::Fp>) {
    // let k = 4;

    // let a = Fp::from(1); // F[0]
    // let b = Fp::from(1); // F[1]
    // let out = Fp::from(1393); // F[9]

    let circuit = MyCircuit {
        scs,
        _marker: std::marker::PhantomData,
    };

    // let mut public_input = vec![a, b, out];

    let prover =
        halo2_proofs::dev::MockProver::run(k, &circuit, vec![public_input.clone()]).unwrap();

    let d = format!("{:#?}", prover);
    let mut file = std::fs::File::create(
        "/Users/oker/2-Project/02-zkkyc/halo2visualizer/packages/cli/src/input.rust",
    )
    .unwrap();
    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

    prover.assert_satisfied();
}

fn format_code(line: String) -> String {
    let reGate = Regex::new(
        r"(?x)
        gate\s(?<name>[\w\d]+)
        \((?<parameters>
        (?:\[[\w\d,\s]*\](?:,\s*)?){2}
        )\)",
    )
    .unwrap();
    // gate add([a, b, c], [s]) {
    // fn add(a, b, c, s) {

    let result = reGate.captures(&line);
    if let Some(v) = result {
        return format!(
            "fn {}({}) {{",
            &v["name"],
            &v["parameters"]
                .replace("[", "")
                .replace("]", "")
                .split(",")
                .map(|x| x.trim())
                .collect::<Vec<&str>>()
                .join(", ")
        );
    }

    let reGateReturn = Regex::new(
        r"(?x)
        return\s+<<(?<exp>.*)>>;",
    )
    .unwrap();
    // return <<s * (a + b - c)>>; // a == a[0]
    // set_gate(s * (a + b - c));

    let result = reGateReturn.captures(&line);
    if let Some(v) = result {
        return format!("set_gate({});", &v["exp"],);
    }

    line
    // return line;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum ColumnType {
    Selector,
    Advice,
    Fixed,
    Instance,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum SpecialType {
    Input,
    Output,
    Field,
    None,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Column {
    name: String,
    ctype: ColumnType,
    stype: SpecialType,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
struct Cell {
    column: Column,
    name: String,
    index: i64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum CellExpression {
    Constant(i64),
    CellValue(Cell),
    Negated(Box<CellExpression>),
    Product(Box<CellExpression>, Box<CellExpression>),
    Sum(Box<CellExpression>, Box<CellExpression>),
    Scaled(Box<CellExpression>, i64),
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

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct SimplifiedConstraitSystem {
    signals: Vec<Cell>,
    columns: Vec<Column>,
    regions: Vec<Region>,
}

#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
struct Region {
    name: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Instruction {
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

fn init_input(v: &str) -> Cell {
    println!("init_input({})", v);
    let cell = Cell {
        name: v.to_string(),
        index: -1,
        column: Column {
            name: v.to_string(),
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
    println!("init_output({})", v);
    let cell = Cell {
        name: v.to_string(),
        index: -1,
        column: Column {
            name: v.to_string(),
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
    println!("init_advice_column({})", v);
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
    println!("init_selector_column({})", v);
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
    println!("define_region({})", v);
    ()
}
// fn assign_constraint(a: Column, b: Column) {
//     println!("assign_constraint({:?}, {:?})", a, b);
//     ()
// }
fn assign_constraint(a: Cell, b: Cell) {
    println!("assign_constraint({:?}, {:?})", a, b);
    ()
}
fn assign_constraint_cell_ce(a: Cell, b: CellExpression) {
    println!("assign_constraint({:?}, {:?})", a, b);
    ()
}
// fn assign_constraint_int(a: Column, b: i64) {
//     println!("assign_constraint({:?}, {})", a, b);
//     ()
// }
fn assign_constraint_int(a: Cell, b: i64) {
    println!("assign_constraint({:?}, {})", a, b);
    ()
}
// fn assign_only(a: Column, b: Column) {
//     println!("assign_only({:?}, {:?})", a, b);
//     ()
// }
// fn assign_only_int(a: Column, b: i64) {
//     println!("assign_only({:?}, {})", a, b);
//     ()
// }
fn assign_only(a: Cell, b: Cell) {
    println!("assign_only({:?}, {:?})", a, b);
    ()
}
fn assign_only_int(a: Cell, b: i64) {
    println!("assign_only({:?}, {})", a, b);
    ()
}
// fn set_gate(advices: Array, selectors: Array, exp: String) {
//     println!("set_gate({:?}, {:?}, {})", advices, selectors, exp);
//     ()
// }
// fn set_gate(exp: &dyn Fn(Dynamic) -> Column) {
//     println!("set_gate()");
//     ()
// }
fn set_gate(exp: CellExpression) {
    println!("set_gate({:#?})", exp);
    ()
}
fn operator_add(a: Cell, b: Cell) -> CellExpression {
    println!("operator: {:?} + {:?}", a, b);
    // let n = format!("{} + {}", a.name, b.name);
    // Column {
    //     name: n,
    //     ctype: ColumnType::Selector,
    //     stype: SpecialType::None,
    // }
    CellExpression::Sum(
        Box::new(CellExpression::CellValue(a)),
        Box::new(CellExpression::CellValue(b)),
    )
}
fn operator_add_column(a: Column, b: Column) -> CellExpression {
    println!("operator: {:?} + {:?}", a, b);
    // let n = format!("{} + {}", a.name, b.name);
    // Column {
    //     name: n,
    //     ctype: ColumnType::Selector,
    //     stype: SpecialType::None,
    // }
    CellExpression::Sum(
        Box::new(CellExpression::CellValue(a.clone().get_field(0))),
        Box::new(CellExpression::CellValue(b.clone().get_field(0))),
    )
}
fn operator_add_cell_column(a: CellExpression, b: Column) -> CellExpression {
    println!("operator: {:?} + {:?}", a, b);
    // let n = format!("{} + {}", a.name, b.name);
    // Column {
    //     name: n,
    //     ctype: ColumnType::Selector,
    //     stype: SpecialType::None,
    // }
    CellExpression::Sum(
        Box::new(a),
        Box::new(CellExpression::CellValue(b.clone().get_field(0))),
    )
}
fn operator_minus_cell_column(a: CellExpression, b: Column) -> CellExpression {
    println!("operator: {:?} - {:?}", a, b);
    CellExpression::Sum(
        Box::new(a),
        Box::new(CellExpression::Negated(Box::new(
            CellExpression::CellValue(b.clone().get_field(0)),
        ))),
    )
}
fn operator_mul_column_cell(a: Column, b: CellExpression) -> CellExpression {
    println!("operator: {:?} * {:?}", a, b);
    CellExpression::Product(
        Box::new(CellExpression::CellValue(a.clone().get_field(0))),
        Box::new(b),
    )
}

/// Transcode the Rhai AST Node to uLisp
#[allow(dead_code)]
fn transcode_node(nodes: &[ASTNode]) -> bool {
    //  We take the root node, ignore the subnodes
    let node = &nodes[0];

    //  Get the source code position
    let pos = match node {
        ASTNode::Stmt(stmt) => stmt.position(),
        ASTNode::Expr(expr) => expr.position(),
        _ => Position::NONE,
    };

    //  Skip this node if we've already handled it
    unsafe {
        static mut LAST_POSITION: Position = Position::NONE;
        if LAST_POSITION == pos {
            return true;
        }
        LAST_POSITION = pos;
        println!("Node: {:#?}", node);
    }

    //  Transcode the Node: Statement or Expression
    let output = match node {
        ASTNode::Stmt(stmt) => println!("transcode_stmt(stmt)"),
        ASTNode::Expr(expr) => println!("transcode_expr(expr)"),
        _ => println!("empty"),
    };

    //  Add the transcoded uLisp S-Expression to the current scope
    // scope::add_to_scope(&output);
    println!("{:#?}", &output);

    //  Return true to walk the next node in the tree
    true
}
