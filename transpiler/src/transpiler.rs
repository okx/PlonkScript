use regex::{Captures, Regex};

pub fn transpile(code: String) -> String {
    let code = format_parameters(code);
    let code = format_hex(code);
    let code = format_gate_fn(code);
    let code = format_region_fn(code);
    let (code, outputs) = format_declaration(code);
    let code = format_assignment(code);
    let code = append_output_assignment(code, outputs);
    code
}

fn append_output_assignment(code: String, outputs: Vec<String>) -> String {
    format!(
        "{}\n{}",
        code,
        outputs
            .iter()
            .map(|x| format!("set_output(\"{}\", {});", x, x).to_string())
            .collect::<Vec<String>>()
            .join("\n")
    )
}

fn format_gate_fn(code: String) -> String {
    let re_gate_fn = Regex::new(
        r"(?s)(?x)
        gate\s+(?P<name>[\w\d]+)
        \((?P<parameters>
        [\w\d,\s]*)\)
        \s+\{
        (?P<code>.*?)
        \}",
    )
    .unwrap();
    re_gate_fn
        .replace_all(&code, |x: &Captures| {
            format!(
                "fn {}({}) {{{}}}",
                &x["name"],
                &x["parameters"],
                format_gate_exp(x["code"].to_string(), x["name"].to_string())
            )
            .to_string()
        })
        .to_string()
}

fn format_gate_exp(code: String, name: String) -> String {
    // s <| a + b - c;
    // set_gate("add", s, a + b - c);
    let re_gate_exp = Regex::new(
        r"(?s)(?x)
        (?P<selector>[\w\d]+)
        \s*
        <\|
        \s*
        (?P<exp>.+?)
        \s*
        ;",
    )
    .unwrap();
    re_gate_exp
        .replace_all(&code, |x: &Captures| {
            format!("set_gate(\"{}\", {}, {});", name, &x["selector"], &x["exp"],).to_string()
        })
        .to_string()
}

fn format_declaration(code: String) -> (String, Vec<String>) {
    let mut outputs = Vec::new();
    // public input in1;
    let re_exp = Regex::new(
        r"(?x)
        (?P<modifier>(?:public|private))
        \s*
        (?P<type>(?:input|output|advice|fixed|selector|instance))
        \s*
        (?P<name>[\w\d]+?)
        \s*
        ;",
    )
    .unwrap();
    let code = re_exp
        .replace_all(&code, |x: &Captures| match (&x["modifier"], &x["type"]) {
            ("public", "input") => format!("let {} = init_input(\"{}\");", &x["name"], &x["name"]),
            ("public", "output") => {
                outputs.push(x["name"].to_string());
                format!("let {} = init_output(\"{}\");", &x["name"], &x["name"])
            }
            ("private", "advice") => format!(
                "let {} = init_advice_column(\"{}\");",
                &x["name"], &x["name"]
            ),
            ("public", "selector") => format!(
                "let {} = init_selector_column(\"{}\");",
                &x["name"], &x["name"]
            ),
            ("public", "fixed") => format!(
                "let {} = init_fixed_column(\"{}\");",
                &x["name"], &x["name"]
            ),
            (modifier, t) => {
                println!("{} {} is not supported", modifier, t);
                todo!()
            }
        })
        .to_string();
    (code, outputs)
}

fn format_assignment(code: String) -> String {
    // a[0] <== in1;
    let re_exp = Regex::new(
        r"(?x)
        (?P<indent>\x20*)
        (?P<to>[\w\d\[\]+\-*\x20]+?)
        \s*
        (?P<assignment><==|<--)
        \s*
        (?P<from>[\w\d\[\]+\-*\x20]+?)
        \s*
        ;",
    )
    .unwrap();
    re_exp
        .replace_all(&code, |x: &Captures| match (&x["assignment"], &x["from"]) {
            ("<==", _) => format!(
                "{}{} = assign_constraint({}, {});",
                &x["indent"], &x["to"], &x["to"], &x["from"]
            ),
            ("<--", "enable") => format!("{}enable_selector({});", &x["indent"], &x["to"]),
            ("<--", _) => format!(
                "{}{} = assign_common({}, {});",
                &x["indent"], &x["to"], &x["to"], &x["from"]
            ),
            (ass, val) => {
                println!("{} {} {} is not supported", &x["to"], ass, val);
                todo!()
            }
        })
        .to_string()
}

fn format_region_fn(code: String) -> String {
    let re_region_fn = Regex::new(
        r"(?s)(?x)
        region\s+(?P<name>[\w\d]+)
        \((?P<parameters>
        [\w\d,\s]*)\)
        \s+\{
        (?P<code>.*?)
        \}",
    )
    .unwrap();
    re_region_fn
        .replace_all(&code, |x: &Captures| {
            format!(
                "fn {}({}) {{define_region(\"{}\");{}}}",
                &x["name"],
                &x["parameters"],
                &x["name"],
                format_gate_exp(x["code"].to_string(), x["name"].to_string())
            )
            .to_string()
        })
        .to_string()
}

fn format_parameters(code: String) -> String {
    let re_parameters = Regex::new(
        r"(?m)(?x)
        ^\#
        \s*
        (?P<name>[\d\w]+)
        :
        \s*
        (?P<val>.*)
        $",
    )
    .unwrap();
    re_parameters
        .replace_all(&code, |x: &Captures| {
            format!("set_parameter(\"{}\", {});", &x["name"], &x["val"]).to_string()
        })
        .to_string()
}


fn format_hex(code: String) -> String {
    let re_hex = Regex::new(
        r"(?x)
        (?P<hex>0x[0-9a-f]{64})",
    )
    .unwrap();
    re_hex
        .replace_all(&code, |x: &Captures| {
            format!("\"{}\"", &x["hex"]).to_string()
        })
        .to_string()
}
