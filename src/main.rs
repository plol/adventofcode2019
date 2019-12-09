#![allow(dead_code)]

use std;

pub mod intcode;

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

mod advent_05 {

    pub fn get_value(
        mem: &Vec<i64>,
        instr: &str,
        instr_number: usize,
        pointer_or_value: i64,
    ) -> i64 {
        match instr.chars().nth(instr.len() - 2 - instr_number).unwrap() {
            '0' => mem[pointer_or_value as usize],
            '1' => pointer_or_value,
            _ => panic!("no"),
        }
    }

    pub fn exec<F, G>(initial_mem: Vec<i64>, input: F, output: G) -> Vec<i64>
    where
        F: Fn() -> i64,
        G: Fn(i64),
    {
        let mut mem = initial_mem.clone();
        let mut pc = 0;
        let mut running = true;
        while running {
            let instr = format!("{:09}", mem[pc]);
            let output_pos = mem[pc + 3] as usize;
            match &instr[instr.len() - 2..instr.len()] {
                "01" => {
                    mem[output_pos] = get_value(&mem, &instr, 1, mem[pc + 1])
                        + get_value(&mem, &instr, 2, mem[pc + 2]);
                    pc += 4;
                }
                "02" => {
                    mem[output_pos] = get_value(&mem, &instr, 1, mem[pc + 1])
                        * get_value(&mem, &instr, 2, mem[pc + 2]);
                    pc += 4;
                }
                "03" => {
                    mem[output_pos] = input();
                    pc += 2;
                }
                "04" => {
                    output(get_value(&mem, &instr, 1, mem[pc + 1]));
                    pc += 2;
                }
                "05" => {
                    let a = get_value(&mem, &instr, 1, mem[pc + 1]);
                    let b = get_value(&mem, &instr, 2, mem[pc + 2]);
                    if a != 0 {
                        pc = b as usize;
                    } else {
                        pc += 3;
                    }
                }
                "06" => {
                    let a = get_value(&mem, &instr, 1, mem[pc + 1]);
                    let b = get_value(&mem, &instr, 2, mem[pc + 2]);
                    if a == 0 {
                        pc = b as usize;
                    } else {
                        pc += 3;
                    }
                }
                "07" => {
                    if get_value(&mem, &instr, 1, mem[pc + 1])
                        < get_value(&mem, &instr, 2, mem[pc + 2])
                    {
                        mem[output_pos] = 1;
                    } else {
                        mem[output_pos] = 0;
                    }
                    pc += 4;
                }
                "08" => {
                    if get_value(&mem, &instr, 1, mem[pc + 1])
                        == get_value(&mem, &instr, 2, mem[pc + 2])
                    {
                        mem[output_pos] = 1;
                    } else {
                        mem[output_pos] = 0;
                    }
                    pc += 4;
                }
                "99" => running = false,
                _ => panic!("no."),
            }
        }
        mem
    }

    pub fn test(input: String) {
        let mut initial_mem: Vec<i64> = input
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        initial_mem.push(0);

        let mem = exec(initial_mem, || 8, |x| println!("output {}", x));
        println!("{:?}", mem);
    }
    pub fn main(input: Vec<String>) {
        let initial_mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();

        let mem = exec(initial_mem, || 1, |x| println!("output {}", x));
        println!("{:?}", mem);
    }
    pub fn main2(input: Vec<String>) {
        let mut initial_mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        initial_mem.push(0);
        let mem = exec(initial_mem, || 5, |x| println!("output {}", x));
        println!("{:?}", mem);
    }
}

