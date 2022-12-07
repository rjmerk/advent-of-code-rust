use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_06.txt").unwrap();
    let result = first_marker_after_character_n(4, &input);
    println!("Nr/ of characters needed to be processed before the first start-of-packet marker after 4 uniques is detected is {}", result);
    let result_2 = first_marker_after_character_n(14, &input);
    println!("Nr/ of characters needed to be processed before the first start-of-packet marker after 14 uniques is detected is {}", result_2);
}

fn first_marker_after_character_n(n:usize, stream: &str) -> usize
{
    let mut last_n_chars: Vec<char> = vec![];
    for (i, c) in stream.chars().enumerate() {
        last_n_chars.push(c);
        if last_n_chars.len() <= n {
            continue;
        }
        last_n_chars.remove(0);
        if all_unique(&last_n_chars) {
            return i + 1;
        }
    }
    stream.len() // this should never happen
}

fn all_unique(chars: &Vec<char>) -> bool
{
    for (i1, c1) in chars.iter().enumerate() {
        for (i2, c2) in chars.iter().enumerate()  {
            if (i1 != i2) && (c1 == c2) {
                return false
            }
        }
    }
    true
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_first_marker_after_character_n()
    {
        assert_eq!(first_marker_after_character_n(4,"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(first_marker_after_character_n(4,"bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(first_marker_after_character_n(4,"nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(first_marker_after_character_n(4,"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(first_marker_after_character_n(4,"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
        assert_eq!(first_marker_after_character_n(14,"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(first_marker_after_character_n(14,"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);

    }

    #[test]
    fn test_all_unique()
    {
        assert_eq!(all_unique(&vec!['m', 'j', 'q', 'j']), false);
        assert_eq!(all_unique(&vec!['v', 'w', 'b', 'j']), true);
    }
}