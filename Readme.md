# math-symbols

Named symbols for use in compute algebra systems. Symbol names are
stored centrally such that copies are cheap and need little
memory.

## Example

```rust
use math_symbols::*;

// Define a number of symbols with variable name equal to symbol name
symbols!(x, y, z);
assert_eq!(x.name(), "x");
assert_eq!(y.name(), "y");
assert_eq!(z.name(), "z");

// Symbols are identified by their names
let xx = Symbol::new("x");
assert_eq!(x, xx);

// Symbols are ordered by their creation time
assert!(x < y);
```
## Similar crates

- [symbol](https://crates.io/crates/symbol)


License: GPL-3.0-or-later
