use anyhow::Result;

fn main() -> Result<()> {
    let mut part1_answer = 0;
    let input = std::fs::read_to_string("./input")?
        .lines()
        .flat_map(|line| {
            let values: Vec<&str> = line.split(",").collect();
            let left_side: Vec<i32> = values[0].split("-").into_iter().map(|number| number.parse::<i32>().unwrap()).collect();
            let right_side: Vec<i32> = values[1].split("-").into_iter().map(|number| number.parse::<i32>().unwrap()).collect();
            
            if left_side[0] >= right_side[0] && left_side[1] <= right_side[1] || right_side[0] >= left_side[0] && right_side[1] <= left_side[1] {
                part1_answer += 1;
            }

            return values;
        }).collect::<Vec<_>>();

    println!("Part 1 Answer: {}", part1_answer);
    return Ok(())
}
