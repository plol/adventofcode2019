pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        20
    }
    fn main1(input: &Vec<String>) -> String {
        let parsed = parse(&input);

        format!("{}", find_shortest_path(&parsed).len() - 1)
    }

    fn main2(input: &Vec<String>) -> String {
        let parsed = parse(&input);

        format!("{}", find_shortest_path2(&parsed).len() - 1)
    }
}
type Pos = (usize, usize);
struct Maze {
    start_pos: Pos,
    end_pos: Pos,
    portals: std::collections::HashMap<Pos, (Pos, bool)>,
    tiles: Vec<Vec<char>>,
}

fn parse(maze: &Vec<String>) -> Maze {
    let tiles = maze
        .iter()
        .map(|row| row.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let mut features = std::collections::HashMap::<String, Vec<(usize, usize, i64, i64)>>::new();

    for y in 0..tiles.len() - 4 {
        for x in 0..tiles[y].len() - 4 {
            match tiles[y + 2][x + 2] {
                '.' => {
                    for &(px1, py1, px2, py2, dx, dy) in [
                        (x + 3, y + 2, x + 4, y + 2, 1, 0),
                        (x + 0, y + 2, x + 1, y + 2, -1, 0),
                        (x + 2, y + 3, x + 2, y + 4, 0, 1),
                        (x + 2, y + 0, x + 2, y + 1, 0, -1),
                    ]
                    .iter()
                    {
                        if ('A'..='Z').contains(&tiles[py1][px1])
                            && ('A'..='Z').contains(&tiles[py2][px2])
                        {
                            let portal_name = [tiles[py1][px1], tiles[py2][px2]]
                                .iter()
                                .collect::<String>();
                            features.entry(portal_name).or_insert(vec![]).push((
                                x + 2,
                                y + 2,
                                dx,
                                dy,
                            ));
                        }
                    }
                }
                _ => {}
            }
        }
    }

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut portals = std::collections::HashMap::new();
    for (feature, positions) in features {
        match feature.as_ref() {
            "AA" => start_pos = (positions[0].0, positions[0].1),
            "ZZ" => end_pos = (positions[0].0, positions[0].1),
            _ => {
                let (x1, y1, dx1, dy1) = positions[0];
                let (x2, y2, dx2, dy2) = positions[1];

                let is_outer_layer1 =
                    x1 < 4 || tiles[0].len() - x1 < 4 || y1 < 4 || tiles.len() - y1 < 4;
                let is_outer_layer2 =
                    x2 < 4 || tiles[0].len() - x2 < 4 || y2 < 4 || tiles.len() - y2 < 4;

                portals.insert(
                    ((x1 as i64 + dx1) as usize, (y1 as i64 + dy1) as usize),
                    ((x2, y2), is_outer_layer1),
                );
                portals.insert(
                    ((x2 as i64 + dx2) as usize, (y2 as i64 + dy2) as usize),
                    ((x1, y1), is_outer_layer2),
                );
            }
        }
    }

    Maze {
        start_pos: start_pos,
        end_pos: end_pos,
        portals: portals,
        tiles: tiles,
    }
}

fn find_shortest_path(maze: &Maze) -> Vec<Pos> {
    super::common::dijkstra::search(
        maze.start_pos,
        &maze.end_pos,
        |p| *p,
        |(x, y)| match maze.tiles[y][x] {
            '.' => [
                check_for_portals((x - 1, y), maze, 1).map(|(pos, _)| (pos, 1)),
                check_for_portals((x + 1, y), maze, 1).map(|(pos, _)| (pos, 1)),
                check_for_portals((x, y - 1), maze, 1).map(|(pos, _)| (pos, 1)),
                check_for_portals((x, y + 1), maze, 1).map(|(pos, _)| (pos, 1)),
            ]
            .iter()
            .map(|a| *a)
            .flatten()
            .collect::<Vec<(Pos, usize)>>(),
            _ => vec![],
        },
    )
    .unwrap()
}

fn find_shortest_path2(maze: &Maze) -> Vec<(Pos, usize)> {
    super::common::dijkstra::search(
        (maze.start_pos, 0),
        &(maze.end_pos, 0),
        |p| *p,
        |((x, y), layer)| match maze.tiles[y][x] {
            '.' => [
                check_for_portals((x - 1, y), maze, layer).map(|pos| (pos, 1)),
                check_for_portals((x + 1, y), maze, layer).map(|pos| (pos, 1)),
                check_for_portals((x, y - 1), maze, layer).map(|pos| (pos, 1)),
                check_for_portals((x, y + 1), maze, layer).map(|pos| (pos, 1)),
            ]
            .iter()
            .map(|a| *a)
            .flatten()
            .collect::<Vec<((Pos, usize), usize)>>(),
            _ => vec![],
        },
    )
    .unwrap()
}

fn check_for_portals((x, y): Pos, maze: &Maze, layer: usize) -> Option<(Pos, usize)> {
    if let Some(&((px, py), is_outer_layer)) = maze.portals.get(&(x, y)) {
        if is_outer_layer {
            if layer == 0 {
                None
            } else {
                Some(((px, py), layer - 1))
            }
        } else {
            Some(((px, py), layer + 1))
        }
    } else {
        Some(((x, y), layer))
    }
}

pub fn test1() {
    let maze = vec![
        "                   A               ".to_owned(),
        "                   A               ".to_owned(),
        "  #################.#############  ".to_owned(),
        "  #.#...#...................#.#.#  ".to_owned(),
        "  #.#.#.###.###.###.#########.#.#  ".to_owned(),
        "  #.#.#.......#...#.....#.#.#...#  ".to_owned(),
        "  #.#########.###.#####.#.#.###.#  ".to_owned(),
        "  #.............#.#.....#.......#  ".to_owned(),
        "  ###.###########.###.#####.#.#.#  ".to_owned(),
        "  #.....#        A   C    #.#.#.#  ".to_owned(),
        "  #######        S   P    #####.#  ".to_owned(),
        "  #.#...#                 #......VT".to_owned(),
        "  #.#.#.#                 #.#####  ".to_owned(),
        "  #...#.#               YN....#.#  ".to_owned(),
        "  #.###.#                 #####.#  ".to_owned(),
        "DI....#.#                 #.....#  ".to_owned(),
        "  #####.#                 #.###.#  ".to_owned(),
        "ZZ......#               QG....#..AS".to_owned(),
        "  ###.###                 #######  ".to_owned(),
        "JO..#.#.#                 #.....#  ".to_owned(),
        "  #.#.#.#                 ###.#.#  ".to_owned(),
        "  #...#..DI             BU....#..LF".to_owned(),
        "  #####.#                 #.#####  ".to_owned(),
        "YN......#               VT..#....QG".to_owned(),
        "  #.###.#                 #.###.#  ".to_owned(),
        "  #.#...#                 #.....#  ".to_owned(),
        "  ###.###    J L     J    #.#.###  ".to_owned(),
        "  #.....#    O F     P    #.#...#  ".to_owned(),
        "  #.###.#####.#.#####.#####.###.#  ".to_owned(),
        "  #...#.#.#...#.....#.....#.#...#  ".to_owned(),
        "  #.#####.###.###.#.#.#########.#  ".to_owned(),
        "  #...#.#.....#...#.#.#.#.....#.#  ".to_owned(),
        "  #.###.#####.###.###.#.#.#######  ".to_owned(),
        "  #.#.........#...#.............#  ".to_owned(),
        "  #########.###.###.#############  ".to_owned(),
        "           B   J   C               ".to_owned(),
        "           U   P   P               ".to_owned(),
    ];

    let parsed = parse(&maze);

    let path = find_shortest_path(&parsed);

    println!("path: {:?}", path);
    println!("portals: {:?}", parsed.portals);
    println!("start_pos: {:?}", parsed.start_pos);
    println!("end_pos: {:?}", parsed.end_pos);
    render_maze_yall(&parsed, Some(&path));

    println!("{}", find_shortest_path(&parsed).len());
}

pub fn test2() {
    let input = vec![
        "             Z L X W       C                 ".to_owned(),
        "             Z P Q B       K                 ".to_owned(),
        "  ###########.#.#.#.#######.###############  ".to_owned(),
        "  #...#.......#.#.......#.#.......#.#.#...#  ".to_owned(),
        "  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  ".to_owned(),
        "  #.#...#.#.#...#.#.#...#...#...#.#.......#  ".to_owned(),
        "  #.###.#######.###.###.#.###.###.#.#######  ".to_owned(),
        "  #...#.......#.#...#...#.............#...#  ".to_owned(),
        "  #.#########.#######.#.#######.#######.###  ".to_owned(),
        "  #...#.#    F       R I       Z    #.#.#.#  ".to_owned(),
        "  #.###.#    D       E C       H    #.#.#.#  ".to_owned(),
        "  #.#...#                           #...#.#  ".to_owned(),
        "  #.###.#                           #.###.#  ".to_owned(),
        "  #.#....OA                       WB..#.#..ZH".to_owned(),
        "  #.###.#                           #.#.#.#  ".to_owned(),
        "CJ......#                           #.....#  ".to_owned(),
        "  #######                           #######  ".to_owned(),
        "  #.#....CK                         #......IC".to_owned(),
        "  #.###.#                           #.###.#  ".to_owned(),
        "  #.....#                           #...#.#  ".to_owned(),
        "  ###.###                           #.#.#.#  ".to_owned(),
        "XF....#.#                         RF..#.#.#  ".to_owned(),
        "  #####.#                           #######  ".to_owned(),
        "  #......CJ                       NM..#...#  ".to_owned(),
        "  ###.#.#                           #.###.#  ".to_owned(),
        "RE....#.#                           #......RF".to_owned(),
        "  ###.###        X   X       L      #.#.#.#  ".to_owned(),
        "  #.....#        F   Q       P      #.#.#.#  ".to_owned(),
        "  ###.###########.###.#######.#########.###  ".to_owned(),
        "  #.....#...#.....#.......#...#.....#.#...#  ".to_owned(),
        "  #####.#.###.#######.#######.###.###.#.#.#  ".to_owned(),
        "  #.......#.......#.#.#.#.#...#...#...#.#.#  ".to_owned(),
        "  #####.###.#####.#.#.#.#.###.###.#.###.###  ".to_owned(),
        "  #.......#.....#.#...#...............#...#  ".to_owned(),
        "  #############.#.#.###.###################  ".to_owned(),
        "               A O F   N                     ".to_owned(),
        "               A A D   M                     ".to_owned(),
    ];

    let maze = parse(&input);

    render_maze_yall(&maze, None);

    println!("{:?}", find_shortest_path2(&maze));
    println!("{:?}", find_shortest_path2(&maze).len());
}

fn render_maze_yall(maze: &Maze, path_or_no: Option<&Vec<Pos>>) {
    let mut tiles = maze.tiles.clone();
    if let Some(path) = path_or_no {
        for &(x, y) in path {
            tiles[y][x] = '*';
        }
    }

    for ((x, y), (_p2, is_outer_layer)) in &maze.portals {
        tiles[*y][*x] = if *is_outer_layer { 'O' } else { 'I' };
    }

    for row in tiles {
        println!("{}", row.iter().collect::<String>());
    }
}
