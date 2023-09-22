// (advices, selector), ?fixed?
// gate add(a, b, c, s) {
//     return <<s * (a + b - c)>>; // a == a[0]
// }

let N = 2;
fn add(a, b, c, s) {
    set_gate(s * (a + b - c));
}

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

    // #region: next row
    define_region("next row");
    for i in 0..N {
        // c[i] <== a[i] + b[i];
        assign_constraint(c[i], a[i] + b[i]);
        if (i > 0) {
            // a[i] <== b[i - 1];
            // b[i] <== c[i - 1];
            assign_constraint(a[i], b[i - 1]);
            assign_constraint(b[i], c[i - 1]);
        }
        // s[i] <-- 1;
        enable_selector(s[i]);
    }

    // out <== c[N-1];
    assign_constraint(out, c[N-1]);
// }