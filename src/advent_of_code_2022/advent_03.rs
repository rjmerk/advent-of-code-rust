use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_03.txt").unwrap();
    let result: u32 = input
        .split("\n")
        .map(find_common_item)
        .map(|x| x.unwrap())
        .map(priority)
        .sum();
    println!("The sum of the priority of all common items is {}", result)

}


fn find_common_item(s: &str) -> Result<String, &str>
{
    let i = s.len() / 2;
    let part_1 = &s[0 .. i];
    let part_2 = &s[i .. ];
    for character in part_1.chars() {
        if part_2.contains(character) {
            return Ok(character.to_string())
        }
    }
    Err("Could not find a common item")
}

fn priority(s: String) -> u32 {
    let x = s.chars().next().unwrap();
    return if x.is_uppercase() {
        x as u32 - 38
    } else {
        x as u32 - 96
    }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_find_common_item()
    {
        assert_eq!(
            find_common_item("vJrwpWtwJgWrhcsFMMfFFhFp"),
            Ok("p".to_string())
        );
        assert_eq!(
            find_common_item("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
            Ok("L".to_string())
        );
        assert_eq!(
            find_common_item("PmmdzqPrVvPwwTWBwg"),
            Ok("P".to_string())
        );
        assert_eq!(
            find_common_item("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
            Ok("v".to_string())
        );
        assert_eq!(
            find_common_item("ttgJtRGJQctTZtZT"),
            Ok("t".to_string())
        );
        assert_eq!(
            find_common_item("CrZsJsPPZsGzwwsLwLmpwMDw"),
            Ok("s".to_string())
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(priority("p".to_string()), 16);
        assert_eq!(priority("L".to_string()), 38);
        assert_eq!(priority("P".to_string()), 42);
        assert_eq!(priority("v".to_string()), 22);
        assert_eq!(priority("t".to_string()), 20);
        assert_eq!(priority("s".to_string()), 19);
    }
}