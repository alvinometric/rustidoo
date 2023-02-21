// Given a string of parenthesis,
// return the number of parenthesis you need to add to the string in order for it to be balanced.
// > numBalanced(`(()`)
// > 1
fn num_balanced(parens: &str) -> i32 {
    let mut balance: i32 = 0;
    for c in parens.chars() {
        if c == '(' {
            balance += 1;
        } else if c == ')' {
            balance -= 1
        }
    }
    return balance.abs();
}

fn main() {
    let result = num_balanced("))()))))()");
    println!("{}", result);
}

#[test]
fn test_num_balanced() {
    assert_eq!(num_balanced("()"), 0);
    assert_eq!(num_balanced("(()"), 1);
    assert_eq!(num_balanced("))()))))()"), 6);
    assert_eq!(num_balanced(")))))"), 5);
}
