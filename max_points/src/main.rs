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
    
        let (dx, dy) = reduce_factors(dx, dy);

        if dx == 0 {
            return Vertical(p1.x);
        }

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
mod tests {
    use super::*;

    fn create_test_vector(points: &[[i32;2]]) -> Vec<Vec<i32>> {
        let mut vec_points: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    
        for point in points {
            let new_point = vec![point[0], point[1]];        
            vec_points.push(new_point);
        }

        return vec_points;
    }

    #[test]
    fn testcase_1() {
        let points = [[1,1],[2,2],[3,3]];
        let vec_points = create_test_vector(&points);
    
        assert_eq!(max_points(vec_points), 3);    
    }    

    #[test]
    fn testcase_2() {
        let points = [[1,1],[3,2],[5,3],[4,1],[2,3],[1,4]];
        let vec_points = create_test_vector(&points);
    
        assert_eq!(max_points(vec_points), 4);    
    }    

    #[test]
    fn testcase_3() {
        let points = [[0,0]];
        let vec_points = create_test_vector(&points);
    
        assert_eq!(max_points(vec_points), 1);    
    }    

    #[test]
    fn testcase_4() {
        let points = [[0,0],[4,5],[7,8],[8,9],[5,6],[3,4],[1,1]];
        let vec_points = create_test_vector(&points);
    
        assert_eq!(max_points(vec_points), 5);
    }    

    #[test]
    fn testcase_5() {
        let points = [[7,3],[19,19],[-16,3],[13,17],[-18,1],[-18,-17],[13,-3],[3,7],[-11,12],[7,19],[19,-12],[20,-18],[-16,-15],[-10,-15],[-16,-18],[-14,-1],[18,10],[-13,8],[7,-5],[-4,-9],[-11,2],[-9,-9],[-5,-16],[10,14],[-3,4],[1,-20],[2,16],[0,14],[-14,5],[15,-11],[3,11],[11,-10],[-1,-7],[16,7],[1,-11],[-8,-3],[1,-6],[19,7],[3,6],[-1,-2],[7,-3],[-6,-8],[7,1],[-15,12],[-17,9],[19,-9],[1,0],[9,-10],[6,20],[-12,-4],[-16,-17],[14,3],[0,-1],[-18,9],[-15,15],[-3,-15],[-5,20],[15,-14],[9,-17],[10,-14],[-7,-11],[14,9],[1,-1],[15,12],[-5,-1],[-17,-5],[15,-2],[-12,11],[19,-18],[8,7],[-5,-3],[-17,-1],[-18,13],[15,-3],[4,18],[-14,-15],[15,8],[-18,-12],[-15,19],[-9,16],[-9,14],[-12,-14],[-2,-20],[-3,-13],[10,-7],[-2,-10],[9,10],[-1,7],[-17,-6],[-15,20],[5,-17],[6,-6],[-11,-8]];
        let vec_points = create_test_vector(&points);
    
        assert_eq!(max_points(vec_points), 6);
    }    


    #[test]
    fn testcase_6() {
        let points = [[10,2],[-15,3],[-15,-7],[0,2],[-15,10],[-15,-15]];
        let vec_points = create_test_vector(&points);
    
        assert_eq!(max_points(vec_points), 4);
    }    
}
