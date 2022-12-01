use std::fs;
pub fn day1() {
    let input = fs::read("./src/inputs/day1.txt").unwrap();
 
    let input_as_string = input.into_iter().map(
        |y| y as char
    ).collect::<String>();

    let elves = input_as_string.split("\n\n");

    let elves_pockets = elves.map(|x| {x.split('\n')});

    let pockets_as_i32 = elves_pockets.map(|x| {x.map(|d| {d.parse::<i32>().unwrap()})});

    let mut pocket_sums = pockets_as_i32.map(|x|{x.reduce(|p, c| {p + c}).unwrap()}).collect::<Vec<i32>>();
    
    pocket_sums.sort();

    pocket_sums.reverse();

    println!("The biggest calorie value is {:?}", pocket_sums.first().unwrap());

    pocket_sums.truncate(3);

    let top_3_sum = pocket_sums.into_iter().reduce(|p, c| {p + c}).unwrap();

    println!("The top 3 calorie values summed come to {:?}", top_3_sum)

}
