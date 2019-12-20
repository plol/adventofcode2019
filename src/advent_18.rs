pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        18
    }
    fn main1(input: &Vec<String>) -> String {
        let maze = Maze::parse(input);
        let len = find_shortest_path_length_to_collect_all_keys(&maze);
        format!("{}", len)
    }
    fn main2(input: &Vec<String>) -> String {
        let unaltered_maze = Maze::parse(&input);
        let pos = unaltered_maze.feature_positions[unaltered_maze.starting_positions.get(0)];
        let altered_input = blit(
            input.clone(),
            (pos.0 - 1, pos.1 - 1),
            &["@#@", "###", "@#@"],
        );
        let maze = Maze::parse(&altered_input);
        let len = find_shortest_path_length_to_collect_all_keys(&maze);
        format!("{}", len)
    }
}

type Pos = (usize, usize);

struct Maze {
    features: Vec<char>,
    feature_positions: Vec<Pos>,
    starting_positions: Poss,
    all_keys: Keys,
    distances: std::collections::HashMap<usize, std::collections::HashMap<Key, (Keys, usize)>>,
}

impl Maze {
    fn parse(input: &Vec<String>) -> Self {
        let map: Vec<Vec<char>> = input.iter().map(|x| x.chars().collect()).collect();

        let mut features = vec![];
        let mut feature_positions = vec![];
        for y in 0..map.len() {
            for x in 0..map[y].len() {
                let c = map[y][x];
                match c {
                    '@' => {
                        features.push(c);
                        feature_positions.push((x, y));
                    }
                    'a'..='z' | 'A'..='Z' => {
                        features.push(c);
                        feature_positions.push((x, y));
                    }
                    _ => {}
                }
            }
        }

        let mut matching_doors = vec![];
        let connections = flood_fill_all_features(&features, &feature_positions, &map);

        for i in 0..features.len() {
            if features[i].is_lowercase() {
                matching_doors.push(
                    features
                        .iter()
                        .position(|f| *f == features[i].to_uppercase().nth(0).unwrap())
                        .unwrap_or(features.len()),
                );
            } else {
                matching_doors.push(features.len());
            }
        }

        let starting_positions = features
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == '@')
            .map(|(i, _)| i)
            .fold(Poss::new(), |poss, pos| poss.insert(pos));

        let mut distances = std::collections::HashMap::new();

        let all_keys_in_maze = features
            .iter()
            .filter(|c| c.is_lowercase())
            .map(|c| Key::from_char(*c))
            .fold(Keys::new(), |keys, key| keys.insert(key));
        for key in all_keys_in_maze.into_iter() {
            distances.insert(
                Self::key_feature_impl(&features, key),
                distances_and_keys_to_all_keys(
                    Self::key_feature_impl(&features, key),
                    &features,
                    &connections,
                ),
            );
        }

        for start in starting_positions.into_iter() {
            distances.insert(
                start,
                distances_and_keys_to_all_keys(start, &features, &connections),
            );
        }

        Maze {
            features: features,
            feature_positions: feature_positions,
            starting_positions: starting_positions,
            all_keys: all_keys_in_maze,
            distances: distances,
        }
    }

    fn key_feature_impl(features: &Vec<char>, key: Key) -> usize {
        features.iter().position(|f| *f == key.as_char()).unwrap()
    }
    fn key_feature(&self, key: Key) -> usize {
        Self::key_feature_impl(&self.features, key)
    }
}

fn flood_fill_to_connected_features(
    pos: Pos,
    maze: &Vec<Vec<char>>,
) -> std::collections::HashMap<char, usize> {
    let mut found_features = vec![];
    let result = super::common::dijkstra::all(
        pos,
        |p| *p,
        |(x, y)| {
            match maze[y][x] {
                'a'..='z' | 'A'..='Z' => {
                    if (x, y) != pos {
                        found_features.push((x, y));
                        return vec![];
                    }
                }
                _ => {}
            }
            let mut candidates = vec![];
            if x > 0 {
                candidates.push((x - 1, y));
            }
            if x + 1 < maze[y].len() {
                candidates.push((x + 1, y));
            }
            if y > 0 {
                candidates.push((x, y - 1));
            }
            if y + 1 < maze.len() {
                candidates.push((x, y + 1));
            }
            candidates
                .iter()
                .map(|&(cx, cy)| {
                    let c = maze[cy][cx];
                    match c {
                        '#' => None,
                        _ => Some(((cx, cy), 1)),
                    }
                })
                .flatten()
                .collect::<Vec<_>>()
        },
    );

    found_features
        .iter()
        .map(|&(x, y)| (maze[y][x], result.get_cost(&(x, y)).unwrap()))
        .collect()
}

