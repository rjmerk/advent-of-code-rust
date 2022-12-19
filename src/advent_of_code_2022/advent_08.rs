use std::fs;

pub fn solve()
{
    let input = fs::read_to_string("data/advent_of_code_2022/input_08.txt").unwrap();
    let mut heightmap = Heightmap::from_str(input);
    let result: usize = heightmap.nr_trees_visible();
    println!("The number of trees visible from outside the map is {}", result);
    let result_2 = heightmap.max_scenic_score();
    println!("The highest scenic score possible for any tree is {}", result_2);
}


struct Heightmap {
    heights: Vec<i32>,
    visibility: Vec<bool>,
    max_x: usize,
    max_y: usize,
}


impl Heightmap {
    fn from_str(input: String) -> Self
    {
        let rows: Vec<&str> = input.split("\n").collect();
        let max_x = rows[0].len();
        let max_y = rows.len();
        let heights = input
            .replace("\n", "")
            .chars()
            .map(|c|  c.to_digit(10).unwrap() as i32)
            .collect();
        Self {
            heights,
            visibility: vec![false; max_x * max_y],
            max_x,
            max_y,
        }
    }

    fn nr_trees_visible(&mut self) -> usize
    {
        for x in 0..self.max_x {
            self.set_as_visible(x, 0);
            self.set_as_visible(x, self.max_y - 1);
        }
        for y in 0..self.max_y {
            self.set_as_visible(0, y);
            self.set_as_visible(self.max_x - 1, y);
        }

        for x in 1..self.max_x - 1 {
            for y in 1..self.max_y -1 {
                if self.is_visible(x, y) {
                    self.set_as_visible(x, y);
                }
            }
        }
        self.visibility.iter().filter(|b| **b).count()
    }

    fn is_visible(&self, x: usize, y: usize) -> bool
    {
        let height = self.height_at(x, y);
        // left
        let mut visible_left = true;
        for x1 in 0..x {
            if self.height_at(x1, y) >= height {
                visible_left = false;
                break;
            }
        }
        if visible_left { return true}
        // right
        let mut visible_right = true;
        for x1 in (x+1..self.max_x).rev() {
            if self.height_at(x1, y) >= height {
                visible_right = false;
                break;
            }
        }
        if visible_right { return true}
        // top
        let mut visible_top = true;
        for y1 in 0..y {
            if self.height_at(x, y1) >= height {
                visible_top = false;
                break;
            }
        }
        if visible_top { return true}
        // bottom
        let mut visible_bottom = true;
        for y1 in (y+1..self.max_y).rev() {
            if self.height_at(x, y1) >= height {
                visible_bottom = false;
                break;
            }
        }
        if visible_bottom { return true}

        false
    }

    fn set_as_visible(&mut self, x: usize, y: usize)
    {
        let index = y * self.max_x + x;
        self.visibility[index] = true;
    }

    fn height_at(&self, x: usize, y: usize) -> i32
    {
        let index = y * self.max_x + x;
        self.heights[index]
    }

    fn scenic_score(&self, x: usize, y: usize) -> i32
    {
        let height = self.height_at(x, y);
        self.scenic_left(x, y, height)
            * self.scenic_right(x, y, height)
            * self.scenic_top(x,y, height)
            * self.scenic_bottom(x, y, height)
    }

    fn scenic_left(&self, x: usize, y: usize, height: i32) -> i32
    {
        let mut result = 0;
        for x1 in (0..x).rev() {
            result += 1;
            if self.height_at(x1, y) >= height {
                return result
            }
        }
        result
    }

    fn scenic_right(&self, x: usize, y: usize, height: i32) -> i32
    {
        let mut result = 0;
        for x1 in x+1..self.max_x {
            result += 1;
            if self.height_at(x1, y) >= height {
                return result
            }
        }
        result
    }

    fn scenic_bottom(&self, x: usize, y: usize, height: i32) -> i32
    {
        let mut result = 0;
        for y1 in y+1..self.max_y {
            result += 1;
            if self.height_at(x, y1) >= height {
                return result
            }
        }
        result
    }

    fn scenic_top(&self, x: usize, y: usize, height: i32) -> i32
    {
        let mut result = 0;
        for y1 in (0..y).rev() {
            result += 1;
            if self.height_at(x, y1) >= height {
                return result
            }
        }
        result
    }

    fn max_scenic_score(&self) -> i32
    {
        let mut result = 0;
        for x in 0..self.max_x {
            for y in 0..self.max_y {
                let score = self.scenic_score(x, y);
                if score > result {
                    result = score
                }
            }
        }
        result
    }

}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_nr_trees_visible() {
        let mut heightmap = Heightmap::from_str(
            "30373\n25512\n65332\n33549\n35390".to_string()
        );
        assert_eq!(heightmap.max_x, 5);
        assert_eq!(heightmap.max_y, 5);
        assert_eq!(heightmap.height_at(4, 3), 9);
        let actual = heightmap.nr_trees_visible();

        assert!(heightmap.is_visible(1, 1));
        assert!(heightmap.is_visible(2, 1));
        assert_eq!(heightmap.is_visible(3, 1), false);

        assert!(heightmap.is_visible(1, 2));
        assert_eq!(heightmap.is_visible(2, 2), false);
        assert!(heightmap.is_visible(3, 2));

        assert_eq!(heightmap.is_visible(1, 3), false);
        assert!(heightmap.is_visible(2, 3));
        assert_eq!(heightmap.is_visible(3, 3), false);

        for y in 0..5 {
            for x in 0..5 {
                let c =  if heightmap.is_visible(x, y) {"1"} else {"0"};
                print!{"{}", c}
            }
            println!();
        }
        println!();
        for y in 0..5 {
            for x in 0..5 {
                print!{"{}", heightmap.height_at(x,y)};
            }
            println!();
        }

        assert_eq!(actual, 21);
    }

    #[test]
    fn test_scenery_score() {
        let heightmap = Heightmap::from_str(
            "30373\n25512\n65332\n33549\n35390".to_string()
        );
        let height = heightmap.height_at(2, 1);
        assert_eq!(heightmap.scenic_top(2, 1, height), 1, "top wrong: expected 1");
        assert_eq!(heightmap.scenic_left(2, 1, height), 1);
        assert_eq!(heightmap.scenic_right(2, 1, height), 2);
        assert_eq!(heightmap.scenic_bottom(2, 1, height), 2);

        assert_eq!(heightmap.scenic_score(2,  1), 4);
        assert_eq!(heightmap.scenic_score(2,  3), 8);
        assert_eq!(heightmap.max_scenic_score(), 8);
    }
}
