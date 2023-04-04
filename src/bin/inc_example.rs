use std::fs;

fn get_latest_dir() -> i32 {
    let paths = fs::read_dir("./examples").unwrap();
    let mut greatest = 0;

    for path in paths {
        let p = path.unwrap().path();
        if p.is_dir() {
            let dir_name: String = p.to_str().unwrap().to_owned();
            let top_level = dir_name.split("/").last().unwrap();
            let top_level_num: i32 = top_level.parse().unwrap();

            if top_level_num > greatest {
                greatest = top_level_num;
            }
        }
    }

    return greatest;
}

fn main() {
    let latest = get_latest_dir();
    let new_dir = format!("./examples/{}", latest + 1);
    let new_file = format!("{}/main.rs", new_dir);
}
