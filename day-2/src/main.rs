// A / X = Rock | If won bonus score for chosen item = 1
// B / Y = Paper | If won bonus score for chosen item = 2
// C / Z = Scissors | If won bonus score for chosen item = 3
// Outcome of round: 0 for Loss, 3 for a draw, 6 if you won

// Part 2
// X = Lose
// Y = Draw
// Z = Win

fn main() {
    let input = std::fs::read_to_string("./input.in").unwrap();

    let lines = input.split("\n");

    let mut part1_score: Vec<i32> = Vec::new();
    let mut part2_score: Vec<i32> = Vec::new();
    
    for line in lines {
        let opp_score = convert_to_number(&line.chars().nth(0).unwrap());
        let my_sc = convert_to_number(&line.chars().nth(2).unwrap());

        // part 1
        if opp_score as f32 / my_sc as f32 != 1.0 {
            if my_sc - 1 == opp_score || my_sc - 1 == opp_score - 3 {
                part1_score.push(6 + my_sc)
            } else {
                part1_score.push(my_sc);
            }
        } else {
            part1_score.push(3 + my_sc);
        }

        // part 2
        match my_sc {
            1=>{
                if opp_score - 1 == 0 {
                    part2_score.push(3);
                } else {
                    part2_score.push(opp_score-1);
                }
            },
            2=>part2_score.push(opp_score+3),
            3=>{
                if opp_score + 1 == 4 {
                    part2_score.push(1+6); // Rock + Win
                } else {
                    part2_score.push((opp_score+1)+6); // Next one up + Win
                }
            },
            __=>println!("Failure.")
        }
    }
    
    let part1_final_score: i32 = part1_score.iter().sum();
    let part2_final_score: i32 = part2_score.iter().sum();
    println!("1st Part: {}", part1_final_score);
    println!("2nd Part: {}", part2_final_score);
}


fn convert_to_number(value: &char) -> i32 {
    if *value == "A".chars().nth(0).unwrap() || *value == "X".chars().nth(0).unwrap() {
        return 1;
    } else if *value == "B".chars().nth(0).unwrap() || *value == "Y".chars().nth(0).unwrap()  {
        return 2;
    } else if *value == "C".chars().nth(0).unwrap() || *value == "Z".chars().nth(0).unwrap() {
        return 3;
    }
    else {
        return 0;
    }
}