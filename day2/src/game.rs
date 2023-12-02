#[derive(Debug, Default)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    pub fn new(red: u32, green: u32, blue: u32) -> Self {
        Self { red, green, blue }
    }
    fn is_possible(&self, bag: &Self) -> bool {
        if self.red > bag.red {
            return false;
        }
        if self.green > bag.green {
            return false;
        }
        if self.blue > bag.blue {
            return false;
        }
        return true;
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            sets: Vec::new(),
        }
    }

    fn add_set(&mut self, set: Set) {
        self.sets.push(set);
    }
}

pub fn parse_line(line: &str) -> Game {
    let line_parts: Vec<&str> = line.split(":").collect();
    let game_part: Vec<&str> = line_parts[0].split(" ").collect();
    let mut game = Game::new(game_part[1].parse().unwrap());
    let sets_part = line_parts[1].split(";");
    for set in sets_part {
        let mut set_struct: Set = Default::default();
        let dices = set.split(",");
        for dice in dices {
            let (amount_str, color) = dice.trim().split_once(" ").unwrap();
            let amount = amount_str.parse().unwrap();
            match color {
                "red" => set_struct.red = amount,
                "green" => set_struct.green = amount,
                "blue" => set_struct.blue = amount,
                _ => continue,
            }
        }
        game.add_set(set_struct);
    }
    game
}

pub fn is_possible(game: &Game, bag: &Set) -> bool {
    for set in &game.sets {
        if !set.is_possible(bag) {
            return false;
        }
    }
    return true;
}

pub fn power_cubes(game: &Game) -> u32 {
    let mut max_set: Set = Default::default();
    for set in &game.sets {
        if set.red > max_set.red {
            max_set.red = set.red
        }
        if set.green > max_set.green {
            max_set.green = set.green
        }
        if set.blue > max_set.blue {
            max_set.blue = set.blue
        }
    }
    return max_set.red * max_set.green * max_set.blue;
}
