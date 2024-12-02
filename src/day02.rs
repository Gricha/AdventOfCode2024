use itertools::Itertools;

use crate::utils::read_input;

pub fn part1() {
    let lines = read_input("inputs/02.txt");
    let reports: i32 = lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| {
            report
                .iter()
                .tuple_windows()
                .map(|(a, b)| a - b)
                .collect::<Vec<i32>>()
        })
        .map(|deltas| {
            if deltas[0] == 0 {
                return 0;
            }
            let has_negatives = deltas[0] < 0;
            let mut is_failed = 0;
            for d in deltas {
                if !(1..=3).contains(&d.abs()) {
                    is_failed = 1;
                    break;
                }
                if has_negatives && d > 0 {
                    is_failed = 1;
                    break;
                }
                if !has_negatives && d < 0 {
                    is_failed = 1;
                    break;
                }
            }

            1 - is_failed
        })
        .sum();
    println!("{}", reports);
}

pub fn part2() {
    let lines = read_input("inputs/02.txt");
    let reports: i32 = lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .map(|report| {
            let mut multiplied_reports = vec![report.clone()];

            for i in 0..report.len() {
                let mut new_report = report.clone();
                new_report.remove(i);
                multiplied_reports.push(new_report);
            }

            multiplied_reports
        })
        .map(|report_list| {
            report_list
                .into_iter()
                .map(|report| {
                    report
                        .iter()
                        .tuple_windows()
                        .map(|(a, b)| a - b)
                        .collect::<Vec<i32>>()
                })
                .map(|deltas| {
                    if deltas[0] == 0 {
                        return 0;
                    }
                    let has_negatives = deltas[0] < 0;
                    let mut is_failed = 0;
                    for d in deltas {
                        if !(1..=3).contains(&d.abs()) {
                            is_failed = 1;
                            break;
                        }
                        if has_negatives && d > 0 {
                            is_failed = 1;
                            break;
                        }
                        if !has_negatives && d < 0 {
                            is_failed = 1;
                            break;
                        }
                    }

                    1 - is_failed
                })
                .sum::<i32>()
                .signum()
        })
        .sum();
    println!("{}", reports);
}
