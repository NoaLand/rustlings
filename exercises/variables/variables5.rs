// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // this is called shadowing in rust, which means, you can use `let` to change the type of a variable
    let number = 3; // don't rename this variable
    println!("Number plus two is : {}", number + 2);
}
