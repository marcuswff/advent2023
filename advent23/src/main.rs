mod utils;
use utils::read_file;
use std::collections::HashMap;

const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/input01.txt";
// const PATH: &str = "/Users/marcus/git/advent2023/advent23/src/data/test/input01.txt";

fn main() {
    task1();
    task2();
}

fn task1() {
    let data = read_file(PATH);

    let res: usize = data
        .iter()
        .map(|row| row.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>())
        .map(|s| format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()))
        .map(|s| s.parse::<usize>().unwrap())
        .sum();

    println!("{res}");
}

fn task2() {
    let data = read_file(PATH);

    let lookup = get_letter_to_number_lookup();

    let res: usize = data
        .iter()
        .map(|s| replace_string(s.to_string(), &lookup) )
        .map(|row| row.chars()
        .filter(|c| c.is_numeric())
        .collect::<String>())
        .map(|s| format!("{}{}", s.chars().next().unwrap(), s.chars().last().unwrap()))
        .map(|s| s.parse::<usize>().unwrap())
        .sum();

        println!("{res}");
    }

fn replace_string(mut input_string: String, lookup: &HashMap<String, String>) -> String {

    for (k, v) in lookup {
        input_string = input_string.replace(k, format!("{k}{v}{k}").as_str())
    }

    input_string
}
 
fn get_letter_to_number_lookup() -> HashMap<String, String> {
    let letters: Vec<String> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let digits: Vec<String> = (1..=9).into_iter().map(|i| i.to_string()).collect();

    letters
        .iter()
        .zip(digits.iter())
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}