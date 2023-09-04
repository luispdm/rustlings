// generics1.rs
//
// This shopping list program isn't compiling! Use your knowledge of generics to
// fix it.
//
// Execute `rustlings hint generics1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let mut shopping_list: Vec<&str> = Vec::new();
    /*
     * the exercise wants us to specify the generic type for Vec,
     * but we could have removed the type definition entirely because Rust
     * is able to infer the type of Vec with the following instruction
     */
    shopping_list.push("milk");
}
