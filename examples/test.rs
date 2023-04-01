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
