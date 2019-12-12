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

mod advent_08 {
    pub fn count_chars(s: &str, ccc: char) -> usize {
        s.chars().filter(|c| c == &ccc).count()
    }
    pub fn uh(input: &str, w: usize, h: usize) {
        let mut input_m = input;
        let mut layers = vec![];
        loop {
            if input_m.len() == 0 {
                break;
            }
            let (image_data, rest) = input_m.split_at(w * h);
            input_m = rest;
            layers.push(image_data);
        }
        let layer_with_fewest_0_digits = layers.iter().min_by_key(|s| count_chars(s, '0')).unwrap();
        println!("{}", layer_with_fewest_0_digits);
        println!(
            "{}",
            count_chars(layer_with_fewest_0_digits, '1')
                * count_chars(layer_with_fewest_0_digits, '2')
        );
    }

    pub fn decode_pixel(i: usize, layers: &Vec<&str>) -> char {
        for layer in layers {
            match layer.chars().nth(i).unwrap() {
                '0' => return ' ',
                '1' => return '#',
                '2' => {}
                _ => panic!(),
            }
        }
        panic!();
    }

    pub fn uh2(input: &str, w: usize, h: usize) {
        let mut input_m = input;
        let mut layers = vec![];
        loop {
            if input_m.len() == 0 {
                break;
            }
            let (image_data, rest) = input_m.split_at(w * h);
            input_m = rest;
            layers.push(image_data);
        }
        for y in 0..h {
            for x in 0..w {
                print!("{}", decode_pixel(y * w + x, &layers));
            }
            println!("");
        }
    }

    pub fn test1() {
        uh("123456789012", 3, 2);
    }
    pub fn main1(input: Vec<String>) {
        uh(&input.join(""), 25, 6);
    }
    pub fn test2() {
        uh2("0222112222120000", 2, 2);
    }
    pub fn main2(input: Vec<String>) {
        uh2(&input.join(""), 25, 6);
    }
}

mod advent_10 {
    pub fn gcd(a: i32, b: i32) -> i32 {
        let mut x = a;
        let mut y = b;
        while y != 0 {
            let t = y;
            y = x % y;
            x = t;
        }
        x.abs()
    }

    pub type Asteroid = (i32, i32);

