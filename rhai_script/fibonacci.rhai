// gate add(a, b, c, s) {
fn add(a, b, c, s) {
    // s <| a + b - c;
    set_gate("add", s, a + b - c);
}

let N = 8;

// circuit main(N) {
    // public input in1;
    // public input in2;
    // public output out;
    let in1 = init_input("in1");
    let in2 = init_input("in2");
    let out = init_output("out");

    // private advice a;
    // private advice b;
    // private advice c;
    let a = init_advice_column("a");
    let b = init_advice_column("b");
    let c = init_advice_column("c");

    // public selector s;
    let s = init_selector_column("s");

    add(a, b, c, s);

    // #region: first row
    define_region("first row");
    // a[0] <== in1;
    // b[0] <== in2;
    assign_constraint(a[0], in1);
    assign_constraint(b[0], in2);
    assign_constraint(c[0], a[0] + b[0]);
    enable_selector(s[0]);

    // #region: next row
    // define_region("next row");
    for i in 1..N {
            // a[i] <== b[i - 1];
            // b[i] <== c[i - 1];
            assign_constraint(a[i], b[i - 1]);
            assign_constraint(b[i], c[i - 1]);
        // c[i] <== a[i] + b[i];
        assign_constraint(c[i], a[i] + b[i]);
        // s[i] <-- 1;
        enable_selector(s[i]);
    }

    // out <== c[N-1];
    assign_constraint(out, c[N-1]);
// }