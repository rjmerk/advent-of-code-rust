mod advent_01; mod advent_02;

use std::collections::HashMap;

pub fn init_advent_code_map_2022() -> HashMap<i32, fn()> {
    let mut result: HashMap<i32, fn() -> ()> = HashMap::new();
    result.insert(1, advent_01::solve);
    result.insert(2, advent_02::solve);
    result
}
