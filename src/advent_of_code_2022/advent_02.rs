use std::str::FromStr;
use std::{fs, io};

const TRY_TO_LOSE: &str = "X";
const TRY_TO_DRAW: &str = "Y";
const TRY_TO_WIN: &str = "Z";


pub fn solve() -> Result<(), io::Error>
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_02.txt")?;
    let rounds: Vec<Round> = input.split("\n").map(|s| create_round_from_str(s).unwrap()).collect();
    let score: i32 = rounds.iter().map(score_for_round).sum();
    println!("Total score of all rock-paper-scissor games is {}", score);
    println!("Star 2");
    let rounds_2: Vec<Round> = input.split("\n").map(|s| create_round_from_str_2(s).unwrap()).collect();
    let score_2: i32 = rounds_2.iter().map(score_for_round).sum();
    println!("Total score of all rock-paper-scissor games with correct Elf instructions is {}", score_2);
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl FromStr for Shape {
    type Err = ();

    fn from_str(input: &str) -> Result<Shape, Self::Err> {
        let lowercase_input: &str = &input.to_lowercase();
        match lowercase_input {
            "a" => Ok(Shape::Rock),
            "x" => Ok(Shape::Rock),
            "b" => Ok(Shape::Paper),
            "y" => Ok(Shape::Paper),
            "c" => Ok(Shape::Scissor),
            "z" => Ok(Shape::Scissor),
            _ => Err(()),
        }
    }
}

struct Round {
    opponent: Shape,
    response: Shape,
}

fn create_round_from_str(input: &str) -> Result<Round, ()> {
    let parts: Vec<Shape> = input.split(" ").map(|s| s.parse().unwrap()).collect();
    Ok(Round {
        opponent: parts[0].clone(),
        response: parts[1].clone(),
    })
}

fn create_round_from_str_2(input: &str) -> Result<Round, ()> {
    let parts: Vec<&str> = input.split(" ").collect();
    let opponent_shape: Shape = parts[0].parse().unwrap();
    let response_shape = find_response(&opponent_shape, parts[1]);
    Ok(Round {
        opponent: opponent_shape.clone(),
        response: response_shape,
    })
}

fn beats(a: &Shape, b: &Shape) -> bool {
    (a == &Shape::Rock && b == &Shape::Scissor)
        || (a == &Shape::Scissor && b == &Shape::Paper)
        || (a == &Shape::Paper && b == &Shape::Rock)
}

fn score_for_round(round: &Round) -> i32 {
    score_shape_selected(&round) + score_outcome(&round)
}

fn score_shape_selected(round: &Round) -> i32 {
    match &round.response {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn score_outcome(round: &Round) -> i32 {
    return if beats(&round.response, &round.opponent) {
        6
    } else if beats(&round.opponent, &round.response) {
        0
    } else {
        3
    };
}

fn find_response(opponent: &Shape, desired_outcome: &str) -> Shape {
    match (opponent, desired_outcome) {
        (Shape::Rock, TRY_TO_LOSE) => Shape::Scissor,
        (Shape::Rock, TRY_TO_DRAW) => Shape::Rock,
        (Shape::Rock, TRY_TO_WIN) => Shape::Paper,

        (Shape::Paper, TRY_TO_LOSE) => Shape::Rock,
        (Shape::Paper, TRY_TO_DRAW) => Shape::Paper,
        (Shape::Paper, TRY_TO_WIN) => Shape::Scissor,

        (Shape::Scissor, TRY_TO_LOSE) => Shape::Paper,
        (Shape::Scissor, TRY_TO_DRAW) => Shape::Scissor,
        (Shape::Scissor, TRY_TO_WIN) => Shape::Rock,
        _ => Shape::Rock, // should not happen
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_1() {
        let actual: Shape = "a".parse().unwrap();
        assert_eq!(actual, Shape::Rock);
    }

    #[test]
    fn test_create_round_from_str() {
        let actual: Round = create_round_from_str("A Y").unwrap();
        assert_eq!(actual.opponent, Shape::Rock);
        assert_eq!(actual.response, Shape::Paper);
    }

    #[test]
    fn test_score_for_round_1() {
        let round = Round {
            opponent: Shape::Rock,
            response: Shape::Paper,
        };
        assert_eq!(score_for_round(&round), 8);
    }

    #[test]
    fn test_score_for_round_2() {
        let round = Round {
            opponent: Shape::Paper,
            response: Shape::Rock,
        };
        assert_eq!(score_for_round(&round), 1);
    }

    #[test]
    fn test_score_for_round_3() {
        let round = Round {
            opponent: Shape::Scissor,
            response: Shape::Scissor,
        };
        assert_eq!(score_for_round(&round), 6);
    }
}
