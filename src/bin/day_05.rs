mod utils;

#[derive(Debug, Clone)]
struct Rule {
    first: i32,
    second: i32,
}

type Update = Vec<i32>;

pub fn main() {
    let (rules, updates) = parse_input();
    solve_part1(&rules, &updates);
    solve_part2(&rules, &updates);
}

fn solve_part1(rules: &Vec<Rule>, updates: &Vec<Update>) {
    let result: i32 = updates
        .iter()
        .filter(|update| does_update_match_rules(update, rules))
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Part 1: {}", result);
}

fn solve_part2(rules: &Vec<Rule>, updates: &Vec<Update>) {
    let corrected_updates = updates
        .iter()
        .filter(|update| !does_update_match_rules(update, rules))
        .map(|update| order_values_by_rules(update, rules))
        .collect::<Vec<Update>>();
    let result: i32 = corrected_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();
    println!("Part 2: {}", result);
}

fn does_update_match_rules(update: &Update, rules: &Vec<Rule>) -> bool {
    return rules.iter().all(|rule| {
        let first_opt = update.iter().position(|&x| x == rule.first);
        let second_opt = update.iter().position(|&x| x == rule.second);
        if let (Some(first_index), Some(second_index)) = (first_opt, second_opt) {
            return first_index < second_index;
        } else {
            return true;
        }
    });
}

fn order_values_by_rules(update: &Update, rules: &Vec<Rule>) -> Update {
    let mut ordered_update = update.clone();
    while !does_update_match_rules(&ordered_update, rules) {
        for i in 0..ordered_update.len() - 1 {
            for j in i + 1..ordered_update.len() {
                if let Some(_) = rules.iter().find(|rule| {
                    rule.first == ordered_update[j] && rule.second == ordered_update[i]
                }) {
                    ordered_update.swap(i, j);
                }
            }
        }
    }
    return ordered_update;
}

fn parse_input() -> (Vec<Rule>, Vec<Update>) {
    let input: Vec<String> = utils::read_puzzle_input();

    let split_index = input.iter().position(|s| s.is_empty()).unwrap();
    let (rule_lines, update_lines) = input.split_at(split_index);
    let rule_lines: Vec<&str> = rule_lines.iter().map(|s| s.as_str()).collect();
    let update_lines: Vec<&str> = update_lines.iter().skip(1).map(|s| s.as_str()).collect();

    let rules = parse_rules(&rule_lines);
    let updates = parse_updates(&update_lines);
    return (rules, updates);
}

fn parse_rules(lines: &Vec<&str>) -> Vec<Rule> {
    return lines
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split("|").collect();
            Rule {
                first: parts[0].parse::<i32>().unwrap(),
                second: parts[1].parse::<i32>().unwrap(),
            }
        })
        .collect();
}

fn parse_updates(lines: &Vec<&str>) -> Vec<Update> {
    return lines
        .iter()
        .map(|line| line.split(",").map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
}
