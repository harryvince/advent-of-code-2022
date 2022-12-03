use itertools::Itertools;

fn get_value(character: &char) -> u32 {
    if character.is_uppercase() {
        return *character as u32 - 38
    } else {
        return  *character as u32 - 96;
    }
}

fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    
    let lines = input.split("\n");

    let mut collection: Vec<u32> = Vec::new();

    let mut lines_3: Vec<char> = Vec::new();
    let mut part2_answer: u32 = 0;

    let mut counter = 1;
    for line in lines {
        let compartment_size = line.len() / 2;
        let compartment_one: String = line.chars().take(compartment_size).unique().collect();
        let compartment_two: String = line.chars().skip(compartment_size).unique().collect();
        
        'outer: for x in 0..compartment_one.len() {
            let value_for_cmp1 = &compartment_one.chars().nth(x).unwrap();
            for y in 0..compartment_two.len() {
                let value_for_cmp2 = &compartment_two.chars().nth(y).unwrap();
                if value_for_cmp2 == value_for_cmp1 {
                        collection.push(get_value(value_for_cmp2));
                        break 'outer;
                }
            }
        }

        let line_to_push: Vec<char> = line.chars().into_iter().unique().collect();
        lines_3.extend(line_to_push);
        counter += 1;

        if counter % 3 == 1 {
            lines_3.sort();
            let badge: Vec<(usize, char)> = lines_3.clone().into_iter().dedup_with_count().filter(|&value| value.0 == 3).collect();
            part2_answer += get_value(&badge[0].1);
            lines_3.clear();
            counter = 1;
        }
    }
    let part1_answer: u32 = collection.iter().sum();
    println!("Part 1 Answer: {}", part1_answer);
    println!("Part 2 Answer: {}", part2_answer);
}
