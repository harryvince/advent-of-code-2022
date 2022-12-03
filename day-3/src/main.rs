fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    
    let lines = input.split("\n");

    let mut collection: Vec<u32> = Vec::new();

    for line in lines {
        let compartment_size = line.len() / 2;
        let compartment_one: String = line.chars().take(compartment_size).collect();
        let compartment_two: String = line.chars().skip(compartment_size).collect();
        
        'outer: for x in 0..compartment_one.len() {
            let value_for_cmp1 = &compartment_one.chars().nth(x).unwrap();
            for y in 0..compartment_two.len() {
                let value_for_cmp2 = &compartment_two.chars().nth(y).unwrap();
                if value_for_cmp2 == value_for_cmp1 {
                    if value_for_cmp2.is_uppercase() {
                        collection.push(*value_for_cmp2 as u32 - 38);
                        break 'outer;
                    } else {
                        collection.push(*value_for_cmp2 as u32 - 96);
                        break 'outer;
                    }
                }
            }
        }
    }
    let part1_answer: u32 = collection.iter().sum();
    println!("Part 1 Answer: {}", part1_answer);
}
