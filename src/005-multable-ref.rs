
fn update_string(s: &mut String) {
    s.push_str(" another extra word");
}

fn main() {
    
    let mut x = "hello".to_string();
    // mutable reference
    let y = &mut x;

    y.push_str(" world");
    update_string(y);

    println!("x: {}", x);


    let mut a = 5;
    let b = &mut a;
    *b += 1;
    // println!("a: {}", a);
    println!("b: {}", b);


}
