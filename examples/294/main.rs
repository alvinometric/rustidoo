// Write a program that prints the amount of characters its source has in English.
// So a program that is 44 characters long would output “forty four”
use num2words::Num2Words;

fn main() {
    let source = include_str!("./main.rs");
    let chars = source.len() as i32;
    let spelled_out = Num2Words::new(chars).to_words().unwrap();
    println!("{}", spelled_out);
}
