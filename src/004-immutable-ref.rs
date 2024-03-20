// immutable reference
fn print_length(s: &String) {
    println!("Length: {}", s.len());
}

fn main() {
    let x = String::from("Hello");
    let y = &x;
    let z = &x;

    println!("x: {}", x);
    println!("z: {}", z);
    // rust is smart to figure out that we want to dereference y
    // so we don't need to use *y or &y
    println!("y: {}", y);
    println!("*y: {}", *y);
    println!("&y: {}", &y);

    // we need to use &x to pass a reference
    // the immutable reference
    print_length(&x);
    // rust is smart to figure out that we want to dereference y
    // so we don't need to use *y or &y
    print_length(&x);
    print_length(y);
    print_length(&y);

}