    pub fn asteroid_coords(map: &Vec<String>) -> Vec<(i32, i32)> {
        let mut ret = vec![];
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                if map[y].chars().nth(x).unwrap() == '#' {
                    ret.push((x as i32, y as i32));
                }
            }
        }
        ret
    }
    pub fn main1(map: Vec<String>) {
        let asteroids = asteroid_coords(&map);

        let mut max_coords = (0, 0);
        let mut max = i32::min_value();

        for (x1, y1) in &asteroids {
            let mut seen = std::collections::HashSet::new();
            for (x2, y2) in &asteroids {
                let dx = x2 - x1;
                let dy = y2 - y1;
                let vx;
                let vy;
                if dx != 0 && dy != 0 {
                    let divisor = gcd(dx, dy);
                    //println!("The divisor is {}", divisor);
                    vx = dx / divisor;
                    vy = dy / divisor;
                } else if dx == 0 && dy != 0 {
                    vx = 0;
                    vy = dy / dy.abs();
                } else if dx != 0 && dy == 0 {
                    vx = dx / dx.abs();
                    vy = 0;
                } else {
                    vx = 0;
                    vy = 0;
                }
                //println!("{:?} see {:?} at {:?}", (x1, y1), (x2, y2), (vx, vy));
                if !seen.contains(&(vx, vy)) {
                    seen.insert((vx, vy));
                }
            }
            let seen_count = seen.len() as i32 - 1;
            //println!("{:?} {} {:?}", (x1, y1), seen_count, seen);
            if seen_count > max {
                max = seen_count;
                max_coords = (*x1, *y1);
            }
        }

        println!("{:?} {}", max_coords, max);
    }
    pub fn test1() {
        main1(vec![
            ".#..#".to_owned(),
            ".....".to_owned(),
            "#####".to_owned(),
            "....#".to_owned(),
            "...##".to_owned(),
        ]);

        main1(vec![
            "......#.#.".to_owned(),
            "#..#.#....".to_owned(),
            "..#######.".to_owned(),
            ".#.#.###..".to_owned(),
            ".#..#.....".to_owned(),
            "..#....#.#".to_owned(),
            "#..#....#.".to_owned(),
            ".##.#..###".to_owned(),
            "##...#..#.".to_owned(),
            ".#....####".to_owned(),
        ]);

        main1(vec![
            "#.#...#.#.".to_owned(),
            ".###....#.".to_owned(),
            ".#....#...".to_owned(),
            "##.#.#.#.#".to_owned(),
            "....#.#.#.".to_owned(),
            ".##..###.#".to_owned(),
            "..#...##..".to_owned(),
            "..##....##".to_owned(),
            "......#...".to_owned(),
            ".####.###.".to_owned(),
        ]);

        main1(vec![
            ".#..#..###".to_owned(),
            "####.###.#".to_owned(),
            "....###.#.".to_owned(),
            "..###.##.#".to_owned(),
            "##.##.#.#.".to_owned(),
            "....###..#".to_owned(),
            "..#.#..#.#".to_owned(),
            "#..#.#.###".to_owned(),
            ".##...##.#".to_owned(),
        ]);

        main1(vec![
            ".#..##.###...#######".to_owned(),
            "##.############..##.".to_owned(),
            ".#.######.########.#".to_owned(),
            ".###.#######.####.#.".to_owned(),
            "#####.##.#.##.###.##".to_owned(),
            "..#####..#.#########".to_owned(),
            "####################".to_owned(),
            "#.####....###.#.#.##".to_owned(),
            "##.#################".to_owned(),
            "#####.##.###..####..".to_owned(),
            "..######..##.#######".to_owned(),
            "####.##.####...##..#".to_owned(),
            ".#####..#.######.###".to_owned(),
            "##...#.##########...".to_owned(),
            "#.##########.#######".to_owned(),
            ".####.#.###.###.#.##".to_owned(),
            "....##.##.###..#####".to_owned(),
            ".#.#.###########.###".to_owned(),
            "#.#.#.#####.####.###".to_owned(),
            "###.##.####.##.#..##".to_owned(),
        ]);
    }

    pub fn scan(asteroids: &Vec<Asteroid>, (x1, y1): (i32, i32)) -> Vec<Vec<Asteroid>> {
        let mut scan_result_map = std::collections::HashMap::<(i32, i32), Vec<Asteroid>>::new();
        for (x2, y2) in asteroids {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let vx;
            let vy;
            if dx != 0 && dy != 0 {
                let divisor = gcd(dx, dy);
                //println!("The divisor is {}", divisor);
                vx = dx / divisor;
                vy = dy / divisor;
            } else if dx == 0 && dy != 0 {
                vx = 0;
                vy = dy / dy.abs();
            } else if dx != 0 && dy == 0 {
                vx = dx / dx.abs();
                vy = 0;
            } else {
                continue;
            }
            //println!("{:?} see {:?} at {:?}", (x1, y1), (x2, y2), (vx, vy));
            if let Some(seen_vec) = scan_result_map.get_mut(&(vx, vy)) {
                seen_vec.push((*x2, *y2));
            } else {
                scan_result_map.insert((vx, vy), vec![(*x2, *y2)]);
            }
        }

        let mut scan_results: Vec<((i32, i32), Vec<Asteroid>)> = scan_result_map
            .drain()
            .map(|(coord, mut asteroids)| {
                asteroids.sort_by_key(|(x2, y2)| (x2 - x1).pow(2) + (y2 - y1).pow(2));
                (coord, asteroids)
            })
            .collect();

        fn pos_angle((vx, vy): (i32, i32)) -> f64 {
            let pi = std::f64::consts::PI;
            (((vy as f64).atan2(vx as f64) + pi * 2.5) % (pi * 2.0))
        }

        scan_results
            .sort_by(|(v1, _), (v2, _)| pos_angle(*v1).partial_cmp(&pos_angle(*v2)).unwrap());

        scan_results
            .drain(0..)
            .map(|(_, asteroids)| asteroids)
            .collect()
    }

    pub fn obliteration_order(scan_results: &Vec<Vec<Asteroid>>) -> Vec<Asteroid> {
        let mut indices = vec![0; scan_results.len()];

        let mut ret = vec![];
        loop {
            let mut any = false;
            for i in 0..indices.len() {
                if indices[i] < scan_results[i].len() {
                    ret.push(scan_results[i][indices[i]]);
                    indices[i] += 1;
                    any = true;
                }
            }
            if !any {
                break;
            }
        }
        ret
    }

    pub fn find_200th_asteroid_to_be_vaporized(asteroids: &Vec<Asteroid>, origin: Asteroid) {
        let scan_results = scan(asteroids, origin);

        let todo = obliteration_order(&scan_results);

        println!("{:?}", todo[199]);
    }

    pub fn main2(map: Vec<String>) {
        let asteroids = asteroid_coords(&map);

        find_200th_asteroid_to_be_vaporized(&asteroids, (20, 20));
    }

    pub fn test2() {
        //let asteroids = asteroid_coords(&vec![
        //    ".#....#####...#..".to_owned(),
        //    "##...##.#####..##".to_owned(),
        //    "##...#...#.#####.".to_owned(),
        //    "..#.....#...###..".to_owned(),
        //    "..#.#.....#....##".to_owned(),
        //]);
        //let scan_results = scan(&asteroids, (8, 3));

        //let todo = obliteration_order(&scan_results);

        find_200th_asteroid_to_be_vaporized(
            &asteroid_coords(&vec![
                ".#..##.###...#######".to_owned(),
                "##.############..##.".to_owned(),
                ".#.######.########.#".to_owned(),
                ".###.#######.####.#.".to_owned(),
                "#####.##.#.##.###.##".to_owned(),
                "..#####..#.#########".to_owned(),
                "####################".to_owned(),
                "#.####....###.#.#.##".to_owned(),
                "##.#################".to_owned(),
                "#####.##.###..####..".to_owned(),
                "..######..##.#######".to_owned(),
                "####.##.####...##..#".to_owned(),
                ".#####..#.######.###".to_owned(),
                "##...#.##########...".to_owned(),
                "#.##########.#######".to_owned(),
                ".####.#.###.###.#.##".to_owned(),
                "....##.##.###..#####".to_owned(),
                ".#.#.###########.###".to_owned(),
                "#.#.#.#####.####.###".to_owned(),
                "###.##.####.##.#..##".to_owned(),
            ]),
            (11, 13),
        );
    }
}

