pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        22
    }
    fn main1(input: &Vec<String>) -> String {
        let (k, m) = parse(input, 10007);
        let x = ModWrappingI64 {
            value: 2019,
            modulus: 10007,
        };
        format!("{}", (k * x + m).value)
    }

    fn main2(input: &Vec<String>) -> String {
        let ops = parse(input, 119315717514047);
        let (k, m) = polynom_pow(ops, 101741582076661);
        let x = ModWrappingI64 {
            value: 2020,
            modulus: 119315717514047,
        };
        let result = (x - m) / k;
        format!("{}", result.value)
    }
}

fn munch<'a>(input: &'a str, pattern: &str) -> Option<&'a str> {
    if input.starts_with(pattern) {
        Some(&input[pattern.len()..])
    } else {
        None
    }
}

fn combine(
    (m1, s1): (ModWrappingI64, ModWrappingI64),
    (m2, s2): (ModWrappingI64, ModWrappingI64),
) -> (ModWrappingI64, ModWrappingI64) {
    (m1 * m2, s1 * m2 + s2)
}

fn parse(input: &Vec<String>, deck_size: i64) -> (ModWrappingI64, ModWrappingI64) {
    input
        .iter()
        .flat_map(|line| {
            {
                vec![
                    munch(line, "deal with increment ").map(|inc| (inc.parse().unwrap(), 0)),
                    munch(line, "cut ").map(|at| (1, -at.parse::<i64>().unwrap())),
                    munch(line, "deal into new stack").map(|_| (-1, -1)),
                ]
            }
        })
        .flatten()
        .map(|(k, m)| {
            (
                ModWrappingI64 {
                    value: k,
                    modulus: deck_size,
                },
                ModWrappingI64 {
                    value: m,
                    modulus: deck_size,
                },
            )
        })
        .fold(
            (
                ModWrappingI64 {
                    value: 1,
                    modulus: deck_size,
                },
                ModWrappingI64 {
                    value: 0,
                    modulus: deck_size,
                },
            ),
            |ret, v| combine(ret, v),
        )
}

fn polynom_pow(x: (ModWrappingI64, ModWrappingI64), n: i64) -> (ModWrappingI64, ModWrappingI64) {
    let ret = match n {
        0 => panic!(),
        1 => x,
        _ if n % 2 == 0 => polynom_pow(combine(x, x), n / 2),
        _ => combine(x, polynom_pow(combine(x, x), (n - 1) / 2)),
    };
    ret
}

fn gcd_extended(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (gcd, x1, y1) = gcd_extended(b % a, a);
        (gcd, y1 - (b / a) * x1, x1)
    }
}

fn mod_inverse(x: i64, modulus: i64) -> i64 {
    let (g, x, _) = gcd_extended(x, modulus);
    if g != 1 {
        panic!("GCD of {} and {} == {}", x, modulus, g);
    } else {
        (x % modulus + modulus) % modulus
    }
}

#[derive(Copy, Clone, Eq, PartialEq, PartialOrd)]
struct ModWrappingI64 {
    value: i64,
    modulus: i64,
}

impl ModWrappingI64 {
    fn invert(&self) -> ModWrappingI64 {
        ModWrappingI64 {
            value: self.modulus - self.value - 1,
            modulus: self.modulus,
        }
    }
}

impl std::fmt::Debug for ModWrappingI64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.value)
    }
}

impl std::ops::Add for ModWrappingI64 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        ModWrappingI64 {
            value: (self.value + other.value + self.modulus) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl std::ops::Sub for ModWrappingI64 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        ModWrappingI64 {
            value: (self.value - other.value + self.modulus) % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl std::ops::Mul for ModWrappingI64 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        ModWrappingI64 {
            value: (((self.value as i128 * other.value as i128) % self.modulus as i128) as i64
                + self.modulus)
                % self.modulus,
            modulus: self.modulus,
        }
    }
}

impl std::ops::Div for ModWrappingI64 {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        self * ModWrappingI64 {
            value: mod_inverse(other.value, self.modulus),
            modulus: self.modulus,
        }
    }
}