fn flood_fill_all_features(
    features: &Vec<char>,
    feature_positions: &Vec<Pos>,
    map: &Vec<Vec<char>>,
) -> Vec<Vec<(usize, usize)>> {
    let mut connections = vec![];
    for i in 0..features.len() {
        let a = flood_fill_to_connected_features(feature_positions[i], map);
        connections.push(
            a.iter()
                .filter(|(feature, _)| **feature != '@')
                .map(|(feature, cost)| (features.iter().position(|f| f == feature).unwrap(), *cost))
                .collect(),
        );
    }
    connections
}

fn distances_and_keys_to_all_keys(
    from_pos: usize,
    features: &Vec<char>,
    connections: &Vec<Vec<(usize, usize)>>,
) -> std::collections::HashMap<Key, (Keys, usize)> {
    let mut found_keys = vec![];
    let result = super::common::dijkstra::all(
        (from_pos, Keys::new()),
        |(p, _)| *p,
        |(p, mut keys)| {
            if features[p].is_uppercase() {
                keys = keys.insert(Key::from_door_char(features[p]));
            } else if features[p].is_lowercase() {
                found_keys.push((p, keys))
            }
            connections[p]
                .iter()
                .map(|&(conn, price)| Some(((conn, keys), price)))
                .flatten()
                .collect::<Vec<_>>()
        },
    );

    found_keys
        .iter()
        .map(|&(key_feature, keys)| {
            let cost = result.get_cost(&key_feature).unwrap();
            (Key::from_char(features[key_feature]), (keys, cost))
        })
        .collect()
}

fn distance_to_key(pos: usize, keys: Keys, key: Key, maze: &Maze) -> Option<usize> {
    if let Some(&(keys2, cost)) = maze.distances[&pos].get(&key) {
        if keys.contains_all(keys2) {
            Some(cost)
        } else {
            None
        }
    } else {
        None
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Copy)]
struct Key {
    repr: u32,
}

impl Key {
    fn as_number(&self) -> u8 {
        for i in 0..32 {
            if (self.repr & (1 << i)) != 0 {
                return i;
            }
        }
        panic!();
    }
    fn as_char(&self) -> char {
        ('a' as u8 + self.as_number() - 1) as char
    }
    fn as_door_char(&self) -> char {
        ('A' as u8 + self.as_number() - 1) as char
    }
    fn from_char(c: char) -> Self {
        Key {
            repr: 1 << (c as u8 - 'a' as u8 + 1),
        }
    }
    fn from_door_char(c: char) -> Self {
        Key {
            repr: 1 << (c as u8 - 'A' as u8 + 1),
        }
    }
    fn from_bit(b: u32) -> Self {
        Key { repr: b }
    }
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "Key{{{}}}", self.as_char())
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Copy)]
struct Keys {
    repr: u32,
}

impl Keys {
    fn new() -> Keys {
        Self { repr: 0 }
    }

    fn is_empty(&self) -> bool {
        self.repr == 0
    }

    fn contains(&self, key: Key) -> bool {
        (self.repr & key.repr) != 0
    }

    fn contains_all(&self, keys: Keys) -> bool {
        (self.repr & keys.repr) == keys.repr
    }

    fn insert(mut self, key: Key) -> Self {
        self.repr |= key.repr;
        self
    }
    fn remove(mut self, key: Key) -> Self {
        self.repr &= !key.repr;
        self
    }
}

impl std::fmt::Debug for Keys {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "Keys{{{}}}",
            self.into_iter().map(|k| k.as_char()).collect::<String>()
        )
    }
}

impl IntoIterator for &Keys {
    type Item = Key;
    type IntoIter = KeysIter;
    fn into_iter(self) -> Self::IntoIter {
        KeysIter { keys: *self, i: 0 }
    }
}

struct KeysIter {
    keys: Keys,
    i: usize,
}