mod advent_11 {
    use super::intcode;

    enum RobotMovementState {
        JustMoved,
        JustPainted,
    }

    pub fn render(painted_squares: &std::collections::HashMap<(i64, i64), i64>) {
        let min_x = painted_squares.keys().min_by_key(|k| k.0).unwrap().0;
        let min_y = painted_squares.keys().min_by_key(|k| k.1).unwrap().1;
        let max_x = painted_squares.keys().max_by_key(|k| k.0).unwrap().0;
        let max_y = painted_squares.keys().max_by_key(|k| k.1).unwrap().1;
        println!("between {},{} and {},{}", min_x, min_y, max_x, max_y);

        let dx = max_x - min_x;
        let dy = max_y - min_y;

        for y in 0..dy + 1 {
            for x in 0..dx + 1 {
                let color = *painted_squares
                    .get(&(x + min_x, (dy - y) + min_y))
                    .unwrap_or(&0);
                print!(
                    "{}",
                    match color {
                        0 => " ",
                        1 => "#",
                        _ => panic!(),
                    }
                );
            }
            println!("");
        }
    }

    pub fn test1(input: Vec<String>) {
        let mem = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        let mut painted_squares = std::collections::HashMap::<(i64, i64), i64>::new();

        let mut robot_brain = intcode::IntcodeComputer::new(&mem);
        let mut robot_movement_state = RobotMovementState::JustMoved;
        let mut robot_heading = (0, 1);
        let mut current_robot_pos = (0, 0);

        robot_brain.run();

        let mut is_first_input = true;

        loop {
            match robot_brain.state {
                intcode::IntcodeState::NeedsInput => {
                    //println!(
                    //    "{}: Robot at {:?} reads input {}",
                    //    i,
                    //    current_robot_pos,
                    //    *painted_squares.get(&current_robot_pos).unwrap_or(&0)
                    //);
                    robot_brain.provide_input(
                        *painted_squares
                            .get(&current_robot_pos)
                            .unwrap_or(if is_first_input { &1 } else { &0 }),
                    );
                    is_first_input = false;
                }
                intcode::IntcodeState::Output(x) => {
                    //println!("{}: Robot output {:?}", i, x);
                    match robot_movement_state {
                        RobotMovementState::JustMoved => {
                            //println!("Painting {:?} to {}", current_robot_pos, x);
                            painted_squares.insert(current_robot_pos, x);
                            robot_movement_state = RobotMovementState::JustPainted;
                        }
                        RobotMovementState::JustPainted => {
                            //print!(
                            //    "Turning! {} ({}), {:?}",
                            //    x,
                            //    if x == 1 { "right" } else { "left" },
                            //    robot_heading
                            //);
                            robot_heading = match x {
                                0 => (-robot_heading.1, robot_heading.0),
                                1 => (robot_heading.1, -robot_heading.0),
                                _ => panic!(),
                            };
                            //print!(" to {:?}", robot_heading);
                            current_robot_pos = (
                                current_robot_pos.0 + robot_heading.0,
                                current_robot_pos.1 + robot_heading.1,
                            );
                            //println!(" and moved to {:?}", current_robot_pos);
                            robot_movement_state = RobotMovementState::JustMoved;
                        }
                    }
                    robot_brain.run();
                }
                intcode::IntcodeState::Halt => {
                    break;
                }
                _ => panic!(),
            }
        }

        render(&painted_squares);
        println!("{:?}", painted_squares.len());
    }
}

