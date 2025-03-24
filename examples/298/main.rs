fn remove_zeroes(arr: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for item in arr {
        if item != 0 {
            result.push(item);
        }
    }
    return result;
}

fn main() {
    println!(
        "{:?}",
        remove_zeroes(vec![0, 0, 0, 3, 1, 4, 1, 5, 9, 0, 0, 0, 0])
    );
    // [3, 1, 4, 1, 5, 9]
    println!("{:?}", remove_zeroes(vec![0, 0, 0]));
    // []
    println!("{:?}", remove_zeroes(vec![8]));
    // [8]
}
