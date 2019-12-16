pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        14
    }
    fn main1(input: &Vec<String>) -> String {
        self::main1(input)
    }
    fn main2(input: &Vec<String>) -> String {
        self::main2(input)
    }
}

type Spec = (i64, String);
#[derive(Debug, Clone)]
struct Data {
    ore_count: i64,
    remnants: std::collections::VecDeque<Spec>,
    current: std::collections::VecDeque<Spec>,
}

fn parse_thing(thing: &str) -> Spec {
    let a = thing.trim().split(" ").collect::<Vec<&str>>();
    (a[0].parse().unwrap(), a[1].to_owned())
}
fn parse_graph(input: &Vec<String>) -> std::collections::HashMap<Spec, Vec<Spec>> {
    let mut graph = std::collections::HashMap::new();
    for s in input {
        let parts = s.split("=>").collect::<Vec<&str>>();
        graph.insert(
            parse_thing(&parts[1]),
            parts[0].split(",").map(parse_thing).collect::<Vec<Spec>>(),
        );
    }
    graph
}

fn merge_with2<T, U, F, F2, F0>(
    a: &mut std::collections::VecDeque<T>,
    b: &std::collections::VecDeque<U>,
    mut cmp: F0,
    mut merge_fn: F,
    mut create: F2,
) where
    F: FnMut(&mut T, &U),
    F2: FnMut(&U) -> T,
    F0: FnMut(&mut T, &U) -> bool,
{
    for i in 0..b.len() {
        let mut found = false;
        for j in 0..a.len() {
            if cmp(&mut a[j], &b[i]) {
                merge_fn(&mut a[j], &b[i]);
                found = true;
                break;
            }
        }
        if !found {
            a.push_back(create(&b[i]));
        }
    }
}
fn merge_with<T, F, F0>(
    a: &mut std::collections::VecDeque<T>,
    b: &std::collections::VecDeque<T>,
    cmp: F0,
    merge_fn: F,
) where
    F: FnMut(&mut T, &T),
    T: Clone,
    F0: FnMut(&mut T, &T) -> bool,
{
    merge_with2(a, b, cmp, merge_fn, |t| t.clone())
}
fn step(data: &mut Data, graph: &std::collections::HashMap<Spec, Vec<Spec>>) {
    let (mut current_count, current_type) = data.current.pop_front().expect("AAAAAb");
    if current_type == "ORE" {
        data.ore_count += current_count;
    } else {
        let key = graph
            .keys()
            .find(|k| k.1 == current_type)
            .unwrap_or_else(|| panic!("i needed to find {}", current_type));
        if let Some(i) = data.remnants.iter().position(|v| v.1 == current_type) {
            //println!("I need {} {} and i have {} in remnants", current_count, current_type, data.remnants[i].0);
            if data.remnants[i].0 > current_count {
                data.remnants[i].0 -= current_count;
                current_count = 0;
            } else {
                current_count -= data.remnants[i].0;
                data.remnants.remove(i);
            }
        }

        let count = current_count / key.0 + (current_count % key.0 != 0) as i64;

        while count * key.0 < current_count {
            panic!();
        }
        //println!("to consume {:?} i need {:?} of them {:?} {:?}",(current_count, &current_type), count, key, graph[key]);
        let mut uhh1: std::collections::VecDeque<Spec> = graph[key].clone().into();
        let mut uhh2 = std::collections::VecDeque::new();
        for i in 0..uhh1.len() {
            uhh1[i].0 *= count;
        }
        if count * key.0 != current_count {
            uhh2.push_back((count * key.0 - current_count, current_type.clone()));
        }
        //println!("consuming {:?}, remnants: {:?}", uhh1, uhh2);

        merge_with(
            &mut data.current,
            &uhh1,
            |a, b| a.1 == b.1,
            |a, b| a.0 += b.0,
        );
        merge_with(
            &mut data.remnants,
            &uhh2,
            |a, b| a.1 == b.1,
            |a, b| a.0 += b.0,
        );
    }
}
pub fn main1(input: &Vec<String>) -> String {
    let graph = parse_graph(&input);

    let mut data = Data {
        ore_count: 0,
        current: vec![(1, "FUEL".to_owned())].into(),
        remnants: vec![].into(),
    };
    while data.current.len() > 0 {
        //println!("\n\n{:?}\n", data);
        step(&mut data, &graph);
    }
    format!("{}", data.ore_count)
}

