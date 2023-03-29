fn hsl_to_rgb(hsl: Vec<f64>) -> Vec<f64> {
    let h = hsl[0];
    let s = hsl[1] / 100.0;
    let l = hsl[2] / 100.0;
    let s = s / 100.0;
    let l = l / 100.0;
    let k = |n| (n + h / 30.0) % 12.0;
    let a = s * l.min(1.0 - l);
    let f = |n| l - a * (-1.0f64).max((k(n) as f64 - 3.0).min(9.0f64 - k(n)).min(1.0) as f64);

    return vec![
        (255.0 * f(0.0)).round(),
        (255.0 * f(8.0)).round(),
        (255.0 * f(4.0)).round(),
    ];
}

fn convert_color(in_format: &str, out_format: &str, color: &str) -> String {
    let no_parentheses = color.replace("(", "").replace(")", "");
    let components: Vec<f64> = no_parentheses
        .split(",")
        .map(|n| n.to_string().parse().unwrap())
        .collect();

    let rgb = match in_format {
        "rgb" => components.clone(),
        "hsl" => hsl_to_rgb(components),

        &_ => components.clone(),
    };

    println!("{:?}", rgb);
    return "".to_string();
}

fn main() {
    let c = convert_color("rgb", "hex", "(255,0,0)");
    println!("{}", c);
}
