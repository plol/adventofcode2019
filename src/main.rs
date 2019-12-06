#![allow(dead_code)]

use std;

fn read_input(filename: &str) -> Vec<String> {
    let file = std::fs::File::open(filename).unwrap();
    let buf = std::io::BufReader::new(file);
    std::io::BufRead::lines(buf)
        .map(|line| line.unwrap())
        .collect()
}

fn advent_01_1(input: Vec<String>) {
    println!(
        "{}",
        input
            .iter()
            .map(|s| s.parse().unwrap())
            .map(|x: i32| x / 3 - 2)
            .sum::<i32>()
    );
}

fn advent_01_2(input: Vec<String>) {
    let mut result: i32 = 0;
    for x in input.iter().map(|s| s.parse::<i32>().unwrap()) {
        let mut j = x / 3 - 2;
        while j > 0 {
            result += j;
            j = j / 3 - 2;
        }
    }
    println!("{}", result);
}

fn advent_02_1(input: Vec<String>) {
    let mut mem: Vec<usize> = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();

    mem[1] = 12;
    mem[2] = 2;
    println!("0 {:?}", mem);

    let mut pc = 0;
    let mut running = true;
    while running {
        let instr = mem[pc];
        let output_pos = mem[pc + 3];
        match instr {
            1 => mem[output_pos] = mem[mem[pc + 1]] + mem[mem[pc + 2]],
            2 => mem[output_pos] = mem[mem[pc + 1]] * mem[mem[pc + 2]],
            99 => running = false,
            _ => panic!("no."),
        }
        pc += 4;
        println!("{} {:?}", pc, mem);
    }

    println!("{:?}", mem);
}

fn advent_02_2(input: Vec<String>) {
    let initial_mem: Vec<usize> = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut mem = initial_mem.clone();

            mem[1] = noun;
            mem[2] = verb;

            let mut pc = 0;
            let mut running = true;
            while running {
                let instr = mem[pc];
                let output_pos = mem[pc + 3];
                match instr {
                    1 => mem[output_pos] = mem[mem[pc + 1]] + mem[mem[pc + 2]],
                    2 => mem[output_pos] = mem[mem[pc + 1]] * mem[mem[pc + 2]],
                    99 => running = false,
                    _ => panic!("no."),
                }
                pc += 4;
            }
            if mem[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                return;
            }
        }
    }
}

mod advent_03 {

    pub type Point = (i32, i32);
    pub type Offset = (i32, i32);
    pub type LineSegment = (Point, Offset);
    pub type LineSegment2 = (Point, Offset, i32);

    pub fn parse_wire(input: &str) -> Vec<Offset> {
        input
            .split(",")
            .map(|instr| {
                let (d, ns) = instr.split_at(1);
                let n: i32 = ns.parse().unwrap();
                match d {
                    "R" => (n, 0),
                    "D" => (0, -n),
                    "L" => (-n, 0),
                    "U" => (0, n),
                    _ => panic!("no"),
                }
            })
            .collect()
    }

    pub fn wire_to_line_segments(wire: &Vec<Offset>) -> Vec<LineSegment> {
        let mut result: Vec<LineSegment> = vec![];

        let mut x = 0;
        let mut y = 0;

        for w in wire {
            result.push(((x, y), *w));
            x += w.0;
            y += w.1;
        }

        result
    }

    pub fn wire_to_line_segments2(wire: &Vec<Offset>) -> Vec<LineSegment2> {
        let mut result: Vec<LineSegment2> = vec![];

        let mut x = 0;
        let mut y = 0;
        let mut d = 0;

        for w in wire {
            result.push(((x, y), *w, d));
            x += w.0;
            y += w.1;
            d += w.0.abs() + w.1.abs()
        }

        result
    }

    pub fn is_between(x: i32, x1: i32, x2: i32) -> bool {
        let k1 = x1.min(x2);
        let k2 = x1.max(x2);
        k1 <= x && x <= k2
    }

    pub fn intersection(s1: &LineSegment, s2: &LineSegment) -> Option<Point> {
        let ((x1, y1), (dx1, dy1)) = s1;
        let ((x2, y2), (dx2, dy2)) = s2;

        if is_between(*x2, *x1, x1 + dx1) && is_between(*y1, *y2, y2 + dy2) {
            Some((*x2, *y1))
        } else if is_between(*x1, *x2, x2 + dx2) && is_between(*y2, *y1, y1 + dy1) {
            Some((*x1, *y2))
        } else {
            None
        }
    }
    pub fn intersection2(s1: &LineSegment2, s2: &LineSegment2) -> Option<(Point, i32)> {
        let ((x1, y1), (dx1, dy1), d1) = s1;
        let ((x2, y2), (dx2, dy2), d2) = s2;

        let total_d = d1 + d2 + (x2 - x1).abs() + (y2 - y1).abs();
        if is_between(*x2, *x1, x1 + dx1) && is_between(*y1, *y2, y2 + dy2) {
            Some(((*x2, *y1), total_d))
        } else if is_between(*x1, *x2, x2 + dx2) && is_between(*y2, *y1, y1 + dy1) {
            Some(((*x1, *y2), total_d))
        } else {
            None
        }
    }

