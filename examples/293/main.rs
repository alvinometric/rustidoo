use rand::Rng;

fn roll_dice(dice_str: &str) -> i32 {
    let mut rng = rand::thread_rng();
    // if there's a "+", there's more than one turn
    let turns: Vec<&str> = dice_str.split('+').collect();

    let mut min = 0;
    let mut max = 0;

    for turn in turns {
        let roll: Vec<&str> = turn.split("d").collect();
        let how_many: i32 = roll[0].parse().unwrap();
        let sides: i32 = roll[1].parse().unwrap();
        min += 1;
        max += how_many * sides;
    }

    let roll = rng.gen_range(min..max + 1);

    return roll;
}
// Given a string in dice notation (i.e., '3d20' = Three 20-sided dice)
// return a random integer you can get by rolling those dice.
fn main() {
    let roll = roll_dice("1d8+2d10");
    println!("{}", roll);
}
#[test]
fn test_roll_dice() {
    let mut result = roll_dice("4d4");
    assert!(
        result >= 4 && result <= 16,
        "Result is out of range: {}",
        result
    );
    result = roll_dice("3d20");
    assert!(
        result >= 3 && result <= 60,
        "Result is out of range: {}",
        result
    );
    result = roll_dice("1d8+2d10");
    assert!(
        result >= 3 && result <= 28,
        "Result is out of range: {}",
        result
    );
}
