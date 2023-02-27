use std::collections::HashMap;

fn repeated_groups(arr: Vec<i32>) -> Vec<Vec<i32>> {
    let mut pairs = Vec::new();
    let mut seen = HashMap::new();

    for i in 0..arr.len() {
        let item = arr[i];
        if seen.contains_key(&item) {
            let mut count: i32 = seen.get(&item).copied().unwrap();
            count += 1;
            seen.insert(item, count);
        } else {
            seen.insert(item, 1);
        }
    }

    for (k, val) in &seen {
        let v = val.to_owned();
        if v > 1 {
            pairs.push(vec![k.to_owned(); v as usize]);
        }
    }

    return pairs;
}

fn main() {
    let result = repeated_groups(vec![1, 1, 0, 0, 8, 4, 4, 4, 3, 2, 1, 9, 9]);
    println!("result {:?}", result);
}

#[test]
fn test_repeated_groups() {
    let input1 = vec![1, 2, 2, 4, 5];
    let expected_output1 = vec![vec![2, 2]];
    assert_eq!(repeated_groups(input1), expected_output1);

    let input2 = vec![1, 1, 0, 0, 8, 4, 4, 4, 3, 2, 1, 9, 9];
    let expected_output2 = vec![vec![1, 1], vec![0, 0], vec![4, 4, 4], vec![9, 9]];
    assert_eq!(repeated_groups(input2), expected_output2);
}
