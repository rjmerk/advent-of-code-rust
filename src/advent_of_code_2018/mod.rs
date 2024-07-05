mod advent_24;
mod advent_25;

use std::collections::HashMap;

pub fn init_advent_code_map() -> HashMap<i32, fn()> {
    let mut result: HashMap<i32, fn() -> ()> = HashMap::new();
    result.insert(25, advent_25::solve);
    result.insert(24, advent_24::solve);
    result
}
