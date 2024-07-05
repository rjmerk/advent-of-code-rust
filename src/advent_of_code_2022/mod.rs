mod advent_01;
mod advent_02;
mod advent_03;
mod advent_04;
mod advent_05;
mod advent_06;
mod advent_07;
mod advent_08;
mod advent_09;
mod advent_10;
mod advent_11;
mod advent_12;
mod advent_13;

use std::collections::HashMap;
use std::io;

pub fn init_advent_code_map_2022() -> HashMap<i32, fn() -> Result<(), io::Error>> {
    let mut result: HashMap<i32, fn() -> Result<(), io::Error>> = HashMap::new();
    result.insert(1, advent_01::solve);
    result.insert(2, advent_02::solve);
    result.insert(3, advent_03::solve);
    result.insert(4, advent_04::solve);
    result.insert(5, advent_05::solve);
    result.insert(6, advent_06::solve);
    result.insert(7, advent_07::solve);
    result.insert(8, advent_08::solve);
    result.insert(9, advent_09::solve);
    result.insert(10, advent_10::solve);
    result.insert(11, advent_11::solve);
    result.insert(12, advent_12::solve);
    result.insert(13, advent_13::solve);
    result
}
