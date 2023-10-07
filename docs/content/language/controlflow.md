## If Statement

`if` statements follow C syntax.

```rust
if foo(x) {
    print("It's true!");
} else if bar == baz {
    print("It's true again!");
} else if baz.is_foo() {
    print("Yet again true.");
} else if foo(bar - baz) {
    print("True again... this is getting boring.");
} else {
    print("It's finally false!");
}
```


## While Loop

`while` loops follow C syntax.

`continue` can be used to skip to the next iteration, by-passing all following statements; `break` can be used to break out of the loop unconditionally.

```rust
let x = 10;

while x > 0 {
    x -= 1;
    if x < 6 { continue; }  // skip to the next iteration
    print(x);
    if x == 5 { break; }    // break out of while loop
}
```

## For Loop

Iterating through a numeric range or an array, or any iterable type, is provided by the `for` … `in` loop.

There are two alternative syntaxes, one including a counter variable:

> `for` _variable_ `in` _expression_ `{` … `}`
>
> `for (` _variable_ `,` _counter_ `)` `in` _expression_ `{` … `}`

### Break or Continue

`continue` can be used to skip to the next iteration, by-passing all following statements; `break` can be used to break out of the loop unconditionally.

### For Expression

`for` statements can also be used as _expressions_.

The `break` statement takes an optional expression that provides the return value.

The default return value of a `for` expression is `()`.

```js
let a = [42, 123, 999, 0, true, "hello", "world!", 987.6543];

// 'for' can be used just like an expression
let index = for (item, count) in a {
    // if the 'for' loop breaks here, return a specific value
    switch item.type_of() {
        "i64" if item.is_even => break count,
        "f64" if item.to_int().is_even => break count,
    }

    // ... if the 'for' loop exits here, the return value is ()
};

if index == () {
    print("Magic number not found!");
} else {
    print(`Magic number found at index ${index}!`);
}
```