fn generate_arrays(iterations: i32) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for i in 0..iterations {
        let mut item = Vec::new();
        for j in 0..i + 1 {
            item.push(j + 1);
        }

        result.push(item);
    }

    println!("{:?}", result);

    return result;
}

fn main() {
    generate_arrays(3);
    println!("hello");
}
