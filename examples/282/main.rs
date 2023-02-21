fn sum_every_other(num: f64) -> u32 {
    let str = num.to_string();

    let mut result = 0;

    let mut i = 1;
    while i < str.len() {
        let c: char = str.chars().nth(i).unwrap();
        let digit = c.to_digit(10).unwrap();

        result += digit;
        i += 2;
    }

    return result;
}

fn main() {
    println!("🪅 Hello Rustidoo, this is issue 282");

    sum_every_other(10.0);
}

#[cfg(test)]
mod tests {
    use crate::sum_every_other;

    #[test]
    fn test_sum_every_other() {
        assert_eq!(sum_every_other(548915381.0), 26);
        assert_eq!(sum_every_other(10.0), 0);
        assert_eq!(sum_every_other(1010.11), 1);
    }
}
