fn star_angles(n: f32, min_points: u8) -> Vec<f32> {
    if n < min_points as f32 {
        return vec![];
    }

    let mut angles = vec![];

    let interior_angle = (180.0 * (n - 2.0)) / n; // Interior angle of a regular polygon with n points
    let star_angle = 180.0 - (180.0 / n);

    for i in 0..n as usize {
        let angle = interior_angle - 2.0 * star_angle * (i as f32 % 2.0);
        angles.push(angle.round());
    }

    return angles;
}

fn main() {
    println!("Hello");
    let min_points = 5;
    let angles = star_angles(7.0, min_points);

    if angles.len() > 0 {
        println!("{:?}", angles);
    } else {
        println!("Not enough points, needs > 5")
    }
}
