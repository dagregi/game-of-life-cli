pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<u64>,
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let size = (width * height) as usize;
        Universe {
            width,
            height,
            cells: vec![0; (size + 63) / 64],
        }
    }

    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for &(row, col) in cells {
            let idx = self.get_index(row, col);
            self.cells[idx / 64] |= 1 << (idx % 64);
        }
    }

    pub fn tick(&mut self) {
        let mut next = vec![0; self.cells.len()];
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let live_neighbours = self.live_neighbour_count(row, col);
                if (self.cells[idx / 64] >> (idx % 64)) & 1 == 1 {
                    next[idx / 64] |= match live_neighbours {
                        2 | 3 => 1 << (idx % 64),
                        _ => 0,
                    };
                } else if live_neighbours == 3 {
                    next[idx / 64] |= 1 << (idx % 64);
                }
            }
        }
        self.cells = next;
    }

    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += ((self.cells[idx / 64] >> (idx % 64)) & 1) as u8;
            }
        }
        count
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn render(&self) -> String {
        let mut output = String::new();
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if (self.cells[idx / 64] >> (idx % 64)) & 1 == 1 {
                    '◼'
                } else {
                    '◻'
                };
                let mut push = " ".to_string();
                push.push(symbol);
                output.push_str(&push);
            }
            output.push('\n');
        }
        output
    }

    pub fn get_row_as_string(&self, row: u32) -> Result<String, String> {
        if row < self.height {
            Ok(self.render().lines().nth(row as usize).unwrap().to_string())
        } else {
            Err("Row out of bounds".to_string())
        }
    }
}
