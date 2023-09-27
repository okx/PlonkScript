#![allow(unused_variables)]
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
        alert(&format!("Hello alert, {}!", name));
    }
}
