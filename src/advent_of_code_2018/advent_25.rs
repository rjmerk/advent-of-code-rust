/*
--- Day 25: Four-Dimensional Adventure ---

The reindeer's symptoms are getting worse, and neither you nor the white-bearded man have a
solution. At least the reindeer has a warm place to rest: a small bed near where you're sitting.

As you reach down, the reindeer looks up at you, accidentally bumping a button on your wrist-mounted
device with its nose in the process - a button labeled "help".

"Hello, and welcome to the Time Travel Support Hotline! If you are lost in time and space, press 1.
If you are trapped in a time paradox, press 2. If you need help caring for a sick reindeer, press 3.
 If you--"

Beep.

A few seconds later, you hear a new voice. "Hello; please state the nature of your reindeer."
You try to describe the situation.

"Just a moment, I think I can remotely run a diagnostic scan." A beam of light projects from the
device and sweeps over the reindeer a few times.

"Okay, it looks like your reindeer is very low on magical energy; it should fully recover if we can
fix that. Let me check your timeline for a source.... Got one. There's actually a powerful source of
magical energy about 1000 years forward from you, and at roughly your position, too! It looks
like... hot chocolate? Anyway, you should be able to travel there to pick some up; just don't
forget a mug! Is there anything else I can help you with today?"

You explain that your device isn't capable of going forward in time. "I... see. That's tricky. Well,
according to this information, your device should have the necessary hardware to open a small portal
and send some hot chocolate back to you. You'll need a list of fixed points in spacetime; I'm
transmitting it to you now."

"You just need to align your device to the constellations of fixed points so that it can lock on to
the destination and open the portal. Let me look up how much hot chocolate that breed of reindeer
needs."

"It says here that your particular reindeer is-- this can't be right, it says there's only one like
that in the universe! But THAT means that you're--" You disconnect the call.

The list of fixed points in spacetime (your puzzle input) is a set of four-dimensional coordinates.
To align your device, acquire the hot chocolate, and save the reindeer, you just need to find the
number of constellations of points in the list.

Two points are in the same constellation if their manhattan distance apart is no more than 3 or if
they can form a chain of points, each a manhattan distance no more than 3 from the last, between the
two of them. (That is, if a point is close enough to a constellation, it "joins" that
constellation.) For example:

 0,0,0,0
 3,0,0,0
 0,3,0,0
 0,0,3,0
 0,0,0,3
 0,0,0,6
 9,0,0,0
12,0,0,0

In the above list, the first six points form a single constellation: 0,0,0,0 is exactly distance 3
from the next four, and the point at 0,0,0,6 is connected to the others by being 3 away from
0,0,0,3, which is already in the constellation. The bottom two points, 9,0,0,0 and 12,0,0,0 are in
a separate constellation because no point is close enough to connect them to the first
constellation. So, in the above list, the number of constellations is 2. (If a point at 6,0,0,0 were
present, it would connect 3,0,0,0 and 9,0,0,0, merging all of the points into a single giant
constellation instead.)

In this example, the number of constellations is 4:

-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0

In this one, it's 3:

1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2

Finally, in this one, it's 8:

1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2

The portly man nervously strokes his white beard. It's time to get that hot
chocolate.

How many constellations are formed by the fixed points in spacetime?
 */


pub fn solve()
{
    let v = Vec::new();
    let u = nr_constellations(&to_points(v));
    println!("{}", u);
}


fn nr_constellations(fixed_points: &Vec<Point>) -> u16
{
    let mut constellations: Vec<Constellation> = vec![];

    for &point in fixed_points {
        constellations = update_constellations(constellations, point);
    }
    constellations.len().try_into().unwrap()
}

