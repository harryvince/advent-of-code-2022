use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./src/input")?
        .split("")
        .map(|s| s.to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>();

    let mut count = 0;
    for x in 0..input.len() {
        if count == 4 {
            println!("Part 1 Answer: {}", x);
            break;
        }
        if input[x] == input[x + 1] || input[x] == input[x + 2] || input[x] == input[x + 3] {
            count = 0;
        } else {
            count += 1;
        }
    }

    const MESSAGE_SIZE: usize = 14; 
    for x in 0..input.len() {
        let mut character_vector: Vec<String> = Vec::new();
        for y in 0..MESSAGE_SIZE {
            character_vector.push(input[x+y].clone());
        }
        let message_starting_point = character_vector.iter().unique().collect::<Vec<&String>>();
        if message_starting_point.len() == 14 {
            println!("Part 2 Answer: {}", x + MESSAGE_SIZE);
            break;
        }
    }

    Ok(())
}