pub fn main2(input: &Vec<String>) -> String {
    let graph = parse_graph(input);

    let mut data = Data {
        ore_count: 0,
        current: vec![].into(),
        remnants: vec![].into(),
    };
    let mut prev_data = data.clone();
    let mut i: i64 = 0;
    for x in vec![1000000, 100000, 10000, 1000, 100, 10, 1] {
        while data.ore_count < 1000000000000 {
            i += x;
            prev_data = data.clone();
            data.current.push_back((x, "FUEL".to_owned()));
            while data.current.len() > 0 {
                //println!("\n\n{:?}\n", data);
                step(&mut data, &graph);
            }
            //println!("{}, {}, {}", i, data.ore_count, x);
        }
        data = prev_data.clone();
        i -= x;
    }
    //println!("{}, {:?}", i, data);
    format!("{}", i)
}
pub fn test1() {
    main1(&vec![
        "9 ORE => 2 A".to_owned(),
        "8 ORE => 3 B".to_owned(),
        "7 ORE => 5 C".to_owned(),
        "3 A, 4 B => 1 AB".to_owned(),
        "5 B, 7 C => 1 BC".to_owned(),
        "4 C, 1 A => 1 CA".to_owned(),
        "2 AB, 3 BC, 4 CA => 1 FUEL".to_owned(),
    ]);
    main1(&vec![
        "157 ORE => 5 NZVS".to_owned(),
        "165 ORE => 6 DCFZ".to_owned(),
        "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL".to_owned(),
        "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ".to_owned(),
        "179 ORE => 7 PSHF".to_owned(),
        "177 ORE => 5 HKGWZ".to_owned(),
        "7 DCFZ, 7 PSHF => 2 XJWVT".to_owned(),
        "165 ORE => 2 GPVTF".to_owned(),
        "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_owned(),
    ]);
    main1(&vec![
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG".to_owned(),
        "17 NVRVD, 3 JNWZP => 8 VPVL".to_owned(),
        "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL".to_owned(),
        "22 VJHF, 37 MNCFX => 5 FWMGM".to_owned(),
        "139 ORE => 4 NVRVD".to_owned(),
        "144 ORE => 7 JNWZP".to_owned(),
        "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC".to_owned(),
        "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV".to_owned(),
        "145 ORE => 6 MNCFX".to_owned(),
        "1 NVRVD => 8 CXFTF".to_owned(),
        "1 VJHF, 6 MNCFX => 4 RFSQX".to_owned(),
        "176 ORE => 6 VJHF".to_owned(),
    ]);
    main1(&vec![
        "171 ORE => 8 CNZTR".to_owned(),
        "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL".to_owned(),
        "114 ORE => 4 BHXH".to_owned(),
        "14 VRPVC => 6 BMBT".to_owned(),
        "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL".to_owned(),
        "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT".to_owned(),
        "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW".to_owned(),
        "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW".to_owned(),
        "5 BMBT => 4 WPTQ".to_owned(),
        "189 ORE => 9 KTJDG".to_owned(),
        "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP".to_owned(),
        "12 VRPVC, 27 CNZTR => 2 XDBXC".to_owned(),
        "15 KTJDG, 12 BHXH => 5 XCVML".to_owned(),
        "3 BHXH, 2 VRPVC => 7 MZWV".to_owned(),
        "121 ORE => 7 VRPVC".to_owned(),
        "7 XCVML => 6 RJRHP".to_owned(),
        "5 BHXH, 4 VRPVC => 5 LTCX".to_owned(),
    ]);
}
pub fn test2() {
    // main2(vec![
    //    "9 ORE => 2 A".to_owned(),
    //    "8 ORE => 3 B".to_owned(),
    //    "7 ORE => 5 C".to_owned(),
    //    "3 A, 4 B => 1 AB".to_owned(),
    //    "5 B, 7 C => 1 BC".to_owned(),
    //    "4 C, 1 A => 1 CA".to_owned(),
    //    "2 AB, 3 BC, 4 CA => 1 FUEL".to_owned()
    // ]);
    main2(&vec![
        "157 ORE => 5 NZVS".to_owned(),
        "165 ORE => 6 DCFZ".to_owned(),
        "44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL".to_owned(),
        "12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ".to_owned(),
        "179 ORE => 7 PSHF".to_owned(),
        "177 ORE => 5 HKGWZ".to_owned(),
        "7 DCFZ, 7 PSHF => 2 XJWVT".to_owned(),
        "165 ORE => 2 GPVTF".to_owned(),
        "3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT".to_owned(),
    ]);
    main2(&vec![
        "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG".to_owned(),
        "17 NVRVD, 3 JNWZP => 8 VPVL".to_owned(),
        "53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL".to_owned(),
        "22 VJHF, 37 MNCFX => 5 FWMGM".to_owned(),
        "139 ORE => 4 NVRVD".to_owned(),
        "144 ORE => 7 JNWZP".to_owned(),
        "5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC".to_owned(),
        "5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV".to_owned(),
        "145 ORE => 6 MNCFX".to_owned(),
        "1 NVRVD => 8 CXFTF".to_owned(),
        "1 VJHF, 6 MNCFX => 4 RFSQX".to_owned(),
        "176 ORE => 6 VJHF".to_owned(),
    ]);
    main2(&vec![
        "171 ORE => 8 CNZTR".to_owned(),
        "7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL".to_owned(),
        "114 ORE => 4 BHXH".to_owned(),
        "14 VRPVC => 6 BMBT".to_owned(),
        "6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL".to_owned(),
        "6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT".to_owned(),
        "15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW".to_owned(),
        "13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW".to_owned(),
        "5 BMBT => 4 WPTQ".to_owned(),
        "189 ORE => 9 KTJDG".to_owned(),
        "1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP".to_owned(),
        "12 VRPVC, 27 CNZTR => 2 XDBXC".to_owned(),
        "15 KTJDG, 12 BHXH => 5 XCVML".to_owned(),
        "3 BHXH, 2 VRPVC => 7 MZWV".to_owned(),
        "121 ORE => 7 VRPVC".to_owned(),
        "7 XCVML => 6 RJRHP".to_owned(),
        "5 BHXH, 4 VRPVC => 5 LTCX".to_owned(),
    ]);
}
