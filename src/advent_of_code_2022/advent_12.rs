use std::collections::HashSet;
use std::{cmp, fs};
use std::collections::VecDeque;

const A_VALUE: i32 = 'a' as i32;
const START_CHAR: char = 'S';
const FINISH_CHAR: char = 'E';
const EXAMPLE_FILENAME: &str  = "data/advent_of_code_2022/example_12.txt";

pub fn solve() {
    let input = fs::read_to_string("data/advent_of_code_2022/input_12.txt").unwrap();
    let heightmap = Heightmap::from_str(&input);
    let result = heightmap.number_of_steps_from_start_to_end();
    println!(
        "The fewest steps required to move from your current position to the \
         location that should get the best signal is {}", result);
    let result_2 = heightmap.min_nr_steps_from_any_a_to_end();
    println!("The fewest steps required to move starting from any square with \
    elevation a to the location that should get the best signal is {}", result_2);
}

#[derive(PartialEq, Debug, Hash, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(PartialEq, Debug)]
struct ActivePoint {
    point: Point,
    distance: i32,
}

struct Heightmap {
    heights: Vec<i32>,
    start_pos: Point,
    end_pos: Point,
    max_x: usize,
    max_y: usize,
}

impl Heightmap {
    fn from_str(input: &str) -> Self {
        let rows: Vec<&str> = input.split("\n").collect();

        let max_x = rows[0].len();
        let max_y = rows.len();

        let heights:  Vec<i32> = input
            .replace("\n", "")
            .replace(START_CHAR, "a")
            .replace(FINISH_CHAR, "z")
            .chars()
            .map(|c|  (c as i32) - A_VALUE)
            .collect();
        let (start_pos, end_pos) = Heightmap::find_start_and_end_pos(input, max_x);
        Self {
            heights,
            start_pos,
            end_pos,
            max_x,
            max_y,
        }
    }

    fn find_start_and_end_pos(input: &str, max_x: usize) -> (Point, Point) {
        let mut start_pos: Point = Point{x: 0, y:0};
        let mut end_pos: Point = Point{x: 0, y:0};
        for (i, c) in input.replace("\n", "").chars().enumerate() {
            if c == START_CHAR {
                start_pos = Point {x: (i % max_x) as usize, y: (i / max_x) as usize};
            } else if c == FINISH_CHAR {
                end_pos = Point {x: (i % max_x) as usize, y: (i / max_x) as usize};
            };
        }
        (start_pos, end_pos)
    }

    fn height_at(&self, x: usize, y: usize) -> i32 {
        let index = y * self.max_x + x;
        self.heights[index]
    }

    fn reachable_points_from(&self, point: &Point) -> Vec<Point> {
        let height = self.height_at(point.x, point.y);
        let mut result: Vec<Point> = Vec::new();

        let mut points_to_check: Vec<Point> = Vec::new();
        if point.x > 0 {
            points_to_check.push(Point{x: point.x-1 , y: point.y });
        }
        if point.x < self.max_x - 1 {
            points_to_check.push(Point{x: point.x+1 , y: point.y });
        }
        if point.y > 0 {
            points_to_check.push(Point{x: point.x , y: point.y-1 });
        }
        if point.y < self.max_y - 1{
            points_to_check.push(Point{x: point.x , y: point.y+1 });
        }
        for p in points_to_check {
            if height > self.height_at(p.x, p.y)
               || self.height_at(p.x, p.y) - height <= 1 {
                result.push(p);
            }
        }
        result
    }

    fn number_of_steps_from_start_to_end(&self) -> i32 {
        let mut visited_points: HashSet<Point> = HashSet::new();
        visited_points.insert(self.start_pos.clone());
        let mut active_points: VecDeque<ActivePoint>= VecDeque::new();
        for point in self.reachable_points_from(&self.start_pos) {
            active_points.push_back(ActivePoint{point, distance: 1 })
        }
        self.nr_steps(visited_points, active_points)
    }

    fn min_nr_steps_from_any_a_to_end(&self) -> i32 {
        let mut visited_points: HashSet<Point> = HashSet::new();
        let mut active_points: VecDeque<ActivePoint> = VecDeque::new();
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                if self.height_at(x, y) == 0 {
                    visited_points.insert(Point{x, y});
                    let ap = ActivePoint{point: Point{x, y}, distance: 0};
                    active_points.push_back(ap);
                }
            }
        }
        self.nr_steps(visited_points, active_points)
    }

    fn nr_steps(
        &self,
        mut visited_points: HashSet<Point>,
        mut active_points: VecDeque<ActivePoint>,
    ) -> i32 {
        let mut min_distance: i32 = i32::MAX;
        while !active_points.is_empty() {
            let ap = active_points.pop_front().unwrap();
            for point in self.reachable_points_from(&ap.point) {
                if point == self.end_pos {
                    println!("found it! {}", ap.distance + 1);
                    min_distance = cmp::min(ap.distance + 1, min_distance);
                }
                if !visited_points.contains(&point) {
                    let distance = ap.distance + 1;
                    active_points.push_back( ActivePoint{point: point.clone(), distance});
                    visited_points.insert(point);
                }
            }
        }
        min_distance
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example() {
        let input = fs::read_to_string(EXAMPLE_FILENAME)
            .unwrap();
        let hm = Heightmap::from_str(&input);

        assert_eq!(hm.start_pos, Point{x: 0, y: 0});
        assert_eq!(hm.end_pos, Point{x: 5, y: 2});

        let result = hm.number_of_steps_from_start_to_end();
        assert_eq!(result, 31);
    }

    #[test]
    fn test_reachable_points_from() {
        let input = fs::read_to_string(EXAMPLE_FILENAME)
            .unwrap();
        let hm = Heightmap::from_str(&input);
        println!("max_x {}", hm.max_x);
        println!("max_y {}", hm.max_y);

        for y in 0..hm.max_y {
            for x in 0..hm.max_x {
                let c: char = char::from_u32( (A_VALUE + hm.height_at(x,y)) as u32).unwrap();
                print!("{}",  c);
            }
            println!();
        }
        let pos = Point{x: 0, y:0};
        let expected = [
            Point{x: 1, y:0},
            Point{x: 0, y:1},
        ];
        assert_eq!(hm.reachable_points_from(&pos), expected);
    }

    #[test]
    fn test_example_2() {
        let input = fs::read_to_string(EXAMPLE_FILENAME)
            .unwrap();
        let hm = Heightmap::from_str(&input);

        assert_eq!(hm.start_pos, Point{x: 0, y: 0});
        assert_eq!(hm.end_pos, Point{x: 5, y: 2});

        let result = hm.min_nr_steps_from_any_a_to_end();
        assert_eq!(result, 29);
    }
}