mod advent_12 {
    use super::advent_10::gcd;

    #[derive(Clone, Copy, Debug)]
    struct Pos {
        x: i32,
        y: i32,
        z: i32,
    }

    #[derive(Clone, Copy, Debug)]
    struct Vel {
        x: i32,
        y: i32,
        z: i32,
    }

    impl std::ops::Add<Vel> for Pos {
        type Output = Self;
        fn add(self, other: Vel) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }
    impl std::ops::Add<Vel> for Vel {
        type Output = Self;
        fn add(self, other: Vel) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z,
            }
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct Moon {
        pos: Pos,
        vel: Vel,
    }

    fn abs_p(pos: &Pos) -> i32 {
        return pos.x.abs() + pos.y.abs() + pos.z.abs();
    }
    fn abs_v(vel: &Vel) -> i32 {
        return vel.x.abs() + vel.y.abs() + vel.z.abs();
    }
    fn moon_energy(moon: &Moon) -> i32 {
        return abs_p(&moon.pos) * abs_v(&moon.vel);
    }

    fn step(moons: &Vec<Moon>, out: &mut Vec<Moon>) {
        for i in 0..moons.len() {
            let mut vel = moons[i].vel;
            for j in 0..moons.len() {
                let d = Vel {
                    x: if moons[i].pos.x > moons[j].pos.x {
                        -1
                    } else if moons[i].pos.x < moons[j].pos.x {
                        1
                    } else {
                        0
                    },
                    y: if moons[i].pos.y > moons[j].pos.y {
                        -1
                    } else if moons[i].pos.y < moons[j].pos.y {
                        1
                    } else {
                        0
                    },
                    z: if moons[i].pos.z > moons[j].pos.z {
                        -1
                    } else if moons[i].pos.z < moons[j].pos.z {
                        1
                    } else {
                        0
                    },
                };
                vel = vel + d;
            }
            out[i].vel = vel;
            out[i].pos = moons[i].pos + vel;
        }
    }

