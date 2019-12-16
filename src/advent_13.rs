pub struct Advent;
impl super::common::Advent for Advent {
    fn advent_number() -> u8 {
        13
    }
    fn main1(input: &Vec<String>) -> String {
        let initial_mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        let outputs = OmegaIterator {
            inner: &mut intcode::run_intcode_with_inputs_and_iterate_over_outputs(
                initial_mem,
                vec![].iter().map(|x| *x),
            ),
        }
        .collect::<Vec<[i64; 3]>>();
        format!(
            "{:?}",
            outputs
                .iter()
                .map(|tile| tile[2] == 2)
                .filter(|a| *a)
                .count()
        )
    }
    fn main2(input: &Vec<String>) -> String {
        let mut initial_mem: Vec<i64> = input
            .join("")
            .split(|c| c == ',')
            .map(|x| x.parse().unwrap())
            .collect();
        initial_mem[0] = 2;

        let game_state = std::rc::Rc::new(std::sync::Mutex::new((0, vec![])));

        let mut input = GameInputAndRenderIterator {
            game_state: game_state.clone(),
        };
        let mut outputs = OmegaIterator {
            inner: &mut intcode::run_intcode_with_inputs_and_iterate_over_outputs(
                initial_mem,
                &mut input,
            ),
        };

        for output in &mut outputs {
            let mut uhhh = game_state.lock().unwrap();
            match output {
                [-1, _, score] => uhhh.0 = score,
                [x, y, tile] => {
                    let length = if uhhh.1.len() > 0 { uhhh.1[0].len() } else { 0 };
                    while y as usize >= uhhh.1.len() {
                        uhhh.1.push(vec![0; length]);
                    }
                    if x as usize >= length {
                        for row in &mut uhhh.1 {
                            for _ in x as usize..(length + 1) {
                                row.push(0);
                            }
                        }
                    }
                    uhhh.1[y as usize][x as usize] = tile;
                }
            }
        }

        //render_game_state(&game_state.lock().unwrap());
        let score = game_state.lock().unwrap().0;
        format!("{}", score)
    }
}
use super::intcode;

struct OmegaIterator<'a, I>
where
    I: Iterator,
{
    inner: &'a mut I,
}
impl<'a, I, T> Iterator for &'a mut OmegaIterator<'a, I>
where
    I: Iterator<Item = T>,
{
    type Item = [T; 3];
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = self.inner.next() {
            Some([x, self.inner.next().unwrap(), self.inner.next().unwrap()])
        } else {
            None
        }
    }
}

type Game = (i64, Vec<Vec<i64>>);

fn render_game_state(game: &Game) {
    println!("Score: {}", game.0);
    for row in &game.1 {
        println!(
            "{}",
            row.iter()
                .map(|c| match c {
                    0 => ' ',
                    1 => '#',
                    2 => '=',
                    3 => '-',
                    4 => '*',
                    _ => panic!(),
                })
                .collect::<String>()
        );
    }
}

fn find_ball_position(game: &Game) -> (i64, i64) {
    for y in 0..game.1.len() {
        for x in 0..game.1[0].len() {
            if game.1[y][x] == 4 {
                return (x as i64, y as i64);
            }
        }
    }
    panic!();
}

fn find_paddle_position(game: &Game) -> (i64, i64) {
    for y in 0..game.1.len() {
        for x in 0..game.1[0].len() {
            if game.1[y][x] == 3 {
                return (x as i64, y as i64);
            }
        }
    }
    panic!();
}

struct GameInputAndRenderIterator {
    game_state: std::rc::Rc<std::sync::Mutex<Game>>,
}

impl Iterator for &mut GameInputAndRenderIterator {
    type Item = i64;
    fn next(&mut self) -> Option<Self::Item> {
        let uhhh = self.game_state.lock().unwrap();
        //render_game_state(&uhhh);
        let ball_now = find_ball_position(&uhhh);
        let paddle_now = find_paddle_position(&uhhh);
        Some((ball_now.0 - paddle_now.0).signum())
    }
}
