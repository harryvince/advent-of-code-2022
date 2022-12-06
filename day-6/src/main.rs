use anyhow::Result;

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

    Ok(())
}