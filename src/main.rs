use std::{
    env,
    fmt::{Debug, Display},
    io::{self, BufRead},
};

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Piece {
    name: String,
    blocks: Vec<Point>,
}

impl Piece {
    fn new(blocks: &[(i32, i32)], name: &str) -> Piece {
        Piece {
            name: name.to_string(),
            blocks: blocks.iter().map(|(x, y)| Point { x: *x, y: *y }).collect(),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

type Row = u32;

struct Board {
    rows: Vec<Row>,
    width: usize,
}

impl Board {
    fn new() -> Board {
        Board {
            rows: Vec::new(),
            width: 7,
        }
    }

    fn can_hold(&self, piece: &Piece, at: &Point) -> bool {
        for block in piece.blocks.iter() {
            let x = at.x + block.x;
            let y = at.y + block.y;

            if x < 0 {
                return false;
            }
            if x >= self.width as i32 {
                return false;
            }

            if y >= self.rows.len() as i32 {
                continue;
            }

            if y < 0 {
                return false;
            }

            if self.rows[y as usize] & (1 << x) != 0 {
                return false;
            }
        }
        true
    }

    fn apply(&mut self, piece: &Piece, at: &Point) {
        for block in piece.blocks.iter() {
            let x = at.x + block.x;
            let y = at.y + block.y;

            if x < 0 {
                continue;
            }
            if x >= self.width as i32 {
                continue;
            }

            if y >= self.rows.len() as i32 {
                self.rows
                    .extend([0].repeat(1 + y as usize - self.rows.len()))
            }

            self.rows[y as usize] |= 1 << x;
        }
    }

    fn show(&self) {
        for row in self.rows.iter().rev() {
            let mut text = String::with_capacity(self.width);
            for x in 0..self.width {
                if row & (1 << x) != 0 {
                    text.push('#');
                } else {
                    text.push('.');
                }
            }
            println!("{}", text);
        }
    }
}

fn parse_input(lines: &[String]) -> String {
    let mut s = "".to_string();
    for line in lines {
        s.push_str(line);
    }
    s
}

#[derive(Debug)]
struct CycleBufferRow {
    input_count: usize,
    piece_count: usize,
    board_height: usize,
    row: u32,
}

fn solution(input: &str, rock_count: usize) -> u32 {
    let pieces = vec![
        Piece::new(&[(0, 0), (1, 0), (2, 0), (3, 0)], "-"),
        Piece::new(&[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)], "+"),
        Piece::new(&[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)], "┘"),
        Piece::new(&[(0, 0), (0, 1), (0, 2), (0, 3)], "|"),
        Piece::new(&[(0, 0), (1, 0), (0, 1), (1, 1)], "▪"),
    ];

    let mut board = Board::new();
    let mut piece_pos = Point {
        x: 2,
        y: board.rows.len() as i32 + 3,
    };
    let mut count = 1;
    let mut input_iter = input.chars().cycle();
    let mut piece_iter = pieces.iter().cycle();
    let mut piece = piece_iter.next().unwrap();
    let mut iteration = 0;

    let mut cycle_buf: Vec<CycleBufferRow> = vec![];

    loop {
        let c = input_iter.next().unwrap();
        let mut new_pos = piece_pos.clone();

        // first, try to move it to the sides
        if c == '>' {
            new_pos.x += 1
        } else {
            new_pos.x -= 1
        };
        if board.can_hold(piece, &new_pos) {
            piece_pos = new_pos.clone();
        }
        // then, try to move it down
        new_pos = piece_pos.clone();
        new_pos.y -= 1;
        if board.can_hold(piece, &new_pos) {
            piece_pos = new_pos.clone();
        } else {
            board.apply(piece, &piece_pos);
            // println!("new piece at iteration {}", iteration);
            if count >= rock_count {
                break;
            }
            // if let Some(127) = board.rows.last() {
            //     println!(
            //         "full line: iteration={} board_height={} pieces={};",
            //         iteration % input.len(),
            //         board.rows.len(),
            //         count
            //     );
            //     // board.show();
            // }
            // try to detect a cycle

            let cycle_length = input.len() * pieces.len();
            let cycle_row = CycleBufferRow {
                input_count: iteration,
                piece_count: count,
                board_height: board.rows.len(),
                row: *board.rows.last().unwrap(),
            };
            if cycle_buf.len() >= cycle_length {

                let c = &cycle_buf[cycle_buf.len() - cycle_length];

                println!("cycle check {:?} to {:?}", c, cycle_row);
                if c.input_count % input.len() == iteration % input.len()
                    && c.piece_count % pieces.len() == count % pieces.len()
                    && c.row == *board.rows.last().unwrap()
                {
                    println!("cycle detected! from {:?} to {:?}", c, cycle_row);
                }
            }

            cycle_buf.push(cycle_row);
            // println!("iteration={} input.len()={}", iteration, input.len());
            if iteration > 0 && iteration % input.len() == 0
            //     && board.rows.len() > 2
            //     && board.rows.first().unwrap() == board.rows.last().unwrap()
            {
                println!(
                    "{:b} input.len()={} iteration={} board_height={} pieces={} last_piece={};",
                    board.rows.last().unwrap_or(&0),
                    input.len(),
                    iteration,
                    board.rows.len(),
                    count,
                    piece,
                );
                //     println!("board.rows.first(): {}", board.rows.first().unwrap());
                //     println!("board.rows.last(): {}", board.rows.last().unwrap());
                //     // println!("repeat at iteration: {}", iteration);
            }
            piece = piece_iter.next().unwrap();
            piece_pos = Point {
                x: 2,
                y: board.rows.len() as i32 + 3,
            };
            count += 1;
        }

        iteration += 1;
    }
    // board.show();
    board.rows.len() as u32
}

fn solve(lines: &[String]) -> u32 {
    let c: usize = env::args().collect::<Vec<String>>()[1].parse().unwrap();
    solution(&parse_input(lines), c)
}

fn main() {
    let lines = io::stdin().lock().lines();
    let lines: Vec<String> = lines.map(|line| line.unwrap()).collect();
    println!("{}", solve(&lines));
}

#[cfg(test)]
mod tests {
    use std::{fs::File, io::BufReader};

    use super::*;

    fn test_file(filename: &str, solution: &str) {
        let reader = BufReader::new(File::open(filename).unwrap());

        let lines: Vec<String> = reader
            .lines()
            .map(|x| x.unwrap().trim().to_string())
            .filter(|x| !x.is_empty())
            .collect();
        assert_eq!(solve(&lines).to_string(), solution);
    }

    #[test]
    fn test_example() {
        test_file("example.txt", "1651");
    }

    #[test]
    fn test_input() {
        test_file("input.txt", "1741");
    }
}