    pub fn find_closest_wire_crossing(wire1_string: &str, wire2_string: &str) {
        let wire1: Vec<Offset> = parse_wire(wire1_string);
        let wire2: Vec<Offset> = parse_wire(wire2_string);

        let line_segments1 = wire_to_line_segments(&wire1);
        let line_segments2 = wire_to_line_segments(&wire2);

        let mut intersections = vec![];
        for s1 in &line_segments1 {
            for s2 in &line_segments2 {
                match intersection(&s1, &s2) {
                    Some(x) => intersections.push(x),
                    None => (),
                }
            }
        }
        let closest_intersection = intersections
            .iter()
            .filter(|x| *x != &(0 as i32, 0 as i32))
            .min_by_key(|x| x.0.abs() + x.1.abs())
            .unwrap();

        println!("{:?}", closest_intersection);
        println!(
            "{:?}",
            closest_intersection.0.abs() + closest_intersection.1.abs()
        );
        println!("{:?}", intersections);
    }

    pub fn find_closest_wire_crossing2(wire1_string: &str, wire2_string: &str) {
        let wire1: Vec<Offset> = parse_wire(wire1_string);
        let wire2: Vec<Offset> = parse_wire(wire2_string);

        let line_segments1 = wire_to_line_segments2(&wire1);
        let line_segments2 = wire_to_line_segments2(&wire2);

        let mut intersections = vec![];
        for s1 in &line_segments1 {
            for s2 in &line_segments2 {
                match intersection2(&s1, &s2) {
                    Some(x) => intersections.push(x),
                    None => (),
                }
            }
        }
        let closest_intersection = intersections
            .iter()
            .filter(|x| x.0 != (0, 0))
            .min_by_key(|x| x.1)
            .unwrap();

        //println!("{:?}", line_segments1);
        //println!("{:?}", line_segments2);
        //println!("{:?}", intersections);
        println!("{:?}", closest_intersection);
        println!("{:?}", closest_intersection.1);
    }

    pub fn main1(input: Vec<String>) {
        find_closest_wire_crossing(&input[0], &input[1]);
    }
    pub fn main2(input: Vec<String>) {
        find_closest_wire_crossing2(&input[0], &input[1]);
    }
    pub fn test1() {
        find_closest_wire_crossing("R8,U5,L5,D3", "U7,R6,D4,L4");
    }
    pub fn test2() {
        find_closest_wire_crossing(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        );
    }
    pub fn test3() {
        find_closest_wire_crossing(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
    }
    pub fn test4() {
        find_closest_wire_crossing2("R8,U5,L5,D3", "U7,R6,D4,L4");
    }
    pub fn test5() {
        find_closest_wire_crossing2(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72",
            "U62,R66,U55,R34,D71,R55,D58,R83",
        );
    }
    pub fn test6() {
        find_closest_wire_crossing2(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
            "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        );
    }
}

mod advent_04 {
    pub fn stupid_1() {
        let mut answer = 0;
        for i in 347312..805915 {
            let x = format!("{:06}", i);
            let mut matches = true;

            let mut has_duplicate = false;
            let mut prev = '\0';
            for d in x.chars() {
                if d == prev {
                    has_duplicate = true;
                }
                if d < prev {
                    matches = false;
                }
                prev = d;
            }
            matches = matches && has_duplicate;

            if matches {
                answer += 1;
            }
        }
        println!("{}", answer);
    }
    pub fn stupid_2() {
        let mut answer = 0;
        for i in 347312..805915 {
            let x = format!("{:06}", i);
            let mut matches = true;

            let mut duplicates = std::collections::HashSet::new();
            let mut meh = std::collections::HashSet::new();
            let mut prev = '\0';
            for d in x.chars() {
                if d == prev {
                    if duplicates.contains(&d) {
                        meh.insert(d);
                    } else {
                        duplicates.insert(d);
                    }
                }
                if d < prev {
                    matches = false;
                }
                prev = d;
            }
            matches = matches && duplicates.difference(&meh).count() > 0;

            if matches {
                answer += 1;
            }
        }
        println!("{}", answer);
    }
}

fn main() {
    advent_04::stupid_2();
}
