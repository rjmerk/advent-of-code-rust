use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_01.txt").unwrap();
    let calories = get_calories_per_elf_sorted(input);
    println!("Highest calories: {}", calories[0]);
    let three_highest_combined: u32 = calories.iter().take(3).sum();
    println!("Highest calories of top three together: {}", three_highest_combined);
}

fn get_calories_per_elf_sorted(input: String) -> Vec<u32>
{
    let mut result: Vec<u32> = input
        .trim().to_string() // split() will create empty values if empty line at end of file
        .split("\n\n")
        .map(split_into_int_vec)
        .map(|v| v.iter().sum())
        .collect();

    result.sort();
    result.reverse();
    result
}

fn split_into_int_vec(s: &str) -> Vec<u32>
{
    s.to_string()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect()
}


#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test()
    {
        let s = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000".to_string();
        let actual = get_calories_per_elf_sorted(s);
        let expected = vec![24000, 11000, 10000, 6000, 4000];
        assert_eq!(actual, expected);
    }
}