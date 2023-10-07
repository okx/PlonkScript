
```plonkscript
gate add(a, b, c, s) {
    s <| a + b - c;
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

a[0] <== in1;
b[0] <== in2;

for i in 0..N {
    if (i > 0) {
        a[i] <== b[i - 1];
        b[i] <== c[i - 1];
    }
    c[i] <== a[i] + b[i];
    s[i] <-- enable;
}

out <== c[N-1];
```