use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn find_number(characters: &[u8], reversed: bool) -> Result<u8, ()> {
    let mut spelled_number = String::new();
    for character in characters {
        match character {
            b'0'..=b'9' => return Ok(character - b'0'), // 0..9
            b'a'..=b'z' => {
                spelled_number.push(*character as char); // a..z
                // Horrible code, but it works - Without regex
                match spelled_number {
                    _ if spelled_number.contains(if !reversed {"zero"}  else {"orez"})  => return Ok(0),
                    _ if spelled_number.contains(if !reversed {"one"}   else {"eno"})   => return Ok(1),
                    _ if spelled_number.contains(if !reversed {"two"}   else {"owt"})   => return Ok(2),
                    _ if spelled_number.contains(if !reversed {"three"} else {"eerht"}) => return Ok(3),
                    _ if spelled_number.contains(if !reversed {"four"}  else {"ruof"})  => return Ok(4),
                    _ if spelled_number.contains(if !reversed {"five"}  else {"evif"})  => return Ok(5),
                    _ if spelled_number.contains(if !reversed {"six"}   else {"xis"})   => return Ok(6),
                    _ if spelled_number.contains(if !reversed {"seven"} else {"neves"}) => return Ok(7),
                    _ if spelled_number.contains(if !reversed {"eight"} else {"thgie"}) => return Ok(8),
                    _ if spelled_number.contains(if !reversed {"nine"}  else {"enin"})  => return Ok(9),
                    _ => continue,
                }
            }
            _ => continue,
        }
    }
    Err(())
}

fn main() -> io::Result<()> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line = line?;
        let characters = line.as_bytes().to_vec();
        let mut characters_reverse = characters.clone();
        characters_reverse.reverse();
        let first_digit = find_number(&characters, false).unwrap() as u32; // Ten digit
        let second_digit = find_number(&characters_reverse, true).unwrap() as u32; // One digit
        sum += first_digit * 10 + second_digit;
        // print!("Input: {} - Numbers: {} and {} - Sum: {}\n", line, first_digit, second_digit, sum); // Debug
    }

    println!("Sum: {}", sum);

    Ok(())
}
