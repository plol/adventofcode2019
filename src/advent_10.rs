pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        10
    }
    fn main1(input: &Vec<String>) -> String {
        self::main1(input)
    }
    fn main2(input: &Vec<String>) -> String {
        self::main2(input)
    }
}
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
pub fn main1(map: &Vec<String>) -> String {
    let asteroids = asteroid_coords(map);

    //let mut max_coords = (0, 0);
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
            //max_coords = (*x1, *y1);
        }
    }

    //println!("{:?} {}", max_coords, max)
    format!("{}", max)
}
pub fn test1() {
    main1(&vec![
        ".#..#".to_owned(),
        ".....".to_owned(),
        "#####".to_owned(),
        "....#".to_owned(),
        "...##".to_owned(),
    ]);

    main1(&vec![
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

    main1(&vec![
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

    main1(&vec![
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

    main1(&vec![
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

    scan_results.sort_by(|(v1, _), (v2, _)| pos_angle(*v1).partial_cmp(&pos_angle(*v2)).unwrap());

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

pub fn find_200th_asteroid_to_be_vaporized(asteroids: &Vec<Asteroid>, origin: Asteroid) -> String {
    let scan_results = scan(asteroids, origin);

    let todo = obliteration_order(&scan_results);

    format!("{}{}", todo[199].0, todo[199].1)
}

pub fn main2(map: &Vec<String>) -> String {
    let asteroids = asteroid_coords(&map);

    find_200th_asteroid_to_be_vaporized(&asteroids, (20, 20))
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
