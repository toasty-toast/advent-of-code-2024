use std::{collections::HashMap, process};

use regex::Regex;

mod utils;

pub fn main() {
    let (list1, list2) = parse_input();
    solve_part1(&list1, &list2);
    solve_part2(&list1, &list2);
}

fn solve_part1(list1: &Vec<i32>, list2: &Vec<i32>) {
    let mut list1 = list1.clone();
    let mut list2 = list2.clone();
    list1.sort();
    list2.sort();

    let mut sum = 0;
    for (first, second) in list1.iter().zip(list2.iter()) {
        sum += first.abs_diff(*second);
    }

    println!("Part 1: {}", sum);
}

fn solve_part2(list1: &Vec<i32>, list2: &Vec<i32>) {
    let mut list2_counts: HashMap<i32, i32> = HashMap::new();
    list2.iter().for_each(|num| {
        *list2_counts.entry(*num).or_insert(0) += 1;
    });

    let mut score: i32 = 0;
    list1.iter().for_each(|num| {
        let count = list2_counts.get(num).unwrap_or(&0);
        score += num * count;
    });

    println!("Part 2: {}", score);
}

fn parse_input() -> (Vec<i32>, Vec<i32>) {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let input = utils::read_puzzle_input();
    let re = Regex::new(r"(\d+)\s+(\d+)").unwrap();
    input.iter().for_each(|line| {
        if let Some(captures) = re.captures(line) {
            let num1: i32 = captures.get(1).unwrap().as_str().parse().unwrap();
            let num2: i32 = captures.get(2).unwrap().as_str().parse().unwrap();
            list1.push(num1);
            list2.push(num2);
        } else {
            eprintln!("Error parsing line: {}", line);
            process::exit(1);
        }
    });
    return (list1, list2);
}
