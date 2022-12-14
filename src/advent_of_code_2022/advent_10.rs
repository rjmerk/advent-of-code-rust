use std::collections::HashMap;
use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_10.txt").unwrap();
    let instructions = input.lines().collect();
    let actual = register_values(&instructions);
    println!("{}", answer_part_1(&actual))

}

fn register_values(instructions: &Vec<&str>) -> Vec<i32>
{
    let mut result: Vec<i32> = vec![1];
    let mut cycle: usize = 1;
    print!("#");
    for instruction in instructions.iter() {
        if *instruction == "noop" {
            result.push(result[cycle - 1]);
            print_ja(&cycle, result[cycle]);
            cycle += 1;

        } else {
            let (_add_x, value_str) = instruction.split_once(" ").unwrap();
            let value: i32 = value_str.parse().unwrap();
            result.push(result[cycle - 1]);
            print_ja(&cycle, result[cycle]);
            cycle += 1;

            result.push(result[cycle - 1] + value);
            print_ja(&cycle, result[cycle]);
            cycle += 1;

        }
    }

    result
}

fn print_ja(cycle: &usize, x: i32) {
    let x_pos = (cycle % 40) as i32;
    if (x_pos - x).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }

    if x_pos == 39 {
        println!()
    }

}

fn answer_part_1(values: &Vec<i32>) -> i32
{
    20 * values[20-1] + 60 * values[60-1] + 100 * values[100-1] + 140 * values[140-1]
    + 180 * values[180-1] + 220 * values[220-1]
}


#[cfg(test)]
mod tests
{
    use itertools::Itertools;
    use super::*;

    #[test]
    fn test()
    {
        let instructions = vec!["noop", "addx 3", "addx -5"];
        let actual = register_values(&instructions);
        assert_eq!(actual[0], 1);
        assert_eq!(actual[1], 1);
        assert_eq!(actual[2], 1);
        assert_eq!(actual[3], 4);
        assert_eq!(actual[4], 4);
        assert_eq!(actual[5], -1);
    }

    #[test]
    fn test_2()
    {
        let input = fs::read_to_string("data/advent_of_code_2022/example_10.txt").unwrap();
        let instructions = input.lines().collect();
        let actual = register_values(&instructions);
        assert_eq!(actual[20-1], 21);
        assert_eq!(actual[60-1], 19);
        assert_eq!(actual[100-1], 18);
        assert_eq!(actual[140-1], 21);
        assert_eq!(actual[180-1], 16);
        assert_eq!(actual[220-1], 18);

        assert_eq!(answer_part_1(&actual), 13140);
    }
}
