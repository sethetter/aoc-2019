use std::fs;

fn main() {
    let raw_contents = fs::read_to_string("input.txt").expect("Error reading the file.");
    let contents: Vec<&str> = raw_contents.trim().split("\n").collect(); // get rid of trailing \n

    let w1: Vec<(isize, isize)> = parse_path(contents[0]);
    let w2: Vec<(isize, isize)> = parse_path(contents[1]);
    println!("Paths parsed.");

    let intersections = find_intersections(w1, w2);
    println!("Found intersections.");

    let (x, y) = closest_intersection(intersections);
    println!("Part 1: ({},{}) {}", x, y, x.abs() + y.abs());
}

fn parse_path(path_str: &str) -> Vec<(isize, isize)> {
    path_str.split(",").fold(vec![(0, 0)], add_steps_to_path)
}

#[test]
fn test_parse_path() {
    assert_eq!(
        parse_path("U3,L4,D4"),
        vec![(0, 0), (0, 1), (0, 2), (0, 3), (-1, 3), (-2, 3), (-3, 3), (-4, 3), (-4, 2), (-4, 1), (-4, 0), (-4, -1)]
    )
}

fn add_steps_to_path(path: Vec<(isize, isize)>, step: &str) -> Vec<(isize, isize)> {
    let (dir, n) = parse_step(step);
    let &(last_x, last_y) = path.last().unwrap();

    // determine if we are going backwards or forwards
    let new_steps: Vec<(isize, isize)> = match dir {
        Direction::Up => (last_y + 1..=last_y + n).map(|y| (last_x, y)).collect(),
        Direction::Down => (last_y - n..=last_y - 1).map(|y| (last_x, y)).rev().collect(),
        Direction::Left => (last_x - n..=last_x - 1).map(|x| (x, last_y)).rev().collect(),
        Direction::Right => (last_x + 1..=last_x + n).map(|x| (x, last_y)).collect(),
        Direction::Invalid => vec![],
    };

    let mut new_path = path.clone();
    new_path.append(&mut new_steps.clone());
    new_path
}

#[test]
fn test_add_steps_to_path() {
    assert_eq!(
        add_steps_to_path(vec![(0, 0), (-1, 0)], "U2"),
        vec![(0, 0), (-1, 0), (-1, 1), (-1, 2)]
    )
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

fn parse_step(step: &str) -> (Direction, isize) {
    let mut chars = step.chars();

    let dir = chars.next();
    let n = chars.as_str().parse::<isize>();

    match (dir, n) {
        (Some('U'), Ok(x)) => (Direction::Up, x),
        (Some('D'), Ok(x)) => (Direction::Down, x),
        (Some('L'), Ok(x)) => (Direction::Left, x),
        (Some('R'), Ok(x)) => (Direction::Right, x),

        // Error cases
        (Some(_), _) => (Direction::Invalid, 0),
        (_, Err(_)) => (Direction::Invalid, 0),
        (None, _) => (Direction::Invalid, 0),
    }
}

#[test]
fn test_parse_step() {
    assert_eq!(parse_step("U2"), (Direction::Up, 2));
    assert_eq!(parse_step("L3"), (Direction::Left, 3));
    assert_eq!(parse_step("R31"), (Direction::Right, 31));
    assert_eq!(parse_step("D99"), (Direction::Down, 99));
}

fn find_intersections(p1: Vec<(isize, isize)>, p2: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    p1.into_iter().filter(|p| p2.contains(p)).collect()
}

#[test]
fn test_find_intersections() {
    let p1 = vec![(0,0), (0,1), (1,1), (1,2)];
    let p2 = vec![(0,0), (1,0), (1,1), (2,1)];
    assert_eq!(find_intersections(p1, p2), vec![(0,0), (1,1)]);
}

fn closest_intersection(ints: Vec<(isize, isize)>) -> (isize, isize) {
    // skip the (0, 0) first intersection
    ints.into_iter().skip(1).fold((0, 0), |(prev_x, prev_y), (x, y)| {
        if prev_x == 0 && prev_y == 0 { return (x, y); }
        let prev_dist = prev_x.abs() + prev_y.abs();
        let dist = x.abs() + y.abs();
        if dist < prev_dist { (x, y) } else { (prev_x, prev_y) }
    })
}

#[test]
fn test_closest_intersection() {
    assert_eq!(closest_intersection(vec![(0,0), (4,5), (3,2), (5,10)]), (3,2));
}
