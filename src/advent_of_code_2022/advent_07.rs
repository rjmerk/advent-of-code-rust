use std::collections::HashMap;
use std::fs;

const MAX_SIZE: u64 = 100000;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_07.txt").unwrap();
    let dir_sizes = find_dir_sizes(&input);
    let result: u64 = dir_sizes.values().filter(|x| x <= && MAX_SIZE).sum();
    println!("The sum of the total sizes of those directories is {}", result);
    let result_2: u64 = smallest_folder(&dir_sizes, 70000000, 30000000);
    println!("The smallest folder size is {}", result_2);

}

fn find_dir_sizes(input: &str) -> HashMap<String, u64>
{
    let mut current_path: Vec<&str> = Vec::new();
    let mut result: HashMap<String, u64> = HashMap::new();
    for line in input.lines() {
        if line == "$ cd .." {
            current_path.pop();
        } else if line.starts_with("$ cd") {
            current_path.push(&line[4..].trim());
        } else if line.starts_with("dir ") || line == "$ ls" {
            continue;
        } else {
            // line is a file description (size + filename)
            result = update_folder_size(line, &current_path, result);
        }
    }
    result
}

fn update_folder_size(
    line: &str,
    current_path: &Vec<&str>,
    mut result: HashMap<String, u64>,
) -> HashMap<String, u64>
{
    let (size_str, _filename) = line.split_once(" ").unwrap();
    let size: u64 = size_str.parse().unwrap();
    let mut c = current_path.clone();
    for _ in 0..c.len() {
        let path_str: String = c.join("/");
        *result.entry(path_str).or_default() += size;
        c.pop();
    }
    result
}

fn smallest_folder(
    dir_sizes: &HashMap<String, u64>,
    disk_space: u64,
    space_needed_for_update: u64,
) -> u64
{
    let total_size_of_everything = dir_sizes.get("/").unwrap();
    let space_needed_to_free = total_size_of_everything - (disk_space - space_needed_for_update);

    let mut smallest_size: u64 = u64::MAX;
    let mut smallest_folder = "";
    for (path, size) in dir_sizes.iter() {
        if *size >= space_needed_to_free && *size < smallest_size {
            smallest_size = *size;
            smallest_folder = path;
        }
    }
    println!("Smallest folder is {}", smallest_folder);
    smallest_size
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_find_dir_sizes() {
        let input = fs::read_to_string("data/advent_of_code_2022/example_07.txt").unwrap();
        let actual = find_dir_sizes(&input);
        let sum_small_dir_sizes: u64 = actual.values().filter(|x| x <= && MAX_SIZE).sum();
        let mut expected = HashMap::new();
        expected.insert("//a/e".to_string(), 584);
        expected.insert("//a".to_string(), 94853);
        expected.insert("//d".to_string(), 24933642);
        expected.insert("/".to_string(), 48381165);
        assert_eq!(actual, expected);
        assert_eq!(sum_small_dir_sizes, 95437);

    }

    #[test]
    fn test_update_folder_size()
    {
        let mut current_path: Vec<&str> = vec!["/", "a", "b"];
        let mut result: HashMap<String, u64> = HashMap::new();
        let line = "123 df9f.txt";
        result = update_folder_size(line, &current_path, result);
        current_path.push("c");
        let line_2 = "1 jadhjs.jpg";
        result = update_folder_size(line_2, &current_path, result);
        let expected_big: u64 = 123 + 1;
        let expected_small: u64 = 1;
        assert_eq!(result.get("/"), Some(&expected_big));
        assert_eq!(result.get("//a"), Some(&expected_big));
        assert_eq!(result.get("//a/b"), Some(&expected_big));
        assert_eq!(result.get("//a/b/c"), Some(&expected_small));
    }

    #[test]
    fn test_smallest_folder()
    {
        let input = fs::read_to_string("data/advent_of_code_2022/example_07.txt").unwrap();
        let dir_sizes = find_dir_sizes(&input);
        assert_eq!(smallest_folder(&dir_sizes, 70000000, 30000000), 24933642);
    }
}
