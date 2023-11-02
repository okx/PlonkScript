use std::fs;

use rhai::EvalAltResult;
use transpiler::try_run;

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>> {
    let code = fs::read_to_string("fibonacci.plonk").unwrap();
    let k = 4;
    let inputs = vec![
        ("in1".to_string(), "1".to_string()),
        ("in2".to_string(), "1".to_string()),
    ];
    let output = try_run(code, k, inputs)?;
    // let d = halo2_summarizer::trim_start_end(&output, 2, 6);
    let d = halo2_summarizer::trim(&output);

    let mut file = std::fs::File::create("data.rust").unwrap();
    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

    Ok(())
}
