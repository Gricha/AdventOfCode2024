use std::collections::HashMap;

use itertools::Itertools;

use crate::utils::read_input;

pub fn part1() {
    let input = read_input("inputs/01_1.txt")
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .collect_vec()
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect_vec();
    let list1 = input.iter().map(|(x, _)| x).sorted().collect_vec();
    let list2 = input.iter().map(|(_, x)| x).sorted().collect_vec();
    let result = list1
        .into_iter()
        .zip(list2)
        .map(|(x, y)| x - y)
        .map(|x| x.abs())
        .sum::<i32>();
    println!("{}", result);
}

pub fn part2() {
    let input = read_input("inputs/01_1.txt")
        .into_iter()
        .map(|x| {
            x.split_whitespace()
                .collect_vec()
                .into_iter()
                .map(|x| x.parse::<i32>().unwrap())
                .collect_tuple::<(i32, i32)>()
                .unwrap()
        })
        .collect_vec();
    let mut right = HashMap::new();
    for (_, y) in &input {
        *right.entry(*y).or_insert(0) += 1;
    }

    let result = input
        .into_iter()
        .fold(0, |acc, (k, _)| acc + (k * right.get(&k).unwrap_or(&0)));
    println!("{}", result);
}
