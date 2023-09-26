use std::collections::HashMap;
use std::fs::{File, self};
use std::io::{self, BufRead};
use std::path::Path;

use circuit::MyCircuit;
use halo2_proofs::arithmetic::Field;
use regex::Regex;
use rhai::{Engine, EvalAltResult};
use system::SimplifiedConstraitSystem;
use transpiler::transpile;

use crate::engine::EngineExt;
use once_cell::sync::Lazy;

mod circuit;
mod engine;
mod system;
mod transpiler;

static mut CONTEXT: SimplifiedConstraitSystem = SimplifiedConstraitSystem {
    // ..Default::default()
    signals: Vec::new(),
    columns: Vec::new(),
    regions: Vec::new(),
    gates: Vec::new(),
    inputs: Lazy::new(|| HashMap::new()),
    instance_count: 0,
};

#[allow(unreachable_code)]
pub fn main() -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    engine.register_plonk_script();

    unsafe {
        CONTEXT.inputs.insert("in1".to_string(), "1".to_string());
        CONTEXT.inputs.insert("in2".to_string(), "1".to_string());
    }

    let num = 1;

    if num == 0 {
        engine.run_file("rhai_script/fibonacci.rhai".into())?;
    } else {
        let code = fs::read_to_string("rhai_script/fibonacci.plonk").unwrap();
        let script = transpile(code);

        // println!("{}", script);
        engine.run(script.as_str())?;
    }

    let d = unsafe { format!("{:#?}", CONTEXT) };
    let mut file = std::fs::File::create("out.rust").unwrap();
    std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

    let public_input = unsafe { CONTEXT.signals.clone() }
        .into_iter()
        .map(|x| halo2_proofs::pasta::Fp::from(x.value.unwrap().parse::<u64>().unwrap()))
        .collect();

    run_prover(4, public_input);

    Ok(())
}

fn run_prover(
    k: u32,
    //  scs: SimplifiedConstraitSystem,
    public_input: Vec<halo2_proofs::pasta::Fp>,
) {
    // let k = 4;

    // let a = Fp::from(1); // F[0]
    // let b = Fp::from(1); // F[1]
    // let out = Fp::from(1393); // F[9]

    let circuit = MyCircuit {
        // scs,
        _marker: std::marker::PhantomData,
    };

    // let mut public_input = vec![a, b, out];

    let presult = halo2_proofs::dev::MockProver::run(k, &circuit, vec![public_input.clone()]);

    if let Ok(prover) = presult {
        let d = format!("{:#?}", prover);
        let mut file = std::fs::File::create(
            "/Users/oker/2-Project/02-zkkyc/halo2visualizer/packages/cli/src/input.rust",
        )
        .unwrap();
        // let mut file = std::fs::File::create("prover_out.rust").unwrap();
        std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();

        prover.assert_satisfied();
        println!("prove completed");
    } else {
        println!("{:?}", presult.err());
    }
}