    fn total_energy(moons: &Vec<Moon>) -> i32 {
        moons.iter().map(moon_energy).sum()
    }

    fn parse_moons(moons: &Vec<String>) -> Vec<Moon> {
        moons
            .iter()
            .map(|m_str| {
                let coords = m_str
                    .trim_matches('<')
                    .trim_matches('>')
                    .split(',')
                    .map(|coord_str| {
                        coord_str
                            .trim_matches(|c| "xyz =".contains(c))
                            .parse::<i32>()
                            .unwrap()
                    })
                    .collect::<Vec<i32>>();
                Moon {
                    pos: Pos {
                        x: coords[0],
                        y: coords[1],
                        z: coords[2],
                    },
                    vel: Vel { x: 0, y: 0, z: 0 },
                }
            })
            .collect()
    }

    pub fn test1() {
        let mut moons1 = vec![
            Moon {
                pos: Pos { x: -1, y: 0, z: 2 },
                vel: Vel { x: 0, y: 0, z: 0 },
            },
            Moon {
                pos: Pos {
                    x: 2,
                    y: -10,
                    z: -7,
                },
                vel: Vel { x: 0, y: 0, z: 0 },
            },
            Moon {
                pos: Pos { x: 4, y: -8, z: 8 },
                vel: Vel { x: 0, y: 0, z: 0 },
            },
            Moon {
                pos: Pos { x: 3, y: 5, z: -1 },
                vel: Vel { x: 0, y: 0, z: 0 },
            },
        ];
        let mut moons2 = moons1.clone();

        println!("{}, {:?}", total_energy(&moons1), moons1);
        step(&moons1, &mut moons2);
        println!("{}, {:?}", total_energy(&moons2), moons2);
        step(&moons2, &mut moons1);
        println!("{}, {:?}", total_energy(&moons1), moons1);

        for _ in 0..4 {
            step(&moons1, &mut moons2);
            step(&moons2, &mut moons1);
        }

        println!("{}, {:?}", total_energy(&moons1), moons1);
        println!("");
        println!("");
        let mut moons3 = parse_moons(&vec![
            "<x=-8, y=-10, z=0>".to_owned(),
            "<x=5, y=5, z=10>".to_owned(),
            "<x=2, y=-7, z=3>".to_owned(),
            "<x=9, y=-8, z=-3>".to_owned(),
        ]);
        let mut moons4 = moons3.clone();

        for _ in 0..50 {
            step(&moons3, &mut moons4);
            step(&moons4, &mut moons3);
        }
        println!("{:?}", total_energy(&moons3));
    }

    pub fn main1(moon_strings: Vec<String>) {
        let mut moons1 = parse_moons(&moon_strings);
        let mut moons2 = moons1.clone();

        for _ in 0..500 {
            step(&moons1, &mut moons2);
            step(&moons2, &mut moons1);
        }
        println!("{:?}", total_energy(&moons1));
    }

    /// OK
    ///

