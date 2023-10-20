use std::io;
fn main() {
    let mut guess = String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("Reading Error");

    println!("Hello, world!");

    let x = 5;
    let y = 10;
    let mut z = 0;
    z = x * y;

    println!("the value of z is {z}");
    println!("The guessed value is {guess}");
}
