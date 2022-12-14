use std::collections::HashSet;
use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_09.txt").unwrap();
    let result = visited_positions(&input);
    println!("The number of positions the tail of the rope visits at least once is {}", result.len());
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}


fn visited_positions(input: &str) -> HashSet<Pos>
{
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut head: Pos = Pos {x:0, y:0};
    let mut tail: Pos = Pos {x:0, y:0};
    visited.insert(tail);
    for line in input.lines() {
        let (m, steps_str) = line.split_once(" ").unwrap();
        let steps: i32 = steps_str.parse().unwrap();
        let direction = match m {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0), // should not happen
        };
        for _ in 0..steps {
            let new_x = &head.x + direction.0;
            let new_y = &head.y + direction.1;
            head = Pos {x: new_x, y: new_y};
            tail = new_pos_tail(&tail, &head);
            visited.insert(tail);
            print_stuff(&visited, &tail, &head);
        }
    }
    visited
}

fn print_stuff(visited: &HashSet<Pos>, tail: &Pos, head: &Pos)
{
    for y in (0..5).rev() {
        for x in 0..6 {
            let p = &Pos{x, y};
            if p == head {
                print!("H")
            } else if p == tail {
                print!("T")
            } else if visited.contains(&Pos{x: x, y:y}) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
    println!();
}

fn new_pos_tail(tail: &Pos, head: &Pos) -> Pos
{
    let x: i32;
    let y: i32;
    x = if head.x >= tail.x + 2 {
        tail.x + 1
    } else if head.x <= tail.x - 2 {
        tail.x - 1
    } else if !touching(tail, head) {
        tail.x + move_x_diagonal(tail, head)
    } else {
        tail.x
    };
    y = if head.y >= tail.y + 2 {
        tail.y + 1
    } else if head.y <= tail.y - 2 {
        tail.y - 1
    } else if !touching(tail, head) {
        tail.y + move_y_diagonal(tail, head)
    } else {
        tail.y
    };
    Pos {x, y}
}

fn touching(tail: &Pos, head: &Pos) -> bool
{
    (tail.x - head.x).abs() <= 1 && (tail.y - head.y).abs() <= 1
}

fn move_x_diagonal(tail: &Pos, head: &Pos) -> i32
{
    if head.x > tail.x {
        1
    } else if head.x < tail.x {
        -1
    } else {
        0
    }
}

fn move_y_diagonal(tail: &Pos, head: &Pos) -> i32
{
    if head.y > tail.y {
        1
    } else if head.y < tail.y {
        -1
    } else {
        0
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test()
    {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        let actual = visited_positions(input);

        assert!(actual.contains(&Pos{x:0, y:0}));
        assert!(actual.contains(&Pos{x:1, y:0}));
        assert!(actual.contains(&Pos{x:2, y:0}));
        assert!(actual.contains(&Pos{x:3, y:0}));

        assert!(actual.contains(&Pos{x:4, y:1}));

        assert!(actual.contains(&Pos{x:1, y:2}));
        assert!(actual.contains(&Pos{x:2, y:2}));
        assert!(actual.contains(&Pos{x:3, y:2}));

        assert!(actual.contains(&Pos{x:3, y:3}));
        assert!(actual.contains(&Pos{x:4, y:3}));

        assert!(actual.contains(&Pos{x:2, y:4}));
        assert!(actual.contains(&Pos{x:3, y:4}));

        assert_eq!(actual.len(), 13);
    }
}
