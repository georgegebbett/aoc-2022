use std::fs;
extern crate array_tool;
use array_tool::vec::Intersect;


pub fn main() {

    let input = fs::read("./src/inputs/day3.txt").unwrap();
 
    let input_as_string = input.into_iter().map(
        |y| y as char
    ).collect::<String>();

    let rucksacks = input_as_string.split("\n").map(|x| {String::from(x)});

    let sack_lengths = rucksacks.clone().map(rucksack_mapper);

    let answer = sack_lengths.reduce(|p, c| {p + c});

    println!("The sum of dupe priorities is {:?}", answer.expect("That's no number"));

    let binding = rucksacks.clone().collect::<Vec<String>>();

    let groups = binding.chunks_exact(3).map(group_mapper);

    let answer2 = groups.reduce(|p,c| p + c);

    println!("The sum of badge priorities {:?}", answer2.expect("that's not a number"));

}

fn char_to_u16 (c: char) -> u16 {
    let mut number = c as u16;

    if number > 96 {number = number - 96}
    else {number = number - 38};

    return number;
}

fn rucksack_mapper (sack: String) -> u16 {

    let length = sack.chars().collect::<Vec<char>>().len();

    let mut compartment1 = sack.clone();
    let compartment2 = compartment1.split_off(length/2);

    let intersection = compartment1.chars().collect::<Vec<char>>().intersect(compartment2.chars().collect::<Vec<char>>());

    return char_to_u16(intersection[0]);

}

fn group_mapper (group: &[String]) -> u16 {

    let intersection = group[0].chars().collect::<Vec<char>>().intersect(group[1].chars().collect::<Vec<char>>()).intersect(group[2].chars().collect::<Vec<char>>());

    return char_to_u16(intersection[0]);

}