pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        6
    }
    fn main1(input: &Vec<String>) -> String {
        self::main1(input)
    }
    fn main2(input: &Vec<String>) -> String {
        self::main2(input)
    }
}
fn main1(input: &Vec<String>) -> String {
    //println!("{:?}", map);
    let mut g = OrbitGraph { bodies: vec![] };
    for d in input {
        let a: Vec<&str> = d.split(")").collect();
        g.push_orbit(a[0], a[1]);
    }
    //println!("{:?}", g);
    format!("{:?}", g.sum_orbits("COM"))
}
fn main2(input: &Vec<String>) -> String {
    let mut g = OrbitGraph { bodies: vec![] };
    for d in input {
        let a: Vec<&str> = d.split(")").collect();
        g.push_orbit(a[0], a[1]);
    }
    format!(
        "{:?}",
        g.orbit_traversal_length(g.get_orbit("YOU").unwrap(), g.get_orbit("SAN").unwrap())
            .unwrap()
            - 2
    )
}
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
            //println!(
            //    "{:?}",
            //    bfs_state
            //        .iter()
            //        .map(|x| &self.bodies[*x].body_name)
            //        .collect::<Vec<&String>>()
            //);
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
pub fn test() {
    main1(&vec![
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
pub fn test2() {
    main2(&vec![
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
