mod utils;

pub fn main() {
    let reports = parse_input();
    solve_part1(&reports);
    solve_part2(&reports);
}

fn solve_part1(reports: &Vec<Vec<i32>>) {
    let safe_reports = reports.iter().filter(|report| is_safe(report)).count();
    println!("Part 1: {}", safe_reports);
}

fn solve_part2(reports: &Vec<Vec<i32>>) {
    let safe_reports = reports.iter().filter(|report| is_safe_with_dampening(report)).count();
    println!("Part 2: {}", safe_reports);
}

fn is_safe(report: &Vec<i32>) -> bool {
    return (is_increasing(report) || is_decreasing(report)) && has_safe_increment(report);
}

fn is_safe_with_dampening(report: &Vec<i32>) -> bool {
    if is_safe(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut report_copy = report.clone();
        report_copy.remove(i);
        if is_safe(&report_copy) {
            return true;
        }
    }

    return false;
}

fn is_increasing(report: &Vec<i32>) -> bool {
    let mut last_level = report[0];
    for level in report.iter().skip(1) {
        if level < &last_level {
            return false;
        }
        last_level = *level;
    }
    return true;
}

fn is_decreasing(report: &Vec<i32>) -> bool {
    let mut last_level = report[0];
    for level in report.iter().skip(1) {
        if level > &last_level {
            return false;
        }
        last_level = *level;
    }
    return true;
}

fn has_safe_increment(report: &Vec<i32>) -> bool {
    for i in 0..report.len() - 1 {
        let diff = report[i + 1].abs_diff(report[i]);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    return true;
}

fn parse_input() -> Vec<Vec<i32>> {
    return utils::read_puzzle_input()
        .iter()
        .map(|line| line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect())
        .collect();
}
