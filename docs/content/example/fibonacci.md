
Very simple fibonacci sequence proving.

```plonkscript
# k: 4
# in1: 1
# in2: 1

gate add(a, b, c, s) {
    s <| a + b - c;
}

region first_row(a, b, c, s, in1, in2) {
    a[0] <== in1;
    b[0] <== in2;
    c[0] <== a[0] + b[0];
    s[0] <-- enable;

    [b[0], c[0]]
}

region next_row(a, b, c, s, last_b, last_c) {
    a[0] <== last_b;
    b[0] <== last_c;
    c[0] <== a[0] + b[0];
    s[0] <-- enable;

    c[0]
}

let N = 10;

public input in1;
public input in2;
public output out;

private advice a;
private advice b;
private advice c;

public selector s;

add(a, b, c, s);
let fr = first_row(a, b, c, s, in1, in2);
let last_b = fr[0];
let last_c = fr[1];
for i in 1..N {
    let c = next_row(a, b, c, s, last_b, last_c);
    last_b = last_c;
    last_c = c;
}

out <== last_c;
```
