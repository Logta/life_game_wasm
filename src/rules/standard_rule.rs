use crate::field::Field;

pub struct StandardRule;

impl StandardRule {
    pub fn new() -> Self {
        StandardRule
    }

    pub fn apply(&self, field: &Field) -> Field {
        let mut next_field = field.clone();
        
        for row in 0..field.height() {
            for col in 0..field.width() {
                let live_neighbors = field.count_live_neighbors(row, col);
                let is_alive = field.get_cell(row, col);
                
                let next_state = match (is_alive, live_neighbors) {
                    // Live cell with 2-3 neighbors survives
                    (true, 2) | (true, 3) => true,
                    // Dead cell with exactly 3 neighbors becomes alive
                    (false, 3) => true,
                    // All other cases: cell dies or stays dead
                    _ => false,
                };
                
                next_field.set_cell(row, col, next_state);
            }
        }
        
        next_field
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::field::Field;

    #[test]
    fn test_standard_rule() {
        let mut field = Field::new(5, 5);
        let rule = StandardRule::new();
        
        // Create a blinker pattern
        field.set_cell(2, 1, true);
        field.set_cell(2, 2, true);
        field.set_cell(2, 3, true);
        
        // Apply rule once
        let next_field = rule.apply(&field);
        
        // Check the pattern has rotated
        assert!(next_field.get_cell(1, 2));
        assert!(next_field.get_cell(2, 2));
        assert!(next_field.get_cell(3, 2));
        assert!(!next_field.get_cell(2, 1));
        assert!(!next_field.get_cell(2, 3));
    }
}
