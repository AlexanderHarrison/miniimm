fn main() {
    let heap = miniimm::MiniImmStr::from_str("akjdhkasjdaskdhakshdjk");
    let stack = miniimm::MiniImmStr::from_str("hello!");

    let heap2 = miniimm::MiniImmStr::from_string(String::from("akjdhkasjdaskdhakshdjk"));
    let stack2 = miniimm::MiniImmStr::from_string(String::from("hello!"));

    println!("on heap: {}", heap);
    println!("inline: {}", stack);
    println!("on heap: {}", heap2);
    println!("inline: {}", stack2);

    assert!(!heap.is_inline());
    assert!(stack.is_inline());
    assert!(!heap2.is_inline());
    assert!(stack2.is_inline());

    std::mem::drop(heap);
    std::mem::drop(stack);
    std::mem::drop(heap2);
    std::mem::drop(stack2);
}
