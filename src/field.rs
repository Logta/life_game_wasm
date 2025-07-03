use crate::error::{GameError, Result};

#[derive(Debug, Clone, PartialEq)]
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

    pub fn cells(&self) -> &[bool] {
        &self.cells
    }

    pub fn cells_mut(&mut self) -> &mut [bool] {
        &mut self.cells
    }

    pub fn get_index(&self, row: usize, col: usize) -> Result<usize> {
        if row >= self.height || col >= self.width {
            return Err(GameError::new(&format!(
                "Cell position ({}, {}) is out of bounds for field of size {}x{}",
                row, col, self.height, self.width
            )));
        }
        Ok(row * self.width + col)
    }

    fn get_index_unchecked(&self, row: usize, col: usize) -> usize {
        row * self.width + col
    }

    pub fn get_cell(&self, row: usize, col: usize) -> Result<bool> {
        let idx = self.get_index(row, col)?;
        Ok(self.cells[idx])
    }

    pub fn get_cell_unchecked(&self, row: usize, col: usize) -> bool {
        let idx = self.get_index_unchecked(row, col);
        self.cells[idx]
    }

    pub fn set_cell(&mut self, row: usize, col: usize, alive: bool) -> Result<()> {
        let idx = self.get_index(row, col)?;
        self.cells[idx] = alive;
        Ok(())
    }

    pub fn set_cell_unchecked(&mut self, row: usize, col: usize, alive: bool) {
        let idx = self.get_index_unchecked(row, col);
        self.cells[idx] = alive;
    }

    pub fn toggle_cell(&mut self, row: usize, col: usize) -> Result<()> {
        let idx = self.get_index(row, col)?;
        self.cells[idx] = !self.cells[idx];
        Ok(())
    }

    pub fn clear(&mut self) {
        self.cells.fill(false);
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
                
                if self.get_cell_unchecked(neighbor_row, neighbor_col) {
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
        
        // All cells should be dead initially
        for cell in field.cells() {
            assert!(!cell);
        }
    }

    #[test]
    fn test_cell_operations() {
        let mut field = Field::new(3, 3);
        
        // Test initial state
        assert!(!field.get_cell(1, 1).unwrap());
        
        // Test set_cell
        field.set_cell(1, 1, true).unwrap();
        assert!(field.get_cell(1, 1).unwrap());
        
        // Test toggle_cell
        field.toggle_cell(1, 1).unwrap();
        assert!(!field.get_cell(1, 1).unwrap());
        
        // Test out of bounds
        assert!(field.get_cell(10, 10).is_err());
        assert!(field.set_cell(10, 10, true).is_err());
        assert!(field.toggle_cell(10, 10).is_err());
    }
    
    #[test]
    fn test_clear() {
        let mut field = Field::new(3, 3);
        
        // Set some cells alive
        field.set_cell(0, 0, true).unwrap();
        field.set_cell(1, 1, true).unwrap();
        field.set_cell(2, 2, true).unwrap();
        
        // Clear the field
        field.clear();
        
        // All cells should be dead
        for row in 0..3 {
            for col in 0..3 {
                assert!(!field.get_cell(row, col).unwrap());
            }
        }
    }
    
    #[test]
    fn test_count_live_neighbors() {
        let mut field = Field::new(3, 3);
        
        // Create a pattern:
        // X O X
        // O X O
        // X O X
        field.set_cell(0, 1, true).unwrap();
        field.set_cell(1, 0, true).unwrap();
        field.set_cell(1, 2, true).unwrap();
        field.set_cell(2, 1, true).unwrap();
        
        // Center cell should have 4 live neighbors
        assert_eq!(field.count_live_neighbors(1, 1), 4);
        
        // Corner cells should see wrapping behavior
        let corner_neighbors = field.count_live_neighbors(0, 0);
        assert!(corner_neighbors > 0); // Due to wrapping
    }
}
