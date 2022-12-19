use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_11.txt").unwrap();
    let monkeys = let_monkeys_play(&input);
    let monkey_business = calculate_monkey_business(&monkeys);
    println!(
        "The level of monkey business after 20 rounds of stuff-slinging simian shenanigans is {}",
        monkey_business,
    );
}

fn let_monkeys_play(input: &str) -> Vec<Monkey> {
    let monkey_strs: Vec<&str> = input.split("\n\n").collect();
    let mut monkeys: Vec<Monkey> = monkey_strs.into_iter().map(parse_monkey).collect();
    for _n in 0..20 {
        let m2 = turn(&monkeys);
        monkeys = m2.clone();
    }
    monkeys
}

fn calculate_monkey_business(monkeys: &Vec<Monkey>) -> u64
{
    let mut result: Vec<u64> = monkeys.iter().map(|m| m.nr_inspects).collect();
    result.sort();
    result.reverse();
    result[0] * result[1]
}

fn parse_monkey(input: &str) -> Monkey
{
    let lines: Vec<&str> = input.lines().collect();
    let (_, items_str): (&str, &str) = (lines[1].split_once(":")).unwrap();
    let items: Vec<u64> = items_str.split(",").map(|x| x.trim().parse().unwrap()).collect();
    let operation = determine_operation(lines[2]);
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

fn determine_operation(line: &str) -> Operation
{
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

fn round(monkey: &Monkey, item_to_be_inspected: &u64) -> RoundResult
{
    // println!("  Monkey inspects an item with a worry level of {}", item_to_be_inspected);
    let (msg, new_value) = match monkey.operation {
        Operation::MultiplyBy(value) => (format!("multiplied by {}", value), item_to_be_inspected * value),
        Operation::MultiplyByOld => (format!("multiplied by {}", item_to_be_inspected), item_to_be_inspected * item_to_be_inspected),
        Operation::Add(value) => (format!("added by {}", value), item_to_be_inspected + value),
    };
    // println!("    Worry level is {} to {}", msg, new_value);
    let item_to_throw = new_value / 3;
    // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}", item_to_throw);
    let throw_to_monkey = if item_to_throw % monkey.divisible_by == 0 {
        // println!("    Current worry level is divisible by {}", monkey.divisible_by);
        monkey.if_true_throw_to_monkey
    } else {
        // println!("    Current worry level is not divisible by {}", monkey.divisible_by);
        monkey.if_false_throw_to_monkey
    };
    // println!("    Item with worry level {} is thrown to monkey {}.", item_to_throw, throw_to_monkey);
    RoundResult {
        throw_to_monkey,
        item_to_throw,
    }
}

fn turn(monkeys: &Vec<Monkey>) -> Vec<Monkey>
{
    let mut new_monkeys: Vec<Monkey> = monkeys.clone();
    for index in 0..new_monkeys.len() {
        println!("Monkey {}", index);
        let monkey = new_monkeys[index].clone();
        for item in &monkey.items {
            let result = round(&monkey, &item);
            new_monkeys[index].items.remove(0);
            new_monkeys[result.throw_to_monkey].items.push(result.item_to_throw);
            new_monkeys[index].nr_inspects += 1;
        }
    }
    new_monkeys
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

#[derive(PartialEq, Debug)]
struct RoundResult {
    throw_to_monkey: usize,
    item_to_throw: u64,
}

#[cfg(test)]
mod tests {
    use crate::advent_of_code_2022::advent_11::Operation::MultiplyBy;
    use super::*;

    #[test]
    fn test_parse_monkey()
    {
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
    fn test_example()
    {
        let input = fs::read_to_string("data/advent_of_code_2022/example_11.txt").unwrap();
        let monkeys = let_monkeys_play(&input);
        assert_eq!(monkeys[0].nr_inspects, 101);
        assert_eq!(monkeys[1].nr_inspects, 95);
        assert_eq!(monkeys[2].nr_inspects, 7);
        assert_eq!(monkeys[3].nr_inspects, 105);
        assert_eq!(calculate_monkey_business(&monkeys), 10605);
    }
}
