use std::{fs, io};
use regex::Regex;

pub fn solve() -> Result<(), io::Error>
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_05.txt").unwrap();
    let (stack_str, instructions_str) = input.split_once("\n\n").unwrap();
    let stack = parse_stack_from_str(stack_str.lines().collect());
    let moves: Vec<Move> = instructions_str.trim().lines().map(parse_move_from_str).collect();
    let actual_stack = make_moves(&moves, stack.clone(), true);
    print!("The crates on top of each stack are ");
    for n in actual_stack {
        print!("{}", n[n.len()-1]);
    }
    println!();

    let actual_stack_2 = make_moves(&moves, stack, false);
    print!("The crates on top of each stack, using the CrateMover 9001 are ");
    for n in actual_stack_2 {
        print!("{}", n[n.len()-1]);
    }
    println!();
    Ok(())
}

fn parse_stack_from_str(input: Vec<&str>) -> Vec<Vec<char>>
{
    let mut result = vec![];
    let last_row = input[input.len() - 1];
    let nr_stacks: usize = last_row.split_whitespace().count();
    for _ in 0..nr_stacks {
        result.push(vec![]);
    }
    for line in input[0..input.len() - 1].iter() {
        for (char_index, char) in line.chars().enumerate() {
            if char.is_alphabetic() {
                let index: usize = (char_index + 2) / 4 as usize;
                result[index].insert(0, char);
            }
        }
    }
    result
}

fn parse_move_from_str(input: &str) -> Move
{
    let re = Regex::new(r"\d+").unwrap();
    let numbers: Vec<usize> = re.find_iter(input)
        .map(|digits| digits.as_str().parse().unwrap()).collect();
    Move {
        n: numbers[0],
        source_stack: numbers[1],
        destination_stack: numbers[2],
    }

}

fn make_moves(moves: &Vec<Move>, stack: Vec<Vec<char>>, use_pop:bool) -> Vec<Vec<char>>
{
    let mut result = stack.clone();
    for mv in moves {
        let source = mv.source_stack - 1;
        let dest = mv.destination_stack - 1;

        if use_pop {
            for _ in 0..mv.n {
                let x = result[source].pop().unwrap();
                result[dest].push(x);
            }
        } else {
            let split_off_point = &result[source].len() - mv.n;
            let (new_source, crates_to_be_moved) = result[source].split_at(split_off_point );
            let new_source = Vec::from(new_source);
            let mut crates_to_be_moved = Vec::from(crates_to_be_moved);
            result[source] = new_source;
            result[dest].append(&mut crates_to_be_moved);
        }

    }
    result
}

#[derive(Clone)]
struct Move {
    n: usize,
    source_stack: usize,
    destination_stack: usize,
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_parse_stack()
    {
        let example_stack: Vec<&str> = vec! [
            "    [D]",
            "[N] [C]",
            "[Z] [M] [P]",
            " 1   2   3",
        ];
        let actual = parse_stack_from_str(example_stack);
        assert_eq!(actual[0], ['Z', 'N']);
        assert_eq!(actual[1], ['M', 'C', 'D']);
        assert_eq!(actual[2], ['P']);
    }

    #[test]
    fn test_solve_stack()
    {
        let stack: Vec<Vec<char>> = vec![ vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']];
        let expected = vec![ vec!['C', 'M'], vec![], vec!['P', 'D', 'N', 'Z']];
        let mv = Move {n: 2, source_stack: 2, destination_stack: 1};
        let actual = make_moves(&vec![mv], stack, true);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_solve_stack_2()
    {
        let stack: Vec<Vec<char>> = vec![ vec![], vec!['M', 'C'], vec!['P', 'D', 'N', 'Z']];
        let expected = vec![ vec!['M', 'C'], vec![], vec!['P', 'D', 'N', 'Z']];
        let mv = Move {n: 2, source_stack: 2, destination_stack: 1};
        let actual = make_moves(&vec![mv], stack, false);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_move_from_str()
    {
        let actual = parse_move_from_str("move 1 from 7 to 4");
        assert_eq!(actual.n, 1);
        assert_eq!(actual.source_stack, 7);
        assert_eq!(actual.destination_stack, 4);
    }


}