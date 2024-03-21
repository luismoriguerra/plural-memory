#[derive(Debug)]
struct Person<'a> {
    name: String,
    age: u8,
    favorite_fruit: &'a str,
}

impl<'a> Person<'a> {
    // next arguments inherit the same lifetime as the struct
    fn print_and_return_fruit(&self, announcement: &str) -> &str {
        println!("The favorite fruit is: {}", self.favorite_fruit);
        println!("{}", announcement);
        self.favorite_fruit
    }
}

fn main() {}
