#[derive(Debug)]
struct Person<'a> {
    name: String,
    age: u8,
    // whaterver value that this value points to
    //must live as long as the Person
    favorite_fruit: &'a str,
}

fn main() {
    let winter_fruits: [String; 4] = [
        String::from("Apple"),
        String::from("Banana"),
        String::from("Orange"),
        String::from("Pear"),
    ];

    let p1 = Person {
        name: String::from("John"),
        age: 25,
        favorite_fruit: &winter_fruits[0],
    };

    let mut p2 = Person {
        name: String::from("Jane"),
        age: 26,
        favorite_fruit: &winter_fruits[1],
    };

    {
        let summer_fruits: [String; 4] = [
            String::from("Watermelon"),
            String::from("Peach"),
            String::from("Strawberry"),
            String::from("Pineapple"),
        ];

        p2 = Person {
            name: String::from("Jane"),
            age: 26,
            // summer_fruits won't leave enough like p2
            favorite_fruit: &summer_fruits[0],
        };
    }

    println!("p1: {:?}", p1);
    println!("p2: {:?}", p2);
}
