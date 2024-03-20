#[derive(Debug, Copy, Clone)]
struct Person {
    id: u32,
    age: u8,
}

fn main() {
    // copy trait work here 
    // because fields are primitive types (no vector, string)
    let p1 = Person { id: 1, age: 20 };

    let mut p2 = p1;
    p2.id = 2;
    p2.age = 21;

    println!("{:?}", p1);
    println!("{:?}", p2);
}
