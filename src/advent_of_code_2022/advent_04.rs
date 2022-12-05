use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_04.txt").unwrap();
    let result = input.split("\n").map(assignment_from_str).map(fully_contains).filter(|x| *x).count();
    println!("Number of fully contained sector assignments: {}", result);
}

struct Assignment {
    start_1: u32,
    end_1: u32,
    start_2: u32,
    end_2: u32,
}

fn assignment_from_str(s: &str) -> Assignment {
    let parts: Vec<&str> = s.split(",").collect();
    let ranges_part_1: Vec<u32> = parts[0].split("-").map(|x| x.parse().unwrap()).collect();
    let ranges_part_2: Vec<u32> = parts[1].split("-").map(|x| x.parse().unwrap()).collect();
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
}