fn str_to_frac(string: &str) -> f64 {
    let operands: Vec<&str> = string.split("/").collect();
    let a: f64 = operands[0].parse().unwrap();
    let b: f64 = operands[1].parse().unwrap();

    return a / b;
}

fn fraction_math(a: &str, operation: &str, b: &str) {
    let frac_a = str_to_frac(a);
    println!("{}", frac_a);
}

fn main() {
    println!("{}", "hi");
    let r = fraction_math("3/4", "add", "3/4");
}
