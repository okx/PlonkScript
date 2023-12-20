You can try the [analyzer](/play/index.html#/analyzer/halo2){target="_blank"} now.

To start analysis, inject the code like this to get the trace for analysis:

```rust
let d = format!("{:#?}", prover);
let mut file = std::fs::File::create("data.rust").unwrap();
std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();
```

or

```toml
halo2_summarizer = "0.1.1"
```

```rust
let d = format!("{:#?}", prover);
let d = halo2_summarizer::trim(&d, Some(0..1024));
let mut file = std::fs::File::create("data.rust").unwrap();
std::io::Write::write_all(&mut file, d.as_bytes()).unwrap();
```
