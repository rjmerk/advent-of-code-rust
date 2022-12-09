mod advent_01; mod advent_02; mod advent_03; mod advent_04; mod advent_05;
mod advent_06; mod advent_07; mod advent_08;

use std::collections::HashMap;

pub fn init_advent_code_map_2022() -> HashMap<i32, fn()> {
    let mut result: HashMap<i32, fn() -> ()> = HashMap::new();
    result.insert(1, advent_01::solve);
    result.insert(2, advent_02::solve);
    result.insert(3, advent_03::solve);
    result.insert(4, advent_04::solve);
    result.insert(5, advent_05::solve);
    result.insert(6, advent_06::solve);
    result.insert(7, advent_07::solve);
    result.insert(8, advent_08::solve);
    result
}