fn update_constellations(constellations: Vec<Constellation>, point: Point) -> Vec<Constellation>
{
    let mut result = Vec::new();
    let mut found_constellation: Option<Constellation> = None;
    for constellation in constellations {
        if in_constellation(&constellation, &point) {
            found_constellation = update_found_constellation(found_constellation, constellation, point);
        } else {
            result.push(constellation);
        }
    }
    match found_constellation {
        None => result.push(Constellation{ points: vec![point]}),
        Some(constellation) => result.push(constellation)
    }
    result
}

fn update_found_constellation(
    found_constellation: Option<Constellation>,
    constellation: Constellation,
    point: Point,
) -> Option<Constellation>
{
    match found_constellation {
        None => {
            let mut new_points = constellation.points;
            new_points.push(point);
            return Some(Constellation{ points: new_points});
        },
        Some(constellation_1) => {
            let mut new_points = constellation.points;
            new_points.push(point);
            let new_found_constellations = vec![
                Constellation{ points: new_points},
                constellation_1,
            ];
            return Some(merge_constellations(new_found_constellations))
        }
    }
}


#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    t: i32,
}

fn distance(p1: &Point, p2: &Point) -> i32
{
    (p1.x - p2.x).abs()
    + (p1.y - p2.y).abs()
    + (p1.z - p2.z).abs()
    + (p1.t - p2.t).abs()
}


fn to_points(raw: Vec<i32>) -> Vec<Point> {
    let mut result = vec! [];
    for n in (0..raw.len()).step_by(4) {
        result.push(
            Point{x: raw[n], y: raw[n+1], z:raw[n+2], t:raw[n+3]},
        )
    }
    result
}

struct Constellation {
    points: Vec<Point>,
}

fn in_constellation(constellation: &Constellation, point: &Point) -> bool {
    for constellation_point in &constellation.points {
        if distance(&point, &constellation_point) <= 3 {
            return true;
        }
    }
    false
}

fn merge_constellations(constellations: Vec<Constellation>) -> Constellation {
    let mut points: Vec<Point> = vec! [];
    for constellation in constellations {
        let mut p = constellation.points.clone();
        points.append(&mut p);
    }
    Constellation {
        points
    }
}



#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_distance()
    {
        assert_eq!(
            distance(&Point{x:0, y:0, z:0, t:0}, &Point{x:3, y:0, z:0, t:0}),
            3
        )
    }
    #[test]
    fn check_if_example_0_works()
    {
        let example = vec! [
            0,0,0,0,
            3,0,0,0,
            0,3,0,0,
            0,0,3,0,
            0,0,0,3,
            0,0,0,6,
            9,0,0,0,
            12,0,0,0,
        ];
        assert_eq!(nr_constellations(&to_points(example)), 2);
    }


    #[test]
    fn check_if_example_1_works()
    {
        let example_1 = vec! [
            -1,2,2,0,
            0,0,2,-2,
            0,0,0,-2,
            -1,2,0,0,
            -2,-2,-2,2,
            3,0,2,-1,
            -1,3,2,2,
            -1,0,-1,0,
            0,2,1,-2,
            3,0,0,0,
        ];
        assert_eq!(nr_constellations(&to_points(example_1)), 4);
    }

    #[test]
    fn check_if_example_2_works()
    {
        let example_2 = vec! [
            1,-1,0,1,
            2,0,-1,0,
            3,2,-1,0,
            0,0,3,1,
            0,0,-1,-1,
            2,3,-2,0,
            -2,2,0,0,
            2,-2,0,-1,
            1,-1,0,-1,
            3,2,0,2,
        ];
        assert_eq!(nr_constellations(&to_points(example_2)), 3);
    }

    #[test]
    fn check_if_example_3_works()
    {
        let example_3 = vec! [
            1,-1,-1,-2,
            -2,-2,0,1,
            0,2,1,3,
            -2,3,-2,1,
            0,2,3,-2,
            -1,-1,1,-2,
            0,-2,-1,0,
            -2,2,3,-1,
            1,2,2,0,
            -1,-2,0,-2,
        ];
        assert_eq!(nr_constellations(&to_points(example_3)), 8);
    }

}
