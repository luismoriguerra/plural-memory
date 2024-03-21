#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 'a show all vars are valid for the same lifetime
fn get_oldest<'a>(p1: &'a Person, p2: &'a Person) -> &'a Person {
    if p1.age > p2.age {
        p1
    } else {
        p2
    }
}

fn main() {
    let p1 = Person {
        name: String::from("John"),
        age: 25,
    };
    let p2 = Person {
        name: String::from("Jane"),
        age: 26,
    };

    {
        let p3 = get_oldest(&p1, &p2);
        println!("The oldest person is {:?}", p3);
    }
    println!("p1: {:?}", p1);
}
