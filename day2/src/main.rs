use std::fs::File;
use std::io::{self, prelude::*, BufReader};
mod game;
use game::*;

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let bag = Set::new(12, 13, 14);
    let mut sum_possible: u32 = 0;
    let mut sum_power_cubes: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let game = parse_line(&line);
        if is_possible(&game, &bag) {
            sum_possible += game.id
        }
        sum_power_cubes += power_cubes(&game);
    }

    print!("Sum of possible games IDs: {}\n", sum_possible);
    print!("Sum of power cubes: {}\n", sum_power_cubes);

    Ok(())
}