mod advent_06 {
    #[derive(Debug)]
    struct OrbitGraph {
        bodies: Vec<OrbitBody>,
    }
    #[derive(Debug)]
    struct OrbitBody {
        body_name: String,
        sub_orbits: Vec<usize>,
    }
    impl OrbitGraph {
        fn get_orbit(&self, body_name: &str) -> Option<usize> {
            self.bodies.iter().position(|a| a.body_name == body_name)
        }
        fn fetch_orbit(&mut self, body_name: &str) -> usize {
            match self.get_orbit(body_name) {
                Some(index) => index,
                None => {
                    self.bodies.push(OrbitBody {
                        body_name: body_name.to_owned(),
                        sub_orbits: vec![],
                    });
                    self.bodies.len() - 1
                }
            }
        }
        fn push_orbit(&mut self, parent_body_name: &str, orbiting_body_name: &str) {
            let parent_index = self.fetch_orbit(parent_body_name);
            let orbiting_index = self.fetch_orbit(orbiting_body_name);
            self.bodies[parent_index].sub_orbits.push(orbiting_index);
        }
        fn sum_orbits(&self, body_name: &str) -> i32 {
            let init = self.get_orbit(body_name).unwrap();

            let mut bfs_state = vec![init];
            let mut bfs_state_next = vec![];

            let mut sum = 0;
            let mut depth = 0;

            while bfs_state.len() > 0 {
                //println!("{:?}", bfs_state);
                for x in bfs_state.iter() {
                    sum += depth;
                    bfs_state_next.extend(self.bodies[*x].sub_orbits.iter())
                }
                std::mem::swap(&mut bfs_state, &mut bfs_state_next);
                bfs_state_next.clear();
                depth += 1;
            }

            sum
        }

        fn orbit_traversal_length(&self, a: usize, b: usize) -> Option<i32> {
            let mut bfs_state = vec![a];
            let mut bfs_state_next = vec![];

            let mut depth = 0;
            let mut seen = std::collections::HashSet::new();

            while bfs_state.len() > 0 {
                println!(
                    "{:?}",
                    bfs_state
                        .iter()
                        .map(|x| &self.bodies[*x].body_name)
                        .collect::<Vec<&String>>()
                );
                for x in bfs_state.iter() {
                    seen.insert(*x);
                    bfs_state_next.extend(
                        self.bodies[*x]
                            .sub_orbits
                            .iter()
                            .filter(|s| !seen.contains(s)),
                    );
                    if let Some(index) = self.bodies.iter().position(|b| b.sub_orbits.contains(x)) {
                        if !seen.contains(&index) {
                            bfs_state_next.push(index);
                        }
                    }
                }
                std::mem::swap(&mut bfs_state, &mut bfs_state_next);
                bfs_state_next.clear();
                depth += 1;

                if bfs_state.contains(&b) {
                    return Some(depth);
                }
            }
            None
        }
    }
    pub fn main(map: Vec<String>) {
        //println!("{:?}", map);
        let mut g = OrbitGraph { bodies: vec![] };
        for d in map {
            let a: Vec<&str> = d.split(")").collect();
            g.push_orbit(a[0], a[1]);
        }
        //println!("{:?}", g);
        println!("{:?}", g.sum_orbits("COM"));
    }
    pub fn test() {
        main(vec![
            "COM)B".to_owned(),
            "B)C".to_owned(),
            "C)D".to_owned(),
            "D)E".to_owned(),
            "E)F".to_owned(),
            "B)G".to_owned(),
            "G)H".to_owned(),
            "D)I".to_owned(),
            "E)J".to_owned(),
            "J)K".to_owned(),
            "K)L".to_owned(),
        ]);
    }
    pub fn main2(map: Vec<String>) {
        let mut g = OrbitGraph { bodies: vec![] };
        for d in map {
            let a: Vec<&str> = d.split(")").collect();
            g.push_orbit(a[0], a[1]);
        }
        println!(
            "{:?}",
            g.orbit_traversal_length(g.get_orbit("YOU").unwrap(), g.get_orbit("SAN").unwrap())
        );
    }
    pub fn test2() {
        main2(vec![
            "COM)B".to_owned(),
            "B)C".to_owned(),
            "C)D".to_owned(),
            "D)E".to_owned(),
            "E)F".to_owned(),
            "B)G".to_owned(),
            "G)H".to_owned(),
            "D)I".to_owned(),
            "E)J".to_owned(),
            "J)K".to_owned(),
            "K)L".to_owned(),
            "K)YOU".to_owned(),
            "I)SAN".to_owned(),
        ]);
    }
}

