fn main() {
    // This is a constant with a static lifetime
    // it will live for the entire duration of the program
    const GLOBAL_GRETTING: &'static str = "Hello, World!";
}
