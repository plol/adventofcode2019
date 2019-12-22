pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        11
    }
    fn main1(input: &Vec<String>) -> String {
        gogo(input, false, 0)
    }
    fn main2(input: &Vec<String>) -> String {
        gogo(input, false, 1);
        "HGEHJHUZ".to_owned() // visual inspection
    }
}
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

pub fn gogo(input: &Vec<String>, print_result: bool, first_input: i64) -> String {
    let mem = input
        .join("")
        .split(|c| c == ',')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut painted_squares = std::collections::HashMap::<(i64, i64), i64>::new();

    let mut robot_brain = intcode::IntcodeComputer::new(mem);
    let mut robot_movement_state = RobotMovementState::JustMoved;
    let mut robot_heading = (0, 1);
    let mut current_robot_pos = (0, 0);

    robot_brain.start();

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
                        .unwrap_or(if is_first_input { &first_input } else { &0 }),
                );
                is_first_input = false;
            }
            intcode::IntcodeState::Output => {
                let x = robot_brain.consume_output();
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
            }
            intcode::IntcodeState::Halt => {
                break;
            }
            _ => panic!(),
        }
    }

    if print_result {
        render(&painted_squares);
    }
    format!("{:?}", painted_squares.len())
}
