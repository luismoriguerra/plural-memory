// we can use &str 
// because we don't need to modify the string
fn print_length(s: &str) {
    println!("Length: {}", s.len());
}

fn main() {
    let x = String::from("Hello");
    let y = &x;
    // by .. we can take a slice of the string
    let z = &x[0..5];
    let w = &x[..];

    println!("x: {}", x);
    println!("z: {}", z);
    println!("y: {}", y);
    println!("*y: {}", *y);
    println!("&y: {}", &y);

    print_length(&x);
    print_length(&x);
    print_length(y);
    print_length(&y);
}
