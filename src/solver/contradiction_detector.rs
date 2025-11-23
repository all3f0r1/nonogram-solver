use crate::grid::{Grid, CellState, Constraints};
use crate::solver::NonogramSolver;

/// Détecteur de contradictions pour nonogrammes
pub struct ContradictionDetector {
    solver: NonogramSolver,
}

impl ContradictionDetector {
    pub fn new() -> Self {
        Self {
            solver: NonogramSolver::new(),
        }
    }

    /// Vérifie si la grille contient des contradictions
    pub fn is_valid(&mut self, grid: &Grid, constraints: &Constraints) -> bool {
        if !self.check_basic_contradictions(grid, constraints) {
            return false;
        }

        if !self.check_impossible_blocks(grid, constraints) {
            return false;
        }

        if !self.check_deduction_contradictions(grid, constraints) {
            return false;
        }

        true
    }

    /// Test hypothétique: vérifie si placer un état dans une case crée une contradiction
    pub fn test_hypothesis(&mut self, grid: &Grid, constraints: &Constraints, row: usize, col: usize, state: CellState) -> bool {
        let mut test_grid = grid.clone();
        
        if test_grid.set(row, col, state).is_err() {
            return false;
        }

        self.is_valid(&test_grid, constraints)
    }

    /// Vérifie les contradictions de base
    fn check_basic_contradictions(&self, grid: &Grid, constraints: &Constraints) -> bool {
        for row in 0..grid.height() {
            let line = match grid.get_row(row) {
                Some(l) => l,
                None => return false,
            };
            let constraint = match constraints.get_row_constraint(row) {
                Some(c) => c,
                None => return false,
            };

            if !self.check_line_basic(&line, constraint) {
                return false;
            }
        }

        for col in 0..grid.width() {
            let column = match grid.get_column(col) {
                Some(c) => c,
                None => return false,
            };
            let constraint = match constraints.get_column_constraint(col) {
                Some(c) => c,
                None => return false,
            };

            if !self.check_line_basic(&column, constraint) {
                return false;
            }
        }

        true
    }

    /// Vérifie les contradictions de base pour une ligne
    fn check_line_basic(&self, line: &[CellState], constraint: &[usize]) -> bool {
        let filled_blocks = self.count_filled_blocks(line);
        
        if filled_blocks.len() > constraint.len() {
            return false;
        }

        for (i, (_, size)) in filled_blocks.iter().enumerate() {
            if i < constraint.len() && *size > constraint[i] {
                return false;
            }
        }

        let filled_count = line.iter().filter(|&&c| c == CellState::Filled).count();
        let required_count: usize = constraint.iter().sum();
        
        if filled_count > required_count {
            return false;
        }

        let available_space = line.iter().filter(|&&c| c != CellState::Crossed).count();
        let min_required_space = if constraint.is_empty() {
            0
        } else {
            constraint.iter().sum::<usize>() + constraint.len() - 1
        };

        if available_space < min_required_space {
            return false;
        }

        true
    }

    /// Vérifie les contradictions liées aux blocs impossibles
    fn check_impossible_blocks(&self, grid: &Grid, constraints: &Constraints) -> bool {
        for row in 0..grid.height() {
            let line = match grid.get_row(row) {
                Some(l) => l,
                None => return false,
            };
            let constraint = match constraints.get_row_constraint(row) {
                Some(c) => c,
                None => return false,
            };

            if !self.check_line_impossible_blocks(&line, constraint) {
                return false;
            }
        }

        for col in 0..grid.width() {
            let column = match grid.get_column(col) {
                Some(c) => c,
                None => return false,
            };
            let constraint = match constraints.get_column_constraint(col) {
                Some(c) => c,
                None => return false,
            };

            if !self.check_line_impossible_blocks(&column, constraint) {
                return false;
            }
        }

        true
    }

    /// Vérifie les blocs impossibles pour une ligne
    fn check_line_impossible_blocks(&self, line: &[CellState], constraint: &[usize]) -> bool {
        if constraint.is_empty() {
            return !line.iter().any(|&c| c == CellState::Filled);
        }

        let segments = self.find_available_segments(line);
        
        for &block_size in constraint.iter() {
            let can_fit = segments.iter().any(|(_, size)| *size >= block_size);
            if !can_fit {
                return false;
            }
        }

        true
    }

    /// Vérifie les contradictions par déduction
    fn check_deduction_contradictions(&mut self, grid: &Grid, constraints: &Constraints) -> bool {
        let mut test_grid = grid.clone();
        
        match self.solver.solve(&mut test_grid, constraints) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    /// Compte les blocs de cases remplies
    fn count_filled_blocks(&self, line: &[CellState]) -> Vec<(usize, usize)> {
        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut block_start = 0;
        let mut block_size = 0;

        for (i, &cell) in line.iter().enumerate() {
            match cell {
                CellState::Filled => {
                    if !in_block {
                        in_block = true;
                        block_start = i;
                        block_size = 1;
                    } else {
                        block_size += 1;
                    }
                }
                _ => {
                    if in_block {
                        blocks.push((block_start, block_size));
                        in_block = false;
                    }
                }
            }
        }

        if in_block {
            blocks.push((block_start, block_size));
        }

        blocks
    }

    /// Trouve les segments disponibles (non barrés)
    fn find_available_segments(&self, line: &[CellState]) -> Vec<(usize, usize)> {
        let mut segments = Vec::new();
        let mut in_segment = false;
        let mut segment_start = 0;
        let mut segment_size = 0;

        for (i, &cell) in line.iter().enumerate() {
            match cell {
                CellState::Crossed => {
                    if in_segment {
                        segments.push((segment_start, segment_size));
                        in_segment = false;
                    }
                }
                _ => {
                    if !in_segment {
                        in_segment = true;
                        segment_start = i;
                        segment_size = 1;
                    } else {
                        segment_size += 1;
                    }
                }
            }
        }

        if in_segment {
            segments.push((segment_start, segment_size));
        }

        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_grid() {
        let mut grid = Grid::new(5, 1);
        let mut constraints = Constraints::new(5, 1);
        constraints.set_row_constraint(0, vec![2]);
        
        grid.set(0, 1, CellState::Filled).unwrap();
        grid.set(0, 2, CellState::Filled).unwrap();
        
        let mut detector = ContradictionDetector::new();
        assert!(detector.is_valid(&grid, &constraints));
    }

    #[test]
    fn test_invalid_grid_too_many_blocks() {
        let mut grid = Grid::new(5, 1);
        let mut constraints = Constraints::new(5, 1);
        constraints.set_row_constraint(0, vec![1]);
        
        grid.set(0, 0, CellState::Filled).unwrap();
        grid.set(0, 2, CellState::Filled).unwrap();
        grid.set(0, 1, CellState::Crossed).unwrap();
        
        let mut detector = ContradictionDetector::new();
        assert!(!detector.is_valid(&grid, &constraints));
    }

    #[test]
    fn test_hypothesis_valid() {
        let grid = Grid::new(5, 1);
        let mut constraints = Constraints::new(5, 1);
        constraints.set_row_constraint(0, vec![2]);
        
        let mut detector = ContradictionDetector::new();
        assert!(detector.test_hypothesis(&grid, &constraints, 0, 1, CellState::Filled));
    }
}
