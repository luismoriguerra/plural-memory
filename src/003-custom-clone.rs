#[derive(Debug)]
struct Person {
    id: u32,
    age: u8,
    // dynamic traits
    // vectors and String
    // those needs to be cloned (deep copy)
    name: String,
}

impl Clone for Person {
    fn clone(&self) -> Person {
        Person {
            id: self.id,
            age: self.age,
            name: self.name.clone(),
        }
    }
}

fn main() {
    let p1 = Person {
        id: 1,
        age: 20,
        name: String::from("John"),
    };

    let mut p2 = p1.clone();
    p2.id = 2;
    p2.age = 21;
    p2.name = String::from("Jane");

    println!("{:?}", p1);
    println!("{:?}", p2);
}
