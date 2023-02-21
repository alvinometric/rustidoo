fn index_of(arr: &str, value: char) -> i32 {
    let mut index: i32 = 0;
    for c in arr.chars() {
        if c == value {
            return index;
        }
        index += 1;
    }
    return -1;
}

fn main() {
    let chars = "abcdefghij";

    let one = index_of(chars, 'b');
    // why does usize even exist 🤦‍♂️
    let ten = chars.len() as i32;
    let hundred = ten * ten;

    for i in one..hundred + one {
        println!("{}", i);
    }
}
