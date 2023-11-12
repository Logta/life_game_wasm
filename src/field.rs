use crate::config::load_yaml;
use crate::config::Color;
use crate::config::CONFIG_PATH;

#[derive(Debug, Clone)]
pub struct Field {
    pub board: Vec<Vec<Color>>, // [y][x]
    pub max_row: u32,
    pub max_column: u32,
}

impl Field {
    pub fn init() -> Self {
        // let confs = load_yaml(CONFIG_PATH);
        // let conf = &confs[0];
        let y = 100; // conf["stageCols"].as_i64().unwrap();
        let x = 100; //conf["stageRows"].as_i64().unwrap();
        Self {
            board: vec![vec![Color::White; x as usize]; y as usize],
            max_row: y as u32,
            max_column: x as u32,
        }
    }

    pub fn invert_cell_state(mut self, x: usize, y: usize) -> Self {
        let current_cell_state = self.board[y][x];
        self.board[y][x] = match current_cell_state {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::White,
        };
        self
    }

    pub fn play_cell(mut self, x: usize, y: usize) -> Self {
        let current_cell_state = self.board[y][x];
        self.board[y][x] = match current_cell_state {
            Color::White => Color::Black,
            Color::Black => Color::White,
            _ => Color::White,
        };
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_field() {
        let confs = load_yaml(CONFIG_PATH);
        let conf = &confs[0];
        let y = conf["stageCols"].as_i64().unwrap();
        let x = conf["stageRows"].as_i64().unwrap();

        let field = Field::init();
        // Test
        assert_eq!(field.board.len(), y as usize);
        assert_eq!(field.board[0].len(), x as usize);
    }
}
