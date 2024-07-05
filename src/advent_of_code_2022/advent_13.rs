use std::{fs, io};

pub fn solve() -> Result<(), io::Error> {
    let _input = fs::read_to_string("data/advent_of_code_2022/input_13.txt")?;
    Ok(())
}

enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}

fn parse(input: &str) -> Vec<Packet> {
    let mut result = Vec::new();
    result.push(Packet::Integer(32));
    let v = Vec::new();
    result.push(Packet::List(v));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let a = solve();
        let b = Some(10);
        let x = b.map(|x| x + 1);
        let input = "[37, 24, 89]";
        let parsed = parse(input);
        println!("jee {}", x.unwrap_or(0));
    }
}
