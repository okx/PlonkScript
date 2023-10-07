
Plonk script utilize `<==` arrow symbol represent assignment and constraints.

```plonkscript
a[0] <== in1;
c[i] <== a[i] + b[i];
s[i] <-- enable;
out === root;
```

The equivalent rhai code: 

```rust
assign_constraint(a[0], in1);
assign_constraint(c[i], a[i] + b[i]);
enable_selector(s[i]);
constraint(out, root);
```