fn main()  {
    let input = include_str!("./data.txt");

    let lines = input.split("\n\n");

    let mut linesParsed: Vec<u32> = lines
        .map(|line| line.split("\n")
        .flat_map(|num| num.parse::<u32>())
        .sum::<u32>())
        .collect();

    linesParsed.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", linesParsed[0]);
    println!(
        "Part 2: {:?}",
        linesParsed.iter().take(3).sum::<u32>()
    );
}