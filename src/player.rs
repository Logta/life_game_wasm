use crate::{config::Color, field, rules::*};

use field::Field;

#[derive(Debug)]
pub struct Player<R: rule_trait::Rule> {
    pub field: Field,
    pub generation: u64,
    _rule: R,
}

impl<R: rule_trait::Rule> Player<R> {
    pub fn init(_rule: R) -> Self {
        Self {
            field: Field::init(),
            generation: 0,
            _rule,
        }
    }

    pub fn invert(mut self, x: usize, y: usize) -> Self {
        self.field = self.field.invert_cell_state(x, y);
        self
    }

    pub fn step(mut self) -> Self {
        self.generation += 1;
        for x in 0..self.field.max_column {
            for y in 0..self.field.max_row {
                let target_cells = R::target_cells(1, 1, 10, 10);
                let current_color = self.field.board[y as usize][x as usize];
                let colors = target_cells
                    .iter()
                    .map(|cs| self.field.board[cs.1 as usize][cs.0 as usize])
                    .collect();
                let next_color = if current_color == Color::White {
                    if R::burth(colors) {
                        Color::Black
                    } else {
                        Color::White
                    }
                } else {
                    if R::stay(colors) {
                        Color::Black
                    } else {
                        Color::White
                    }
                };
                if next_color != current_color {
                    self = self.invert(x as usize, y as usize)
                }
            }
        }
        self
    }

    pub fn play(mut self, generation: u64) -> Self {
        for gen in 0..generation {
            if gen % 500 == 0 {
                println!("世代: {}", gen);
                println!("{:?}", self.field.board);
            }
            self = self.step();
        }
        self
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_player() {
        let player = Player::init(standard_rule::StandardRule {});
        // Test
        assert_eq!(player.generation, 0);
    }

    #[test]
    fn test_step() {
        let mut pl = Player::init(standard_rule::StandardRule {});
        pl = pl.invert(1, 1);
        pl = pl.invert(2, 1);
        pl = pl.invert(1, 2);
        pl = pl.step();
        // Test
        assert_eq!(pl.generation, 1);
    }

    #[test]
    fn test_play() {
        let mut pl = Player::init(standard_rule::StandardRule {});
        pl = pl.invert(10, 10);
        pl = pl.invert(12, 10);
        pl = pl.invert(9, 11);
        pl = pl.invert(9, 12);
        pl = pl.play(2000);
        // Test
        assert_eq!(pl.generation, 2000);
        println!("{:?}", pl.field.board);
    }
}
