fn main() {
    let mut a = 5;

    // we can't have mutable ref and immutable ref at the same time
    // for the same value
    // to fix the issue we can use a new scope
    {
        let b = &mut a;
        *b += 1;
        println!("b: {}", b);
    }

    println!("a: {}", a);
}
