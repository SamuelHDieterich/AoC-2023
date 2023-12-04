use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(matrix: &Vec<Vec<char>>) -> u32 {
    let number_rows = matrix.len();
    let number_cols = matrix[0].len();

    let mut numbers_start_position: HashSet<(usize, usize)> = Default::default();

    for row in 0..number_rows {
        for col in 0..number_cols {
            if matrix[row][col].is_ascii_digit() || matrix[row][col] == '.' {
                continue;
            }
            for search_row in {
                if row == 0 {
                    1
                } else {
                    row
                }
            } - 1..=row + 1
            {
                for search_col in {
                    if col == 0 {
                        1
                    } else {
                        col
                    }
                } - 1..=col + 1
                {
                    let mut offset: usize = 0;
                    if search_row >= number_rows
                        || search_col >= number_cols
                        || !matrix[search_row][search_col].is_ascii_digit()
                    {
                        continue;
                    }
                    while search_col > offset
                        && matrix[search_row][search_col - offset - 1].is_ascii_digit()
                    {
                        offset += 1;
                    }
                    numbers_start_position.insert((search_row, search_col - offset));
                }
            }
        }
    }

    let mut sum: u32 = 0;
    for (row, col) in numbers_start_position {
        let mut text = String::new();
        let mut offset: usize = 0;
        while col + offset < number_cols && matrix[row][col + offset].is_ascii_digit() {
            text.push(matrix[row][col + offset]);
            offset += 1;
        }
        sum += text.parse::<u32>().unwrap();
    }
    sum
}

fn part2(matrix: &Vec<Vec<char>>) -> u32 {
    let number_rows = matrix.len();
    let number_cols = matrix[0].len();

    let mut sum: u32 = 0;
    for row in 0..number_rows {
        for col in 0..number_cols {
            if matrix[row][col] != '*' {
                continue;
            }

            let mut possible_values: HashSet<(usize, usize)> = Default::default();

            for search_row in {
                if row == 0 {
                    1
                } else {
                    row
                }
            } - 1..=row + 1
            {
                for search_col in {
                    if col == 0 {
                        1
                    } else {
                        col
                    }
                } - 1..=col + 1
                {
                    let mut offset: usize = 0;
                    if search_row >= number_rows
                        || search_col >= number_cols
                        || !matrix[search_row][search_col].is_ascii_digit()
                    {
                        continue;
                    }
                    while search_col > offset
                        && matrix[search_row][search_col - offset - 1].is_ascii_digit()
                    {
                        offset += 1;
                    }
                    possible_values.insert((search_row, search_col - offset));
                }
            }
            if possible_values.len() != 2 {
                continue;
            }

            let mut numbers: Vec<u32> = Vec::with_capacity(2);
            for (row, col) in &possible_values {
                let mut text = String::new();
                let mut offset: usize = 0;
                while *col + offset < number_cols && matrix[*row][*col + offset].is_ascii_digit() {
                    text.push(matrix[*row][*col + offset]);
                    offset += 1;
                }
                numbers.push(text.parse::<u32>().unwrap());
            }
            sum += numbers[0] * numbers[1];
        }
    }
    sum
}

fn main() -> io::Result<()> {
    let file = File::open("input").expect("Can't open file");
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|line| line.unwrap());

    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars = line.chars().collect();
        matrix.push(chars);
    }

    let part1_sum = part1(&matrix);
    let part2_sum = part2(&matrix);

    println!("{}", part1_sum);
    println!("{}", part2_sum);

    Ok(())
}
