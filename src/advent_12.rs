pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        12
    }
    fn main1(input: &Vec<String>) -> String {
        let mut moons1 = parse_moons(input);
        let mut moons2 = moons1.clone();

        for _ in 0..500 {
            step(&moons1, &mut moons2);
            step(&moons2, &mut moons1);
        }
        format!("{:?}", total_energy(&moons1))
    }
    fn main2(input: &Vec<String>) -> String {
        self::main2(input)
    }
}
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

pub fn main2(moon_strs: &Vec<String>) -> String {
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
    format!(
        "{}",
        (x_loop as i64) * (y_loop as i64) * (z_loop as i64) / (xy_gcd as i64 * yz_gcd as i64)
    )
}

pub fn test2() {
    main2(&vec![
        "<x=-8, y=-10, z=0>".to_owned(),
        "<x=5, y=5, z=10>".to_owned(),
        "<x=2, y=-7, z=3>".to_owned(),
        "<x=9, y=-8, z=-3>".to_owned(),
    ]);
}
