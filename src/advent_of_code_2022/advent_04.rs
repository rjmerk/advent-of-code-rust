use std::{fs, io};

pub fn solve() -> Result<(), io::Error>
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_04.txt").unwrap();
    let assignments = input.split("\n").map(assignment_from_str);
    let result = assignments.clone().map(fully_contains).filter(|x| *x).count();
    println!("Number of fully contained sector assignments: {}", result);
    let result_2 = assignments.map(overlaps).filter(|x| *x).count();
    println!("Number of overlapping sector assignments: {}", result_2);
    Ok(())
}

struct Assignment {
    start_1: u32,
    end_1: u32,
    start_2: u32,
    end_2: u32,
}

fn assignment_from_str(s: &str) -> Assignment {
    let halves: Vec<&str> = s.split(",").collect();
    let ranges_part_1: Vec<u32> = halves[0].split("-").map(|x| x.parse().unwrap()).collect();
    let ranges_part_2: Vec<u32> = halves[1].split("-").map(|x| x.parse().unwrap()).collect();
    Assignment {
        start_1: ranges_part_1[0],
        end_1: ranges_part_1[1],
        start_2: ranges_part_2[0],
        end_2: ranges_part_2[1],
    }
}

fn fully_contains(assignment: Assignment) -> bool
{
    (assignment.start_1 >= assignment.start_2 && assignment.end_1 <= assignment.end_2)
    || (assignment.start_2 >= assignment.start_1 && assignment.end_2 <= assignment.end_1)
}

fn overlaps(assignment: Assignment) -> bool
{
    // It's easier to determine when they DONT overlap and negate that
    !(
        (assignment.end_1 < assignment.start_2)
        || (assignment.end_2 < assignment.start_1)
    )
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_assignment_from_str()
    {
        let actual = assignment_from_str("2-4,6-8");
        assert_eq!(actual.start_1, 2);
        assert_eq!(actual.end_1, 4);
        assert_eq!(actual.start_2, 6);
        assert_eq!(actual.end_2, 8);
    }

    #[test]
    fn test_fully_contains()
    {
        let a1 = assignment_from_str("2-8,3-7");
        let a2 = assignment_from_str("6-6,4-6");
        let a3 = assignment_from_str("5-7,7-9");
        assert_eq!(fully_contains(a1), true);
        assert_eq!(fully_contains(a2), true);
        assert_eq!(fully_contains(a3), false);
    }

    #[test]
    fn test_overlaps()
    {
        let a1 = assignment_from_str("5-7,7-9");
        let a2 = assignment_from_str("2-8,3-7");
        let a3 = assignment_from_str("2-6,4-8");
        let a4 = assignment_from_str("2-4,6-8");
        let a5 = assignment_from_str("2-3,4-5");
        assert_eq!(overlaps(a1), true);
        assert_eq!(overlaps(a2), true);
        assert_eq!(overlaps(a3), true);
        assert_eq!(overlaps(a4), false);
        assert_eq!(overlaps(a5), false);
    }
}
