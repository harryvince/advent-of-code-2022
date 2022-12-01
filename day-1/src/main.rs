fn main()  {
    let input = include_str!("./data.txt");

    let lines = input.split("\n\n");

    let linesParsed: Option<u32> = lines
        .map(|line| line.split("\n")
        .flat_map(|num| num.parse::<u32>())
        .sum::<u32>())
        .max();

    println!("{:?}", linesParsed);
}