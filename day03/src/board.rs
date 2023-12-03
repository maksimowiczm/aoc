#[derive(Clone)]
enum CellValidType {
    Default,
    Gear,
}

#[derive(Clone)]
enum CellInfo {
    Valid {
        id: usize,
        cell_type: CellValidType,
    },
    Invalid,
    Number {
        id: Option<usize>,
        cell_type: CellValidType,
    },
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
    pub fn valid_part_numbers_01(&self) -> Vec<u32> {
        let info = self.board_info().unwrap();

        info.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, cell)| match *cell {
                        CellInfo::Number { .. } => self.board[y][x],
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

    pub fn valid_part_numbers_02(&self) -> Vec<u32> {
        let info = self.board_info().unwrap();

        vec![]
    }

    fn validate(
        valid_table: &mut Vec<Vec<CellInfo>>,
        (x, y): (usize, usize),
        cell_type: CellValidType,
        gear_id: usize,
    ) {
        // invalidate cell if not None
        let mut validate_cell = |x: usize, y: usize| {
            if let Some(line) = &mut valid_table.get_mut(y) {
                if let Some(cell) = &mut line.get_mut(x) {
                    **cell = CellInfo::Valid {
                        id: gear_id,
                        cell_type: cell_type.clone(),
                    };
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

    fn spread_number(
        &self,
        valid_table: &mut Vec<Vec<CellInfo>>,
        (x, y): (usize, usize),
        cell_type: CellValidType,
    ) {
        let x_size = self.board.first().unwrap().len();

        // set cells as numbers
        let c_type = cell_type.clone();
        let mut set_number = |i: usize| -> Option<()> {
            let cell = self.board.get(y)?.get(i)?;
            if cell.is_digit(10) {
                *valid_table.get_mut(y)?.get_mut(i)? = CellInfo::Number {
                    id: None,
                    cell_type: c_type.clone(),
                };
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

    fn board_info(&self) -> Option<Vec<Vec<CellInfo>>> {
        let x_size = self.board.first()?.len();
        let y_size = self.board.len();

        let mut info = vec![vec![CellInfo::Invalid; x_size]; y_size];
        let mut gears_count = 0;

        self.board.iter().enumerate().for_each(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, ch)| !ch.is_digit(10) && **ch != '.')
                .for_each(|(x, ch)| {
                    let cell_type = if *ch == '*' {
                        CellValidType::Gear
                    } else {
                        CellValidType::Default
                    };

                    Board::validate(&mut info, (x, y), cell_type, gears_count);
                    gears_count += 1;
                })
        });

        for y in 0..y_size {
            for x in 0..x_size {
                let valid_cell = info.get(y)?.get(x)?.clone();
                match valid_cell {
                    CellInfo::Valid { cell_type, .. } => {
                        self.spread_number(&mut info, (x, y), cell_type.clone())
                    }
                    _ => (),
                }
            }
        }

        Some(info)
    }
}
