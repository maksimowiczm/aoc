#[derive(Clone)]
enum CellInfo {
    Valid,
    Invalid,
    Number,
}

pub struct Board {
    board: Vec<Vec<char>>,
}

impl From<&str> for Board {
    fn from(value: &str) -> Self {
        Board {
            board: value
                .split("\n")
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect(),
        }
    }
}

impl Board {
    pub fn valid_part_numbers(&self) -> Vec<u32> {
        let info = self.board_info().unwrap();

        info.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, cell)| match *cell {
                        CellInfo::Number => self.board[y][x],
                        _ => ' ',
                    })
                    .collect::<String>()
            })
            .map(|line| {
                line.trim()
                    .split(" ")
                    .map(|str| str.parse::<u32>())
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect()
    }

    fn validate(valid_table: &mut Vec<Vec<CellInfo>>, (x, y): (usize, usize)) {
        // invalidate cell if not None
        let mut validate_cell = |x: usize, y: usize| {
            if let Some(line) = &mut valid_table.get_mut(y) {
                if let Some(cell) = &mut line.get_mut(x) {
                    **cell = CellInfo::Valid;
                }
            }
        };

        // invalidate cell and cells around the cell
        for i in -1..2 {
            for j in -1..2 {
                validate_cell((x as i32 + i) as usize, (y as i32 + j) as usize);
            }
        }
    }

    fn board_info(&self) -> Option<Vec<Vec<CellInfo>>> {
        let x_size = self.board.first()?.len();
        let y_size = self.board.len();

        let mut info = vec![vec![CellInfo::Invalid; x_size]; y_size];

        self.board.iter().enumerate().for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, ch)| !ch.is_digit(10) && **ch != '.')
                .for_each(|(x, _)| Board::validate(&mut info, (x, y)))
        });

        for y in 0..y_size {
            for x in 0..x_size {
                let valid_cell = info.get(y)?.get(x)?;
                match *valid_cell {
                    CellInfo::Valid => {
                        // set cells as numbers
                        let mut set_number = |i: usize| -> Option<()> {
                            let cell = self.board.get(y)?.get(i)?;
                            if cell.is_digit(10) {
                                *info.get_mut(y)?.get_mut(i)? = CellInfo::Number;
                            } else {
                                return None;
                            }

                            return Some(());
                        };
                        // right
                        for i in x..x_size {
                            match set_number(i) {
                                None => break,
                                _ => (),
                            }
                        }
                        // left
                        for i in (0..x + 1).rev() {
                            match set_number(i) {
                                None => break,
                                _ => (),
                            }
                        }
                    }
                    _ => (),
                }
            }
        }

        Some(info)
    }
}
