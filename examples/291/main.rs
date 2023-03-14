use fraction::Fraction;

fn frac_to_num(frac: &str) -> f64 {
    let operands: Vec<&str> = frac.split("/").collect();
    let a: f64 = operands[0].parse().unwrap();
    let b: f64 = operands[1].parse().unwrap();
    return a / b;
}

fn do_operation(a: f64, b: f64, operation: &str) -> f64 {
    let result = match operation {
        "add" => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide" => a / b,
        &_ => a + b,
    };

    return result;
}

fn fraction_math(a: &str, operation: &str, b: &str) -> String {
    let num_a = frac_to_num(a);
    let num_b = frac_to_num(a);
    let num_result = do_operation(num_a, num_b, operation);
    return format!("{}", Fraction::from(num_result));
}

fn main() {
    let r = fraction_math("3/4", "add", "3/4");
    println!("{}", r);
}
