// use std::collections::HashSet;
use std::fs;
// use itertools::Position;

const A_VALUE: i32 = 'a' as i32;
const START_CHAR: char = 'S';
const FINISH_CHAR: char = 'E';

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_12.txt").unwrap();
    let heightmap = Heightmap::from_str(&input);
    let result = heightmap.number_of_steps();
    println!(
        "The fewest steps required to move from your current position to the \
         location that should get the best signal is {}", result);

}



#[derive(PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

struct Heightmap {
    heights: Vec<i32>,
    start_pos: Point,
    end_pos: Point,
    max_x: usize,
    max_y: usize,
}

impl Heightmap {
    fn from_str(input: &str) -> Self
    {
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

        // let mut heights : Vec<i32> = Vec::new();
        // for x in 0..max_x +1 {
        //     for y in 0..max_y +1 {
        //         if x == 0 || x == max_x || y == 0 || y == max_y {
        //             heights.push(26);
        //         } else {
        //             let index = y * max_x + x;
        //             heights.push(old_heights[index]);
        //         }
        //     }
        // }
        let (start_pos, end_pos) = Heightmap::find_start_and_end_pos(input);
        Self {
            heights,
            start_pos,
            end_pos,
            max_x,
            max_y,
        }
    }

    fn find_start_and_end_pos(input: &str) -> (Point, Point)
    {
        let mut start_pos: Point = Point{x: 0, y:0};
        let mut end_pos: Point = Point{x: 0, y:0};
        for (i, c) in input.replace("\n", "").chars().enumerate() {
            if c == START_CHAR {
                start_pos = Point {x: (i % 8) as usize, y: (i / 8) as usize};
            } else if c == FINISH_CHAR {
                end_pos = Point {x: (i % 8) as usize, y: (i / 8) as usize};
            };
        }
        (start_pos, end_pos)
    }


    fn height_at(&self, x: usize, y: usize) -> i32
    {
        let index = y * self.max_x + x;
        self.heights[index]
    }

    fn reachable_points_from(&self, point: &Point) -> Vec<Point>
    {
        let height = self.height_at(point.x, point.y);
        let mut result: Vec<Point> = Vec::new();

        let mut points_to_check: Vec<Point> = Vec::new();
        if point.x > 0 {
            points_to_check.push(Point{x: point.x-1 , y: point.y });
        }
        if point.x < self.max_x {
            points_to_check.push(Point{x: point.x+1 , y: point.y });
        }
        if point.y > 0 {
            points_to_check.push(Point{x: point.x , y: point.y-1 });
        }
        if point.y < self.max_y {
            points_to_check.push(Point{x: point.x , y: point.y+1 });
        }
        for p in points_to_check {
            if i32::abs(height - self.height_at(p.x, p.y)) <= 1 {
                result.push(p);
            }
        }
        result
    }

    fn number_of_steps(&self) -> u32 {
        let mut paths: Vec<Vec<Point>> = Vec::new();
        let mut finished_paths: Vec<Vec<Point>> = Vec::new();
        for pos in self.reachable_points_from(&self.start_pos) {
            paths.push(Vec!(pos))
        }
        loop {
            let mut new_paths: Vec<Vec<Point>> = Vec::new();
            for mut path in paths {
                let last_point = path.last().unwrap();
                let new_points = self.reachable_points_from(last_point);
                for new_point in new_points {
                    let new_path = path.copy();
                    new_path.push(new_point);
                    new_paths.push(new_path);
                }
            }
            break;
        }
        3
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_example()
    {
        let input = fs::read_to_string("data/advent_of_code_2022/example_12.txt")
            .unwrap();
        let hm = Heightmap::from_str(&input);

        assert_eq!(hm.start_pos, Point{x: 0, y: 0});
        assert_eq!(hm.end_pos, Point{x: 5, y: 2});

        let result = hm.number_of_steps();
        assert_eq!(result, 31);
    }

    #[test]
    fn test_reachable_points_from()
    {
        let input = fs::read_to_string("data/advent_of_code_2022/example_12.txt")
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
        assert_eq!(hm.reachable_points_from(&pos), expected)
    }
}
q