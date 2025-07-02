#[derive(Debug, Clone)]
pub struct Field {
    cells: Vec<bool>,
    width: usize,
    height: usize,
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            cells: vec![false; width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn cells(&self) -> &Vec<bool> {
        &self.cells
    }

    pub fn get_index(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get_cell(&self, row: usize, col: usize) -> bool {
        let idx = self.get_index(row, col);
        self.cells[idx]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, alive: bool) {
        let idx = self.get_index(row, col);
        self.cells[idx] = alive;
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) {
        let idx = self.get_index(row, col);
        self.cells[idx] = !self.cells[idx];
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            *cell = false;
        }
    }

    pub fn randomize(&mut self) {
        use js_sys::Math;
        for cell in &mut self.cells {
            *cell = Math::random() > 0.5;
        }
    }

    pub fn count_live_neighbors(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = (row + delta_row) % self.height;
                let neighbor_col = (col + delta_col) % self.width;
                
                if self.get_cell(neighbor_row, neighbor_col) {
                    count += 1;
                }
            }
        }
        
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field_creation() {
        let field = Field::new(10, 10);
        assert_eq!(field.width(), 10);
        assert_eq!(field.height(), 10);
        assert_eq!(field.cells().len(), 100);
    }

    #[test]
    fn test_cell_operations() {
        let mut field = Field::new(3, 3);
        
        // Test initial state
        assert!(!field.get_cell(1, 1));
        
        // Test set_cell
        field.set_cell(1, 1, true);
        assert!(field.get_cell(1, 1));
        
        // Test toggle_cell
        field.toggle_cell(1, 1);
        assert!(!field.get_cell(1, 1));
    }
}
