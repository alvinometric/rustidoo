fn scramble(str: &str) -> String {
    let words = str.split(" ");

    let mut out = String::new();
    for word in words {
        if word.len() > 3 {
            let mut w: Vec<char> = word.chars().collect();
            w.swap(2, 3);
            out.push_str(&String::from_iter(w));
        } else {
            out.push_str(word);
        }
        out.push_str(" ")
    }
    return out.trim().to_string();
}

fn main() {
    let s = scramble("A quick brown fox jumped over the lazy dog");
    println!("{:?}", s);
}
