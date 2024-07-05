use std::{fs, io};

pub fn solve() -> Result<(), io::Error> {
    let input = fs::read_to_string("data/advent_of_code_2022/input_11.txt").unwrap();
    let result_part_1 = solve_x(&input, true, 20);
    println!(
        "The level of monkey business after 20 rounds of stuff-slinging simian shenanigans is {}",
        result_part_1,
    );
    let result_part_2 = solve_x(&input, false, 10000);
    println!(
        "The level of monkey business after 10000 rounds is {}",
        result_part_2,
    );
    Ok(())
}

fn solve_x(input: &str, with_worry_update: bool, nr_round: i32) -> u64 {
    let monkey_strs: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = monkey_strs.into_iter().map(parse_monkey).collect();
    let common_divider: u64 = monkeys.iter().map(|monkey| monkey.divisible_by).product();

    for _n in 0..nr_round {
        for m in 0..monkeys.len() {
            let monkey = &mut monkeys[m];
            let mut items_to_throw: Vec<(usize, u64)> = vec![];
            while monkey.items.len() > 0 {
                let (target_monkey, item) = execute_round(&with_worry_update, monkey);
                items_to_throw.push((target_monkey, item % common_divider));
            }
            for (target_monkey, item) in items_to_throw {
                monkeys[target_monkey].items.push(item);
            }
        }
    }
    calculate_monkey_business(&monkeys)
}

fn execute_round(with_worry_update: &bool, monkey: &mut Monkey) -> (usize, u64) {
    let item = monkey.items.remove(0);
    let mut item_updated = match monkey.operation {
        Operation::MultiplyBy(value) => item * value,
        Operation::MultiplyByOld => item * item,
        Operation::Add(value) => item + value,
    };
    if *with_worry_update {
        item_updated = item_updated / 3;
    }
    monkey.nr_inspects += 1;
    let target_monkey = match item_updated % monkey.divisible_by {
        0 => monkey.if_true_throw_to_monkey,
        _ => monkey.if_false_throw_to_monkey,
    };
    (target_monkey, item_updated)
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> u64 {
    let mut result: Vec<u64> = monkeys.iter().map(|m| m.nr_inspects).collect();
    result.sort();
    result.reverse();
    result[0] * result[1]
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.lines().collect();
    let (_, items_str): (&str, &str) = (lines[1].split_once(":")).unwrap();
    let items: Vec<u64> = items_str
        .split(",")
        .map(|x| x.trim().parse().unwrap())
        .collect();
    let operation = parse_operation(lines[2]);
    let (_, divisible_str) = (lines[3].split_once("by")).unwrap();
    let divisible_by: u64 = divisible_str.trim().parse().unwrap();

    let true_str = &lines[4]["    If true: throw to monkey ".len()..];
    let false_str = &lines[5]["    If false: throw to monkey ".len()..];
    Monkey {
        items,
        operation,
        divisible_by,
        if_true_throw_to_monkey: true_str.parse().unwrap(),
        if_false_throw_to_monkey: false_str.parse().unwrap(),
        nr_inspects: 0,
    }
}

fn parse_operation(line: &str) -> Operation {
    let (_, op_str) = (line.split_once("old")).unwrap();

    if &op_str[3..] == "old" {
        return Operation::MultiplyByOld;
    }
    let value: u64 = op_str[3..].parse().unwrap();
    return if op_str.trim().starts_with("*") {
        Operation::MultiplyBy(value)
    } else if op_str.trim().starts_with("+") {
        Operation::Add(value)
    } else {
        Operation::Add(0) // should not happen
    };
}

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    MultiplyBy(u64),
    MultiplyByOld,
    Add(u64),
}

#[derive(PartialEq, Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisible_by: u64,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
    nr_inspects: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::advent_of_code_2022::advent_11::Operation::MultiplyBy;

    #[test]
    fn test_parse_monkey() {
        let input = "Monkey 0:\n  Starting items: 79, 98\n  Operation: new = old * 19\n  Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3\n";
        let actual = parse_monkey(input);
        let expected = Monkey {
            items: vec![79, 98],
            operation: MultiplyBy(19),
            divisible_by: 23,
            if_true_throw_to_monkey: 2,
            if_false_throw_to_monkey: 3,
            nr_inspects: 0,
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_example_1() {
        let input = fs::read_to_string("data/advent_of_code_2022/example_11.txt").unwrap();
        let result = solve_x(&input, true, 20);
        assert_eq!(result, 10605);
    }

    #[test]
    fn test_example_2() {
        let input = fs::read_to_string("data/advent_of_code_2022/example_11.txt").unwrap();
        let result = solve_x(&input, false, 10000);
        assert_eq!(result, 2713310158);
    }
}
