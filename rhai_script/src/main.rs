use std::fs;

use rhai::EvalAltResult;
use rhai_script::try_run;

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>> {
    let code = fs::read_to_string("rhai_script/fibonacci.plonk").unwrap();
    let k = 4;
    let inputs = vec![
        ("in1".to_string(), "1".to_string()),
        ("in2".to_string(), "1".to_string()),
    ];
    let d = try_run(code, k, inputs)?;

    let mut file = std::fs::File::create(
        "/Users/oker/2-Project/02-zkkyc/halo2visualizer/packages/cli/src/input.rust",
    )
    .unwrap();
    // let mut file = std::fs::File::create("prover_out.rust").unwrap();
    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

    Ok(())
}
