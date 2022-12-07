use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_06.txt").unwrap();
    let result = first_marker_after_character_n(&input);
    println!("Nr/ of characters needed to be processed before the first start-of-packet marker is detected is {}", result);
}

fn first_marker_after_character_n(stream: &str) -> usize
{
    let mut last_4_chars: Vec<char> = vec![];
    for (i, c) in stream.chars().enumerate() {
        last_4_chars.push(c);
        if last_4_chars.len() <= 4 {
            continue;
        }
        last_4_chars.remove(0);
        if all_unique(&last_4_chars) {
            return i + 1;
        }
    }
    0 // thus
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
        assert_eq!(first_marker_after_character_n("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
        assert_eq!(first_marker_after_character_n("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(first_marker_after_character_n("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(first_marker_after_character_n("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(first_marker_after_character_n("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_all_unique()
    {
        assert_eq!(all_unique(&vec!['m', 'j', 'q', 'j']), false);
        assert_eq!(all_unique(&vec!['v', 'w', 'b', 'j']), true);
    }
}