use std::{fs, io};

pub fn solve() -> Result<(), io::Error>
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_03.txt")?;
    let result: u32 = input
        .split("\n")
        .map(find_common_item)
        .map(|x| x.unwrap())
        .map(priority)
        .sum();
    println!("The sum of the priority of all common items is {}", result);
    let strings: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
    let triplets: Vec<Vec<String>> = split_into_triplets(strings);
    let result_2: u32 = triplets.into_iter()
        .map(badge)
        .map(|x| x.unwrap())
        .map(priority)
        .sum();
    println!("The sum of the priority of the badges is {}", result_2);
    Ok(())
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

fn badge(strings: Vec<String>) -> Result<String, String>
{
    for character in strings[0].chars() {
        let mut found = true;
        for string in strings[1..].iter() {
            if ! string.contains(character) {
                found = false;
            }
        }
        if found {
            return Ok(character.to_string());
        }
    }
    Err("Couldn't find the badge".to_string())
}

fn split_into_triplets<T>(input: Vec<T>) -> Vec<Vec<T>> where T: Clone {
    let mut result: Vec<Vec<T>> = vec![];
    let mut n: u32 = 0;
    let mut store: Vec<T> = vec![];
    for item in input.into_iter() {
        store.push(item);
        n += 1;
        if n == 3 {
            result.push(store.clone());
            store.clear();
            n = 0;
        }
    }
    result
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
        assert_eq!(
            find_common_item("abcdef"),
            Err("Could not find a common item")
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

    #[test]
    fn test_badge() {
        let xs = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string()
        ];
        assert_eq!(badge(xs), Ok("r".to_string()));

        let ys = vec![
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        assert_eq!(badge(ys), Ok("Z".to_string()));
    }

    #[test]
    fn test_triplets() {
        let actual = split_into_triplets(vec!["1", "2", "3", "4", "5", "6"]);
        let expected = vec![vec!["1", "2", "3"], vec!["4", "5", "6"]];
        assert_eq!(actual, expected)
    }
}