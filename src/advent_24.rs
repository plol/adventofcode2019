pub struct Advent;

impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        24
    }
    fn main1(input: &Vec<String>) -> String {
        format!("{}", find_first_repetition(parse(input)))
    }

    fn main2(input: &Vec<String>) -> String {
        format!(
            "{}",
            count_bugs(
                (0..200)
                    .fold((0, vec![parse(input)]), |(min_layer, layers), _| {
                        evolve_all_layers(min_layer, layers)
                    })
                    .1
            )
        )
    }
}

fn count_bugs(layers: Vec<u32>) -> usize {
    let mut count = 0;
    for board in layers {
        for i in 0..25 {
            if (board & (1 << i)) != 0 {
                count += 1;
            }
        }
    }
    count
}

fn find_first_repetition(mut board: u32) -> u32 {
    let mut seen = std::collections::HashSet::new();
    loop {
        if seen.contains(&board) {
            return board;
        }
        seen.insert(board);
        board = evolve1(board, 0);
    }
}

fn evolve1(board: u32, outer: u32) -> u32 {
    let bl = left_of(board, outer);
    let br = right_of(board, outer);
    let ba = above_of(board, outer);
    let bb = below_of(board, outer);
    (bl & !br & !ba & !bb)
        | (!bl & br & !ba & !bb)
        | (!bl & !br & ba & !bb)
        | (!bl & !br & !ba & bb)
        | (!board & bl & br & !ba & !bb)
        | (!board & bl & !br & ba & !bb)
        | (!board & bl & !br & !ba & bb)
        | (!board & !bl & br & ba & !bb)
        | (!board & !bl & br & !ba & bb)
        | (!board & !bl & !br & ba & bb)
}

fn left_of(board: u32, outer: u32) -> u32 {
    let mask =
        (outer & 0b_00000_00000_00010_00000_00000 != 0) as u32 * 0b_00001_00001_00001_00001_00001;
    ((board << 1) & 0b_11110_11110_11110_11110_11110) | mask
}
fn right_of(board: u32, outer: u32) -> u32 {
    let mask =
        (outer & 0b_00000_00000_01000_00000_00000 != 0) as u32 * 0b_10000_10000_10000_10000_10000;
    ((board >> 1) & 0b_01111_01111_01111_01111_01111) | mask
}
fn above_of(board: u32, outer: u32) -> u32 {
    let mask =
        (outer & 0b_00000_00000_00000_00100_00000 != 0) as u32 * 0b_00000_00000_00000_00000_11111;
    ((board << 5) & 0b_11111_11111_11111_11111_00000) | mask
}
fn below_of(board: u32, outer: u32) -> u32 {
    let mask =
        (outer & 0b_00000_00100_00000_00000_00000 != 0) as u32 * 0b_11111_00000_00000_00000_00000;
    ((board >> 5) & 0b_00000_11111_11111_11111_11111) | mask
}

fn evolve2(board: u32, inner: u32) -> u32 {
    let board_top = (board >> 7) & 1;
    let board_left = (board >> 11) & 1;
    let board_right = (board >> 13) & 1;
    let board_bottom = (board >> 17) & 1;
    let top_neighbors = ((inner >> 0) & 1)
        + ((inner >> 1) & 1)
        + ((inner >> 2) & 1)
        + ((inner >> 3) & 1)
        + ((inner >> 4) & 1)
        + ((board >> 8) & 1)
        + ((board >> 6) & 1)
        + ((board >> 2) & 1);
    let left_neighbors = ((inner >> 0) & 1)
        + ((inner >> 5) & 1)
        + ((inner >> 10) & 1)
        + ((inner >> 15) & 1)
        + ((inner >> 20) & 1)
        + ((board >> 6) & 1)
        + ((board >> 10) & 1)
        + ((board >> 16) & 1);
    let bottom_neighbors = ((inner >> 20) & 1)
        + ((inner >> 21) & 1)
        + ((inner >> 22) & 1)
        + ((inner >> 23) & 1)
        + ((inner >> 24) & 1)
        + ((board >> 16) & 1)
        + ((board >> 22) & 1)
        + ((board >> 18) & 1);
    let right_neighbors = ((inner >> 4) & 1)
        + ((inner >> 9) & 1)
        + ((inner >> 14) & 1)
        + ((inner >> 19) & 1)
        + ((inner >> 24) & 1)
        + ((board >> 18) & 1)
        + ((board >> 14) & 1)
        + ((board >> 8) & 1);
    ((top_neighbors == 1 || board_top == 0 && top_neighbors == 2) as u32
        * 0b_00000_00000_00000_00100_00000)
        | (left_neighbors == 1 || board_left == 0 && left_neighbors == 2) as u32
            * 0b_00000_00000_00010_00000_00000
        | (bottom_neighbors == 1 || board_bottom == 0 && bottom_neighbors == 2) as u32
            * 0b_00000_00100_00000_00000_00000
        | (right_neighbors == 1 || board_right == 0 && right_neighbors == 2) as u32
            * 0b_00000_00000_01000_00000_00000
}

fn evolve_with_layers(board: u32, outer: u32, inner: u32) -> u32 {
    (evolve1(board, outer) & 0b_11111_11011_10001_11011_11111) | evolve2(board, inner)
}

fn evolve_all_layers(min_layer: i32, layers: Vec<u32>) -> (i32, Vec<u32>) {
    let mut ret = vec![];
    let mut next_min_layer = min_layer;
    let tentative_next_min_layer = evolve_with_layers(0, 0, layers[0]);
    if tentative_next_min_layer != 0 {
        ret.push(tentative_next_min_layer);
        next_min_layer = min_layer - 1;
    }

    for i in 0..layers.len() {
        let next = evolve_with_layers(
            layers[i],
            if i > 0 { layers[i - 1] } else { 0 },
            if i + 1 < layers.len() {
                layers[i + 1]
            } else {
                0
            },
        );
        ret.push(next);
    }
    let tentative_next_max_layer = evolve_with_layers(0, layers[layers.len() - 1], 0);
    if tentative_next_max_layer != 0 {
        ret.push(tentative_next_max_layer);
    }
    (next_min_layer, ret)
}

fn parse(board_strs: &Vec<String>) -> u32 {
    board_strs
        .join("")
        .bytes()
        .enumerate()
        .map(|(i, b)| if b == b'#' { 1 } else { 0 } << i)
        .fold(0, |r, b| r | b)
}
