// A / X = Rock | If won bonus score for chosen item = 1
// B / Y = Paper | If won bonus score for chosen item = 2
// C / Z = Scissors | If won bonus score for chosen item = 3
// Outcome of round: 0 for Loss, 3 for a draw, 6 if you won

fn main() {
    let input = std::fs::read_to_string("./input.in").unwrap();

    let lines = input.split("\n");

    let mut my_score: Vec<i32> = Vec::new();
    
    for line in lines {
        let opp_score = convert_to_number(&line.chars().nth(0).unwrap());
        let my_sc = convert_to_number(&line.chars().nth(2).unwrap());

        if opp_score as f32 / my_sc as f32 != 1.0 {
            if my_sc - 1 == opp_score || my_sc - 1 == opp_score - 3 {
                my_score.push(6 + my_sc)
            } else {
                my_score.push(my_sc);
            }
        } else {
            my_score.push(3 + my_sc);
        }
    }
    
    let final_score: i32 = my_score.iter().sum();
    println!("1st Part: {}", final_score);
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