mod advent_07 {
    fn permutations2<T, F>(k: usize, arr: &mut Vec<T>, output: &mut F)
    where
        F: FnMut(&Vec<T>),
    {
        if k == 1 {
            output(arr);
        } else {
            permutations2(k - 1, arr, output);
            for i in 0..(k - 1) {
                if k % 2 == 0 {
                    arr.swap(i, k - 1);
                } else {
                    arr.swap(0, k - 1);
                }
                permutations2(k - 1, arr, output);
            }
        }
    }

    fn permutations<T, F>(x: &Vec<T>, mut output: F)
    where
        T: Clone,
        F: FnMut(&Vec<T>),
    {
        permutations2(x.len(), &mut x.clone(), &mut output);
    }
    pub fn run_with_input(initial_mem: &Vec<i64>, config: i64, signal: i64) -> i64 {
        let mut computer = super::intcode::IntcodeComputer::new(initial_mem);
        computer.run();
        computer.provide_input(config);
        computer.provide_input(signal);
        computer.current_output().unwrap()
    }
    pub fn run(initial_mem: &Vec<i64>) {
        let mut max = i64::min_value();
        permutations(&vec![0, 1, 2, 3, 4], |v| {
            let a = run_with_input(&initial_mem, v[0], 0);
            let b = run_with_input(&initial_mem, v[1], a);
            let c = run_with_input(&initial_mem, v[2], b);
            let d = run_with_input(&initial_mem, v[3], c);
            let e = run_with_input(&initial_mem, v[4], d);
            if e > max {
                max = e
            }
        });
        println!("max = {}", max);
    }
    pub fn run2(initial_mem: &Vec<i64>) {
        let mut max = i64::min_value();
        permutations(&vec![5, 6, 7, 8, 9], |v| {
            let mut computers = [
                super::intcode::IntcodeComputer::new(initial_mem),
                super::intcode::IntcodeComputer::new(initial_mem),
                super::intcode::IntcodeComputer::new(initial_mem),
                super::intcode::IntcodeComputer::new(initial_mem),
                super::intcode::IntcodeComputer::new(initial_mem),
            ];
            for i in 0..computers.len() {
                computers[i].run();
                computers[i].provide_input(v[i]);
            }

            computers[0].provide_input(0);
            loop {
                for i in 0..5 {
                    match (computers[i].state, computers[(i + 1) % 5].state) {
                        (
                            super::intcode::IntcodeState::Output(x),
                            super::intcode::IntcodeState::NeedsInput,
                        ) => {
                            computers[(i + 1) % 5].provide_input(x);
                            computers[i].run();
                        }
                        _ => {}
                    }
                }

                if let (
                    super::intcode::IntcodeState::Halt,
                    super::intcode::IntcodeState::Output(x),
                ) = (computers[0].state, computers[4].state)
                {
                    if x > max {
                        max = x;
                    }
                    break;
                }
            }
        });
        println!("max = {}", max);
    }
    pub fn test() {
        run(&vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ]);
        run(&vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ]);
        run(&vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ]);
    }

    pub fn main(input: Vec<String>) {
        let mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        run(&mem);
    }

    pub fn test2() {
        run2(&vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ]);
        run2(&vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ]);
    }
    pub fn main2(input: Vec<String>) {
        let mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        run2(&mem);
    }
}

mod advent_09 {
    use super::intcode;

    pub fn test1() {
        intcode::run_intcode_with_inputs_and_print_outputs(
            &vec![
                109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
            ],
            &Vec::<i64>::new(),
        );

        intcode::run_intcode_with_inputs_and_print_outputs(
            &vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0],
            &Vec::<i64>::new(),
        );

        intcode::run_intcode_with_inputs_and_print_outputs(
            &vec![104, 1125899906842624, 99],
            &Vec::<i64>::new(),
        );
    }

    pub fn main1(input: Vec<String>) {
        let mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        intcode::run_intcode_with_inputs_and_print_outputs(&mem, &vec![1]);
    }
    pub fn main2(input: Vec<String>) {
        let mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        intcode::run_intcode_with_inputs_and_print_outputs(&mem, &vec![2]);
    }
}

fn main() {
    //advent_09::test1();
    advent_09::main1(read_input("inputs/input9"));
    advent_09::main2(read_input("inputs/input9"));
}