    fn find_loop(vs_orig: [i16; 4], ds_orig: [i16; 4]) -> i32 {
        let mut vs: [i16; 4] = vs_orig;
        let mut ds: [i16; 4] = ds_orig;

        let mut i = 0;

        loop {
            {
                let a01 = (vs[1] - vs[0]).signum();
                let a02 = (vs[2] - vs[0]).signum();
                let a03 = (vs[3] - vs[0]).signum();
                let a12 = (vs[2] - vs[1]).signum();
                let a13 = (vs[3] - vs[1]).signum();
                let a23 = (vs[3] - vs[2]).signum();

                ds[0] += a01 + a02 + a03;
                ds[1] += -a01 + a12 + a13;
                ds[2] += -a02 - a12 + a23;
                ds[3] += -a03 - a13 - a23;
                vs[0] += ds[0];
                vs[1] += ds[1];
                vs[2] += ds[2];
                vs[3] += ds[3];
                i += 1;
                if vs == vs_orig && ds == ds_orig {
                    return i;
                }
            }
        }
    }

    pub fn main2(moon_strs: Vec<String>) {
        let moons = parse_moons(&moon_strs);

        let vxs: Vec<i16> = moons.iter().map(|m| m.pos.x as i16).collect();
        //let vdxs: Vec<i32> = moons.iter().map(|m| m.vel.x).collect();
        let vys: Vec<i16> = moons.iter().map(|m| m.pos.y as i16).collect();
        //let vdys: Vec<i32> = moons.iter().map(|m| m.vel.y).collect();
        let vzs: Vec<i16> = moons.iter().map(|m| m.pos.z as i16).collect();
        //let vdzs: Vec<i32> = moons.iter().map(|m| m.vel.z).collect();
        let xs = [vxs[0], vxs[1], vxs[2], vxs[3]];
        let ys = [vys[0], vys[1], vys[2], vys[3]];
        let zs = [vzs[0], vzs[1], vzs[2], vzs[3]];
        let dxs = [0, 0, 0, 0];
        let dys = [0, 0, 0, 0];
        let dzs = [0, 0, 0, 0];

        let x_loop_thread = std::thread::spawn(move || find_loop(xs, dxs));
        //println!("x looped after {} steps", x_loop);

        let y_loop_thread = std::thread::spawn(move || find_loop(ys, dys));
        //println!("y looped after {} steps", y_loop);

        let z_loop = find_loop(zs, dzs);
        //println!("z looped after {} steps", z_loop);
        //println!(
        //    "{} {} {}",
        //    gcd(x_loop, y_loop),
        //    gcd(y_loop, z_loop),
        //    gcd(x_loop, z_loop)
        //);
        //println!("{}", (x_loop as i64) * (y_loop as i64) * (z_loop as i64));

        //let xy_loop = (x_loop as i64) * (y_loop as i64) / (gcd(x_loop, y_loop) as i64);
        //let xyz_loop = (xy_loop as i64) * (z_loop as i64) / (gcd(xy_loop as i32, z_loop) as i64);
        //
        let x_loop = x_loop_thread.join().unwrap();
        let y_loop = y_loop_thread.join().unwrap();

        let xy_gcd = gcd(x_loop, y_loop);
        let yz_gcd = gcd(y_loop, z_loop);
        println!(
            "{}",
            (x_loop as i64) * (y_loop as i64) * (z_loop as i64) / (xy_gcd as i64 * yz_gcd as i64)
        );
    }

    pub fn test2() {
        main2(vec![
            "<x=-8, y=-10, z=0>".to_owned(),
            "<x=5, y=5, z=10>".to_owned(),
            "<x=2, y=-7, z=3>".to_owned(),
            "<x=9, y=-8, z=-3>".to_owned(),
        ]);
    }
}

fn main() {
    let mut min = 123456.0;
    for _ in 0..100000 {
        let now = std::time::Instant::now();
        //advent_12::test1();
        //advent_12::main1(read_input("inputs/input12"));

        //advent_12::test2();
        advent_12::main2(read_input("inputs/input12"));
        let dt = (std::time::Instant::now() - now).as_micros() as f64 / 1000.0;
        //println!("Took {} ms", dt);
        if dt < min {
            min = dt
        }
    }
    println!("Took minimally {} ms", min);
}
