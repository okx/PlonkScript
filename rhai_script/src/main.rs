use regex::Regex;
use rhai::{ASTNode, Array, Engine, EvalAltResult, Position};

use rhai::{Dynamic, ImmutableString};

// Normal function that returns a standard type
// Remember to use 'ImmutableString' and not 'String'
fn add_len(x: i64, s: ImmutableString) -> i64 {
    x + s.len() as i64
}
// Alternatively, '&str' maps directly to 'ImmutableString'
fn add_len_count(x: i64, s: &str, c: i64) -> i64 {
    x + (s.len() as i64) * c
}
// Function that returns a 'Dynamic' value
fn get_any_value() -> Dynamic {
    42_i64.into() // standard types can use '.into()'
}

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

    // Notice that all three functions are overloaded into the same name with
    // different number of parameters and/or parameter types.
    engine
        .register_fn("add", add_len)
        .register_fn("add", add_len_count)
        .register_fn("add", get_any_value)
        .register_fn("inc", |x: i64| {
            // closure is also OK!
            x + 1
        })
        .register_fn("log", |label: &str, x: i64| {
            println!("{label} = {x}");
        });

    let result = engine.eval::<i64>(r#"add(40, "xxx")"#)?;

    println!("Answer: {result}"); // prints 42

    let result = engine.eval::<i64>(r#"add(40, "xxx", 2)"#)?;

    println!("Answer: {result}"); // prints 42

    let result = engine.eval::<i64>("add()")?;

    println!("Answer: {result}"); // prints 42

    let result = engine.eval::<i64>("inc(40)")?;

    println!("Answer: {result}"); // prints 42

    engine.run(r#"log("value", 42)"#)?; // prints "value = 42"

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
        .register_fn("init_input", init_input)
        .register_fn("init_output", init_output)
        .register_fn("init_advice_column", init_advice_column)
        .register_fn("init_selector_column", init_selector_column)
        .register_fn("define_region", define_region)
        .register_fn("assign_constraint", assign_constraint)
        .register_fn("assign_only", assign_only)
        .register_fn("set_gate", set_gate);

    engine.run_file("rhai_script/fibonacci.plonk".into())?;

    // Done!
    Ok(())
}

fn init_input(v: &str) -> &str {
    println!("init_input({})", v);
    v
}
fn init_output(v: String) -> String {
    println!("init_output({})", v);
    v
}
fn init_advice_column(v: String) -> String {
    println!("init_advice_column({})", v);
    v
}
fn init_selector_column(v: String) -> String {
    println!("init_selector_column({})", v);
    v
}
fn define_region(v: String) {
    println!("define_region({})", v);
    ()
}
fn assign_constraint(a: String, b: String) {
    println!("assign_constraint({}, {})", a, b);
    ()
}
fn assign_only(a: String, b: String) {
    println!("assign_only({}, {})", a, b);
    ()
}
fn set_gate(advices: Array, selectors: Array, a: String, b: String, c: String) {
    println!("set_gate({:?}, {:?}, {}, {}, {})", advices, selectors, a, b, c);
    ()
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
