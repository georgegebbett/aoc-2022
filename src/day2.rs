use std::fs;

use crate::utils_building::utils::sum_reducer;


pub fn main() {
    let input = fs::read("./src/inputs/day2.txt").unwrap();
 
    let input_as_string = input.into_iter().map(
        |y| y as char
    ).collect::<String>();

    let rounds = input_as_string.split("\n").map(|x| {String::from(x)});

    let rounds2 = rounds.clone().map(round_scorer).reduce(sum_reducer);

    println!("Following the score guide as per part one gives you a score of {:?}", rounds2.unwrap());

    let rounds3 = rounds.map(decoder).map(round_scorer).reduce(sum_reducer);

    println!("Following the score guide as per part two gives you a score of {:?}", rounds3.unwrap());
    
}

fn round_scorer (round: String) -> i32 {

    // Rock -     A X 1
    // Paper -    B Y 2
    // Scissors - C Z 3

    let a: Vec<char> = round.chars().collect();

    let me = a[2];
    let op = a[0];

    let is_win = 
           (op == 'A' && me == 'Y')
        || (op == 'B' && me == 'Z')
        || (op == 'C' && me == 'X');

    let is_draw = 
        (op == 'A' && me == 'X')
     || (op == 'B' && me == 'Y')
     || (op == 'C' && me == 'Z');

    let is_loss = !(is_win || is_draw);

    let mut score = 0;
    
    let win_score = 6;
    let draw_score = 3;
    let lose_score = 0;
    
    let rock_score = 1;
    let paper_score = 2;
    let scissors_score = 3;

    if is_win {score = score + win_score};
    if is_draw {score = score + draw_score};
    if is_loss {score = score + lose_score};

    if me == 'Y' {score = score + paper_score}
    if me == 'X' {score = score + rock_score}
    if me == 'Z' {score = score + scissors_score}

    return score;

}

fn decoder (code: String) -> String {

    // Rock -     A X 1
    // Paper -    B Y 2
    // Scissors - C Z 3

    // Lose - X
    // Draw - Y
    // Win  - Z

    let a: Vec<char> = code.chars().collect();

    let outcome = a[2];
    let op = a[0];

    let me: Option<char>;

    if outcome == 'X' {
        match op {
            'A' => me = Some('Z'),
            'B' => me = Some('X'),
            'C' => me = Some('Y'),
            _ => me = None,
        }
    } else if outcome == 'Y' {
        match op {
            'A' => me = Some('X'),
            'B' => me = Some('Y'),
            'C' => me = Some('Z'),
            _ => me = None,
        }
    } else {
        match op {
            'A' => me = Some('Y'),
            'B' => me = Some('Z'),
            'C' => me = Some('X'),
            _ => me = None,
        }
    }

    return format!("{} {}", op, me.unwrap());

}