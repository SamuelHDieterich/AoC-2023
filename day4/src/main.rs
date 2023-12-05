use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Card {
    id: usize,
    winning: Vec<u32>,
    have: Vec<u32>,
}

fn parser(line: String) -> Card {
    let Some((card_section, number_section)) = line.split_once(": ") else {
        panic!();
    };
    let id = card_section.split_whitespace().collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    let numbers = number_section
        .split(" | ")
        .map(|num_list| {
            num_list
                .split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Card {
        id: id,
        winning: numbers[0].to_owned(),
        have: numbers[1].to_owned(),
    }
}

fn count_winning_conditions(card: &Card) -> usize {
    card.have
        .iter()
        .map(|num| card.winning.contains(num))
        .filter(|res| *res)
        .count()
}

fn main() -> io::Result<()> {
    let file = File::open("input").expect("Can't open file");
    let reader = BufReader::new(file);

    let data: Vec<Card> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parser(line))
        .collect();

    let points_part1: u32 = data
        .iter()
        .map(|card| count_winning_conditions(card))
        .map(|res| {
            if res != 0 {
                u32::pow(2, u32::try_from(res).unwrap() - 1_u32)
            } else {
                0
            }
        })
        .sum();

    println!("{}", points_part1);

    let mut multipliers = vec![1_usize; data.len()];
    for card in data {
        let wins = count_winning_conditions(&card);
        for i in card.id..card.id + wins {
            multipliers[i] += multipliers[card.id - 1]
        }
    }
    let points_part2: usize = multipliers.iter().sum();

    println!("{}", points_part2);

    Ok(())
}
