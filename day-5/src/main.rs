use anyhow::Result;

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("./src/input.test")?
    .lines()
    .map(|value| String::from(value))
    .collect();

    let cargo: Vec<String> = input.clone().into_iter().take(9).collect();
    let instructions: Vec<String> = input.into_iter().skip(10).collect();

    let mut sorted_cargo = Vec::new();
    let mut answer_string: String = "".to_string();

    let mut count = 0;
    for x in 0..cargo.len() {
        if x != 8 {
            let split_values: Vec<&str> = cargo[x].split(" ").collect();
            let values = split_values.iter().map(|s| {
                if s.is_empty() {
                    count += 1;
                }
                if count == 4 {
                    count = 0;
                    return "NULL".to_string();
                }
                let first_replace = s.replace("[", "");
                let return_value = first_replace.replace("]", "");
                return return_value.to_string()
            }).filter(|s| !s.is_empty()).collect::<Vec<String>>();
            sorted_cargo.push(values);
        } 
    }
    sorted_cargo.reverse();

    let mut stacks: Vec<Vec<String>> = Vec::new();
    for _i in 0..sorted_cargo[0].len() {
        stacks.push(Vec::new());
    }
    for x in 0..sorted_cargo.len() {
        for y in 0..sorted_cargo[x].len() {
            if sorted_cargo[x][y] != "NULL" {
                stacks[y].push(sorted_cargo[x][y].clone());
            }
        }
    }

    for line in instructions {
        let split_instructions: Vec<&str> = line.split(" ").collect();
        let stack_to_remove_from = split_instructions[3].parse::<usize>().unwrap() - 1;
        let stack_to_push_to = split_instructions[5].parse::<usize>().unwrap() - 1;
        for _x in 0..split_instructions[1].parse::<u32>().unwrap() {
            let pop_value = stacks[stack_to_remove_from].pop().unwrap();
            stacks[stack_to_push_to].push(pop_value);
        }
    }

    for x in 0..stacks.len() {
        let last_value = stacks[x].len() - 1;
        answer_string = answer_string + &stacks[x][last_value].to_string();
    }

    println!("{}", answer_string);

    return Ok(())
}
