# MiniImm

MiniImm is a small rust library providing small string optimization for immutable strings.
If you have many small strings, it can be beneficial to store them inline on the stack
rather than have many small allocations.
This is doubly true for immutable strings, where the inline or heap allocated nature of the string will never need to change.

There are many libraries providing small string optimization for mutable strings,
such as [smartstring](https://crates.io/crates/smartstring), or [tinystr](https://crates.io/crates/tinystr),
but there aren't any specifically for immutable strings.

## Why?

Why not just use a mutable string library instead?
Memory - mutable strings contain three fields on the stack when allocated, a pointer, a length, and a capacity
for a total stack size of 24 bytes per string.
Allocated immutable strings only require a pointer and a length,
meaning the stack size is only 16 bytes.
However, this means that this library can only store strings of up to 14 bytes inline as opposed to over 20
bytes for mutable strings.

## Unsafe?
Yes.

## Example:

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
