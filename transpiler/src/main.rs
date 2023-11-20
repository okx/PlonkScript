use std::fs;

use rhai::EvalAltResult;
use transpiler::try_run;

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>> {
    let code = fs::read_to_string("fibonacci.plonk").unwrap();
    let output = try_run(code)?;
    // let d = halo2_summarizer::trim(&output, Some(0..16));
    // let d = halo2_summarizer::trim(&output);
    let d = output;

    let mut file = std::fs::File::create("visualization.rust").unwrap();
    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

    Ok(())
}
