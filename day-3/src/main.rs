fn main() {
    let input = std::fs::read_to_string("./input").unwrap();
    
    let lines = input.split("\n");

    let mut collection: Vec<u32> = Vec::new();

    for line in lines {
        let compartment_size = line.len() / 2;
        let compartment_one = line.chars().take(compartment_size);
        let compartment_two = line.chars().skip(compartment_size);
        
        'outer: for x in compartment_one.clone() {
            for y in compartment_two.clone() {
                if y == x {
                    if y.is_uppercase() {
                        collection.push(y as u32 - 38);
                        break 'outer;
                    } else {
                        collection.push(y as u32 - 96);
                        break 'outer;
                    }
                }
            }
        }
    }
    let part1_answer: u32 = collection.iter().sum();
    println!("Part 1 Answer: {}", part1_answer);
}
