fn print_letter(c: &String) {
    println!("{}", c);
}

fn main() {
    // let a = 42;
    // let b = a;

    // println!("{}", a);
    // println!("{}", b);

    // let x = String::from("42");
    // let y = x;

    // println!("{}", x);
    // println!("{}", y);

    // let z = String::from("42");

    // print_letter(&z);
    // print_letter(&z);

    // implement copy trait
    // if no copy-trait,
    //then ownership is moved

    let x = 43;
    let mut y = x;

    y = 44;

    println!("{}", x);
    println!("{}", y);
}
