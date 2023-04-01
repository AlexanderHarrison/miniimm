# MiniImm [![Docs](https://docs.rs/miniimm/badge.svg)](https://docs.rs/icu) [![Latest Version]](https://crates.io/crates/serde)

MiniImm is a small rust library providing small string optimization for immutable strings.
If you have many small strings, it can be beneficial to store them inline on the stack
rather than have many small allocations.
This is doubly true for immutable strings, where the inline or heap allocated nature of the string will never need to change.

There are many libraries providing small string optimization for mutable strings,
such as [smartstring](https://crates.io/crates/smartstring), or [tinystr](https://crates.io/crates/tinystr),
but there aren't any specifically for immutable strings.

### Why?

Mutable strings require 24 bytes on the stack, but MiniImm only requires 16 bytes.
Mutable strings contain three fields on the stack when allocated, a pointer, a length, and a capacity.
Immutable strings do not need the capacity field and only require a pointer and a length.
However, this means that this library can only store strings of up to 14 bytes inline as opposed to over 20
bytes for mutable strings.

### Unsafe?
Yes.

### Example:

```rust
use miniimm::MiniImmStr;

fn main() {
    let heap = MiniImmStr::from_str("akjdhkasjdaskdhakshdjk");
    let stack = MiniImmStr::from_str("hello!");

    println!("on heap: {}", heap);
    println!("inline: {}", stack);

    assert!(!heap.is_inline());
    assert!(stack.is_inline());

    // MiniImmStr Derefs to &str, meaning you can call any str method.
    assert!(heap.is_ascii());
    assert!(stack.is_ascii());

    assert!(std::mem::size_of::<MiniImmStr>() == 16);
}
```

### Caveats
- Various methods and traits may not be implemented (make an issue!).
- Only 14 bytes can be stored inline, as opposed to 20+ for other libraries.
- Due to implementation details, the length of a string must fit in a `u32`.
