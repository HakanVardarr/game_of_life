use super::cell::Cell;

pub struct Board {
    pub cells: Vec<Vec<Cell>>,
    pub new_cells: Vec<Vec<Cell>>,
    pub height: usize,
    pub width: usize,
}

impl Board {
    pub fn new(height: usize, width: usize) -> Self {
        let mut cells = Vec::new();
        let mut new_cells = Vec::new();
        for _ in 0..height {
            let mut cells_inner = Vec::new();
            let mut new_cells_inner = Vec::new();
            for _ in 0..width {
                cells_inner.push(Cell::dead());
                new_cells_inner.push(Cell::dead());
            }
            cells.push(cells_inner);
            new_cells.push(new_cells_inner);
        }

        Self {
            cells,
            new_cells,
            height,
            width,
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{}", self.cells[y][x])?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}
