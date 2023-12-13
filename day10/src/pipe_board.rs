#[derive(Debug)]
pub struct PipesBoard(Vec<Vec<bool>>);

impl From<(usize, usize)> for PipesBoard {
    fn from((height, width): (usize, usize)) -> Self {
        PipesBoard {
            0: vec![vec![false; width]; height],
        }
    }
}

impl PipesBoard {
    pub fn push(&mut self, (width, height): (i64, i64)) {
        self.0[height as usize][width as usize] = true;
    }

    fn map_row(row: &Vec<bool>) -> Vec<bool> {
        let last_pipe = row.iter().enumerate().fold(0, |acc, (i, pipe)| match pipe {
            true => i,
            false => acc,
        });

        row.iter()
            .enumerate()
            .fold(
                (vec![], false, false),
                |(mut row, next, prev_pipe), (i, &v)| {
                    if i > last_pipe {
                        row.push(false);
                        (row, false, false)
                    } else {
                        match v {
                            true => {
                                row.push(false);
                                // match prev_pipe {
                                //     true => (row, next, true),
                                //     false => (row, !next, true),
                                // }
                                (row, !next, true)
                            }
                            false => {
                                row.push(next);
                                (row, next, false)
                            }
                        }
                    }
                },
            )
            .0
    }

    fn map_rows(rows: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
        // map closed blocks from right block to left block ===>
        let left_to_right = rows.iter().map(Self::map_row);
        let right_to_left = rows.iter().map(Self::map_row);

        left_to_right
            .zip(right_to_left)
            .map(|(lr_row, row_rl)| {
                lr_row
                    .iter()
                    .zip(row_rl.iter())
                    .map(|(lr, rl)| *lr && *rl)
                    .rev()
                    .collect()
            })
            .collect()
    }

    fn print(rows: &Vec<Vec<bool>>) {
        rows.iter().for_each(|row| {
            row.iter()
                .for_each(|v| if *v { print!("|") } else { print!(".") });
            print!("\n");
        })
    }

    pub fn count_inside_blocks(&self) -> i64 {
        let rows_mapped = Self::map_rows(&self.0);

        Self::print(&self.0);

        let height = self.0.len();
        let width = self.0[0].len();
        let cols = (0..width)
            .flat_map(|y| {
                Some(
                    (0..height)
                        .flat_map(|x| {
                            let row = self.0.get(x)?;
                            row.get(y).copied()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<Vec<_>>();

        Self::print(&cols);
        let cols_mapped_rotated = Self::map_rows(&cols);
        let cols_mapped = (0..height)
            .flat_map(|y| {
                let width = self.0.get(y)?.len();

                Some(
                    (0..width)
                        .flat_map(|x| {
                            let row = cols_mapped_rotated.get(x)?;
                            row.get(y).copied()
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .rev()
            .collect::<Vec<_>>();
        Self::print(&cols_mapped);
        Self::print(&rows_mapped);

        rows_mapped
            .iter()
            .flatten()
            .zip(cols_mapped.iter().flatten())
            .fold(
                0,
                |acc, (row, col)| if *row && *col { acc + 1 } else { acc },
            )
    }
}