impl Iterator for KeysIter {
    type Item = Key;

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < 32 {
            let k = Key::from_bit(1 << self.i);
            self.i += 1;
            if self.keys.contains(k) {
                return Some(k);
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Poss {
    repr: [u8; 4],
}
impl Poss {
    fn new() -> Self {
        Self {
            repr: [0xFF, 0xFF, 0xFF, 0xFF],
        }
    }

    fn insert(mut self, pos: usize) -> Self {
        assert!(pos < 0xFF);
        for i in 0..4 {
            if self.repr[i] == 0xFF {
                self.repr[i] = pos as u8;
                return self;
            }
        }
        panic!();
    }

    fn update(mut self, i: usize, pos: usize) -> Self {
        assert!(pos < 0xFF && i < 4);
        self.repr[i] = pos as u8;
        self
    }

    fn get(&self, i: usize) -> usize {
        self.repr[i] as usize
    }

    fn len(&self) -> usize {
        for i in 0..4 {
            if self.repr[i] == 0xFF {
                return i;
            }
        }
        4
    }
}

impl IntoIterator for &Poss {
    type Item = usize;
    type IntoIter = PossIter;
    fn into_iter(self) -> Self::IntoIter {
        PossIter { poss: *self, i: 0 }
    }
}

struct PossIter {
    poss: Poss,
    i: usize,
}

impl Iterator for PossIter {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.poss.len() {
            let ret = self.poss.get(self.i);
            self.i += 1;
            Some(ret)
        } else {
            None
        }
    }
}

fn dfs_for_keys_with_cache(
    poss: Poss,
    keys: Keys,
    remaining_keys: Keys,
    cache: &mut std::collections::HashMap<(Poss, Keys), usize>,
    maze: &Maze,
) -> usize {
    if remaining_keys.is_empty() {
        0
    } else {
        if let Some(&cost) = cache.get(&(poss, keys)) {
            cost
        } else {
            let mut min_cost: Option<usize> = None;
            for key in remaining_keys.into_iter() {
                for (i, pos) in poss.into_iter().enumerate() {
                    if let Some(distance) = distance_to_key(pos, keys, key, maze) {
                        let cost = distance
                            + dfs_for_keys_with_cache(
                                poss.update(i, maze.key_feature(key)),
                                keys.insert(key),
                                remaining_keys.remove(key),
                                cache,
                                maze,
                            );
                        min_cost = min_cost.map(|c| c.min(cost)).or(Some(cost));
                        break;
                    }
                }
            }
            cache.insert((poss, keys), min_cost.unwrap());
            min_cost.unwrap()
        }
    }
}

fn find_shortest_path_length_to_collect_all_keys(maze: &Maze) -> usize {
    let mut cache = std::collections::HashMap::new();

    let k = dfs_for_keys_with_cache(
        maze.starting_positions,
        Keys::new(),
        maze.all_keys,
        &mut cache,
        &maze,
    );
    k
}

pub fn test1() {
    let map2 = vec![
        "########################".chars().collect(),
        "#@..............ac.GI.b#".chars().collect(),
        "###d#e#f################".chars().collect(),
        "###A#B#C################".chars().collect(),
        "###g#h#i################".chars().collect(),
        "########################".chars().collect(),
    ];

    let maze = Maze::parse(&map2);

    let now = std::time::Instant::now();
    let path = find_shortest_path_length_to_collect_all_keys(&maze);
    println!(
        "{:?} {}",
        path,
        (std::time::Instant::now() - now).as_nanos() as f64 / 1000000.0
    );
}

pub fn test2() {
    let map2 = vec![
        "########################".chars().collect(),
        "#@..............ac.GI.b#".chars().collect(),
        "###d#e#f################".chars().collect(),
        "###A#B#C################".chars().collect(),
        "###g#h#i################".chars().collect(),
        "########################".chars().collect(),
    ];

    let map3 = blit(map2.clone(), (6, 1), &["#@"]);
    for row in &map3 {
        println!("{}", row);
    }
    let maze = Maze::parse(&map3);

    let now = std::time::Instant::now();
    let path = find_shortest_path_length_to_collect_all_keys(&maze);
    println!(
        "{:?} {}",
        path,
        (std::time::Instant::now() - now).as_nanos() as f64 / 1000000.0
    );
}

fn blit(mut data: Vec<String>, pos: Pos, to_blit: &[&str]) -> Vec<String> {
    for y in 0..to_blit.len() {
        data[pos.1 + y] = [
            &data[pos.1 + y][0..pos.0],
            to_blit[y],
            &data[pos.1 + y][pos.0 + to_blit[y].len()..data[pos.1 + y].len()],
        ]
        .iter()
        .map(|x| *x)
        .collect::<String>();
    }
    data
}
