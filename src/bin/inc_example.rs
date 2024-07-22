use std::error::Error;
use std::fs;

// go through the /examples folder
// find the directory named with the greatest number
fn get_latest_dir() -> i32 {
    let dir_names: Vec<String> = fs::read_dir("./examples")
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|p| p.is_dir())
        .map(|path| path.to_str().unwrap().to_owned())
        .collect();

    let mut greatest = 0;

    for dir_name in dir_names {
        let top_level = dir_name.split("/").last().unwrap();
        let top_level_num: i32 = top_level.parse().unwrap();

        if top_level_num > greatest {
            greatest = top_level_num;
        }
    }

    return greatest;
}

fn main() -> Result<(), Box<dyn Error>> {
    let latest = get_latest_dir();
    let new_dir = format!("./examples/{}", latest + 1);
    let new_file = format!("{}/main.rs", new_dir);

    fs::create_dir(new_dir)?;

    fs::write(new_file, "")?;

    println!("✅ Created a new ./examples folder");
    println!("🚀 Run the following command to start the latest example");
    println!("👇");
    println!(r#"cargo watch -x "run --example {}""#, latest + 1);
    Ok(())
}
