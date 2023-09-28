#![allow(unused_variables)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TryRunRequest {
    pub k: u32,
    pub code: String,
    pub inputs: HashMap<String, String>,
}

#[allow(dead_code)]
fn main() {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        fn alert(s: &str);

        // Use `js_namespace` here to bind `console.log(..)` instead of just
        // `log(..)`
        #[wasm_bindgen(js_namespace = console)]
        fn log(s: &str);
    }

    #[wasm_bindgen]
    pub fn greet(name: &str) {
        log(&format!("Hello log, {}!", name));
        // alert(&format!("Hello alert, {}!", name));
    }

    #[wasm_bindgen]
    pub fn try_run(
        request: JsValue,
    ) -> Result<JsValue, JsValue> {
        // log(&format!("try_run!"));

        let req: TryRunRequest = serde_wasm_bindgen::from_value(request)?;
        // log(&format!("deserialized!"));
        // log(&format!("{:?}", req.code));
        // log(&format!("{:?}", req.k));
        let inputs = Vec::from_iter(
            req.inputs
                .iter()
                .map(|x| (x.0.to_string(), x.1.to_string())),
        );
        // log(&format!("{:?}", inputs));
        match rhai_script::try_run(req.code, req.k, inputs) {
            Ok(d) => Ok(JsValue::from_str(d.as_str())),
            Err(d) => Err(JsValue::from_str(d.to_string().as_str())),
        }
    }
}
