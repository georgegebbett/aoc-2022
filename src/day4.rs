use std::{fs, cmp::Ordering};


pub fn main() {
    let input = fs::read("./src/inputs/day4.txt").unwrap();
 
    let input_as_string = input.into_iter().map(
        |y| y as char
    ).collect::<String>();

    let pairs = input_as_string.split("\n").map(|x| {String::from(x)});

    let result = pairs.clone().map(fully_contains).collect::<Vec<bool>>();

    let sum = result.into_iter().filter(|x| x.clone() == true).collect::<Vec<bool>>().len();
    println!("{:?} ranges fully contain each other.", sum);

    let overlap = pairs.clone().map(overlaps).collect::<Vec<bool>>();
    let sum = overlap.into_iter().filter(|x| x.clone() == true).collect::<Vec<bool>>().len();


    println!("{:?} ranges overlap.", sum);
    

}

fn fully_contains(pairs: String) -> bool {

    let pairs2 = pairs.split(",");
    let pairs3 = pairs2.map(|x| x.split('-').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let pairs4 = pairs3.into_iter().map(|x| {x.into_iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()}).collect::<Vec<Vec<i32>>>();

    
    let mut pairs5 = pairs4.clone().concat();

    pairs5.sort();

    let first = pairs5.first().expect("eeee");
    let last = pairs5.last().expect("oooo");

    if (pairs4.first().unwrap().contains(first) && pairs4.first().unwrap().contains(last)) ||  
        (pairs4.last().unwrap().contains(first) && pairs4.last().unwrap().contains(last))
         {return true} else {return false};


}

fn overlaps(pairs: String) -> bool {
    let pairs2 = pairs.split(",");
    let pairs3 = pairs2.map(|x| x.split('-').collect::<Vec<&str>>()).collect::<Vec<Vec<&str>>>();

    let pairs4 = pairs3.into_iter().map(|x| {x.into_iter().map(|y| y.parse::<i32>().unwrap()).collect::<Vec<i32>>()}).collect::<Vec<Vec<i32>>>();

    let mut pairs5 = pairs4.clone();

    pairs5.sort_by(|x, y| {if x.first().unwrap() > y.first().unwrap() {return Ordering::Greater;} else {return Ordering::Less;}});

    return pairs5.first().unwrap().last().unwrap() >= pairs5.last().unwrap().first().unwrap();

}