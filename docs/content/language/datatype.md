
```plonk
public input in1;
public output out1;
private advice adv1;
private input in2;
private hint h1;
public fixed f1;
public selector s1;
```

```rust
let in1 : Cell = init_input("in1");
let out : Cell = init_output("out1");
let a : Column = init_advice_column("adv1");
let in2 : Cell = init_private_input("in2");
let h1 : Cell = init_hint("h1");
let f1 : Column = init_fixed_column("f1");
let s1 : Column = init_selector_column("s1");
```