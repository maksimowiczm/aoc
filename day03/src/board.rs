#[derive(Clone)]
enum CellValidType {
    Default,
    Gear { id: usize },
}

#[derive(Clone)]
enum CellInfo {
    Valid { cell_type: CellValidType },
    Invalid,
    Number { cell_type: CellValidType },
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
    pub fn valid_parts(&self) -> Vec<u32> {
        let (info, _) = self.board_info().unwrap();

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

    pub fn valid_gears(&self) -> Vec<u32> {
        let (info, gears_count) = self.board_info_with_gears().unwrap();

        // 💀☠💀☠💀☠💀☠💀☠💀☠💀☠💀☠
        // 🙏
        info.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, cell)| match cell {
                        CellInfo::Number { cell_type } => match cell_type {
                            CellValidType::Gear { id } => (Some(*id), self.board[y][x]),
                            _ => (None, ' '),
                        },
                        _ => (None, ' '),
                    })
                    .collect::<Vec<_>>()
            })
            .map(|line| {
                let mut gears = vec![false; gears_count + 1];
                line.iter().for_each(|(id, _)| {
                    if let Some(g) = *id {
                        gears[g] = true
                    }
                });

                gears
                    .iter()
                    .enumerate()
                    .filter(|(_, v)| **v)
                    .map(|(i, _)| i)
                    .map(|gear_id| {
                        line.iter()
                            .map(|(id, ch)| {
                                if let Some(g_id) = *id {
                                    if g_id == gear_id {
                                        *ch
                                    } else {
                                        ' '
                                    }
                                } else {
                                    ' '
                                }
                            })
                            .collect::<String>()
                            .split(" ")
                            .map(|str| str.parse::<u32>())
                            .flatten()
                            .map(|v| (gear_id, v))
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .fold(vec![vec![]; gears_count], |acc, (id, v)| {
                let mut c_acc = acc.clone();
                c_acc[id - 1].push(v);
                c_acc
            })
            .iter()
            .map(|v| {
                if v.len() != 2 {
                    None
                } else {
                    Some(v[0] * v[1])
                }
            })
            .flatten()
            .collect()
    }

    fn validate_around(
        valid_table: &mut Vec<Vec<CellInfo>>,
        (x, y): (usize, usize),
        cell_type: CellValidType,
    ) {
        // invalidate cell if not None
        let mut validate_cell = |x: usize, y: usize, cell_type: CellValidType| {
            if let Some(line) = &mut valid_table.get_mut(y) {
                if let Some(cell) = &mut line.get_mut(x) {
                    **cell = CellInfo::Valid { cell_type };
                }
            }
        };

        // invalidate cell and cells around the cell
        for i in -1..2 {
            for j in -1..2 {
                validate_cell(
                    (x as i32 + i) as usize,
                    (y as i32 + j) as usize,
                    CellValidType::Default,
                );
            }
        }
        validate_cell(x, y, cell_type);
    }

    fn setup_number(
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

    fn board_info_with_gears(&self) -> Option<(Vec<Vec<CellInfo>>, usize)> {
        let (info, gears_count) = self.board_info()?;
        let mut c_info = info.clone();

        info.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, cell)| match cell {
                        CellInfo::Valid { cell_type } => match cell_type {
                            CellValidType::Gear { id } => Some((*id, (x, y))),
                            _ => None,
                        },
                        _ => None,
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .for_each(|(id, (x, y))| {
                for i in -1..2 {
                    for j in -1..2 {
                        self.setup_number(
                            &mut c_info,
                            ((x as i32 + i) as usize, (y as i32 + j) as usize),
                            CellValidType::Gear { id },
                        )
                    }
                }
            });

        Some((c_info, gears_count))
    }

    fn board_info(&self) -> Option<(Vec<Vec<CellInfo>>, usize)> {
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
                        gears_count += 1;
                        CellValidType::Gear { id: gears_count }
                    } else {
                        CellValidType::Default
                    };

                    Board::validate_around(&mut info, (x, y), cell_type);
                })
        });

        for y in 0..y_size {
            for x in 0..x_size {
                let valid_cell = info.get(y)?.get(x)?.clone();
                match valid_cell {
                    CellInfo::Valid { cell_type, .. } => {
                        self.setup_number(&mut info, (x, y), cell_type.clone())
                    }
                    _ => (),
                }
            }
        }

        Some((info, gears_count))
    }
}
