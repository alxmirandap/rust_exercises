use crate::Line::*;

/*

Max points on a line:

Brute-force:
- iterate through all points of pairs, compute the line that they define, and increment the respective counter

*/

use std::collections::{HashSet, HashMap};

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
struct Point {
    pub x: i32,
    pub y: i32
}

// A line's equation is y = mx + b
// Or alternatively, for two known points, (y1 - y0) = m (x1 - x0)
// m is the slope, dy/dx. Since we are dealing with integer coordinates, 
// we have always a rational slope and can represent it by an irreducible fraction
// To compute b, we can apply any of the points to the first equation and get:
// b = y - mx
// This only works for non-vertical lines. Those have equation x = a, 
// and obviously here the affine equation does not make sense

#[derive(Clone, Debug, PartialEq, Hash, Eq)]
enum Line {
    Vertical(i32),
    Regular {dx: i32, dy: i32, b: i32}
}

impl Point {
    fn from(point: &Vec<i32>) -> Self {
        return Point {
            x: point[0],
            y: point[1]
        }
    }
}

impl Line {
    fn find_line(p1: &Point, p2: &Point) -> Self {
        let dy: i32 = p2.y - p1.y;
        let dx: i32 = p2.x - p1.x;
        
        if dx == 0 {
            return Vertical(p1.x);
        }

        let (dx, dy) = reduce_factors(dx, dy);
        let b: i32 = p1.y - (dy * p1.x / dx);

        return Regular {
            dx,
            dy,
            b
        }
    }
    
}

fn main() {   

    let points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]];
    let mut vec_points: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();

    for point in points {
        let new_point = vec![point[0], point[1]];        
        vec_points.push(new_point);
    }

    println!("Return {:?}", max_points(vec_points));
}

pub fn max_points(points: Vec<Vec<i32>>) -> i32 {
    if points.len() == 1 {
        return 1;
    }

    let number_points = points.len();
    let mut lines = HashMap::<Line, HashSet<Point>>::new();
    let mut most_frequent_line = 0;

    for i in 0..number_points {
        for j in i+1 .. number_points {
            let p1 = Point::from(&points[i]);
            let p2 = Point::from(&points[j]);

            let line = Line::find_line(&p1, &p2);
            let this_count = register_points_in_line(&mut lines, line, &p1, &p2);

            if this_count > most_frequent_line {
                most_frequent_line = this_count;
            }
        }
    }

    return most_frequent_line;
}


fn reduce_factors(a: i32, b: i32) -> (i32, i32) {
    let abs_a = if a >= 0 {a} else {-a};
    let abs_b = if b >= 0 {b} else {-b};

    let g = gcd(abs_a, abs_b);
    return (a/g, b/g);
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == b {
        return a;
    }

    if a > b {
        return gcd(b, a);
    }

    // always guaranteed that a < b:
    if a == 0 {
        return b;
    }

    if a == 1 {
        return 1;
    }

    return gcd(b % a, a);
}

fn register_points_in_line(memo: &mut HashMap<Line, HashSet<Point>>, line: Line, p1: &Point, p2: &Point) -> i32 {
    let line_set: &mut HashSet<Point>;

    if !memo.contains_key(&line) {
        memo.insert(line.clone(), HashSet::<Point>::new());
    } 
    line_set = memo.get_mut(&line).unwrap();

    line_set.insert(p1.clone());
    line_set.insert(p2.clone());

    return line_set.len() as i32;
}

#[cfg(test)]
mod unit_tests;