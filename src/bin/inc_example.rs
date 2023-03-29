use std::fs;

fn get_latest_dir() -> i32 {
    let paths = fs::read_dir("./examples").unwrap();
    let mut greatest = 0;
    for path in paths {
        println!("Name: {:#?}", path.unwrap());
    }

    return 2;
}

fn main() {
    let latest = get_latest_dir();
    println!("{}", latest);
}
