use crate::grid::{Grid, CellState, Constraints};
use crate::solver::{Deduction, AdvancedSolver, AdvancedSolverConfig};
use super::contradiction_detector::ContradictionDetector;
use std::collections::HashSet;

/// Configuration pour le backtracking
#[derive(Debug, Clone)]
pub struct BacktrackingConfig {
    pub max_depth: usize,
    pub max_states: usize,
    pub verbose: bool,
}

impl Default for BacktrackingConfig {
    fn default() -> Self {
        Self {
            max_depth: 10,
            max_states: 10000,
            verbose: false,
        }
    }
}

/// Solveur avec backtracking intelligent
pub struct BacktrackingSolver {
    config: BacktrackingConfig,
    advanced_solver: AdvancedSolver,
    contradiction_detector: ContradictionDetector,
    states_explored: usize,
    visited_states: HashSet<String>,
}

impl BacktrackingSolver {
    pub fn new() -> Self {
        Self::with_config(BacktrackingConfig::default())
    }

    pub fn with_config(config: BacktrackingConfig) -> Self {
        let advanced_config = AdvancedSolverConfig {
            use_cross_analysis: true,
            use_advanced_heuristics: true,
            max_iterations: 50,
            verbose: false,
        };

        Self {
            config,
            advanced_solver: AdvancedSolver::with_config(advanced_config),
            contradiction_detector: ContradictionDetector::new(),
            states_explored: 0,
            visited_states: HashSet::new(),
        }
    }

    /// Résout la grille avec backtracking intelligent
    pub fn solve(&mut self, grid: &mut Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        self.states_explored = 0;
        self.visited_states.clear();

        if self.config.verbose {
            println!("🔄 Démarrage du backtracking intelligent");
            println!("   - Profondeur max: {}", self.config.max_depth);
            println!("   - États max: {}", self.config.max_states);
        }

        let initial_deductions = self.advanced_solver.solve(grid, constraints)?;

        if self.config.verbose {
            println!("   ✓ Solveur avancé: {} déductions", initial_deductions.len());
        }

        if grid.count_empty_cells() == 0 {
            if self.config.verbose {
                println!("   ✓ Grille complète sans backtracking");
            }
            return Ok(initial_deductions);
        }

        if self.config.verbose {
            println!("   🔍 Lancement du backtracking ({} cases vides)", grid.count_empty_cells());
        }

        let mut all_deductions = initial_deductions.clone();
        
        match self.backtrack(grid, constraints, 0) {
            Ok(additional_deductions) => {
                all_deductions.extend(additional_deductions);
                
                if self.config.verbose {
                    println!("   ✅ Backtracking réussi");
                    println!("   - États explorés: {}", self.states_explored);
                    println!("   - Total déductions: {}", all_deductions.len());
                }
                
                Ok(all_deductions)
            }
            Err(e) => {
                if self.config.verbose {
                    println!("   ⚠️  Backtracking incomplet: {}", e);
                    println!("   - États explorés: {}", self.states_explored);
                }
                Ok(all_deductions)
            }
        }
    }

    /// Fonction récursive de backtracking
    fn backtrack(&mut self, grid: &mut Grid, constraints: &Constraints, depth: usize) -> Result<Vec<Deduction>, String> {
        if depth >= self.config.max_depth {
            return Err("Profondeur maximale atteinte".to_string());
        }

        if self.states_explored >= self.config.max_states {
            return Err("Nombre maximal d'états atteint".to_string());
        }

        self.states_explored += 1;

        let state_key = self.grid_to_string(grid);
        if self.visited_states.contains(&state_key) {
            return Err("État déjà visité".to_string());
        }
        self.visited_states.insert(state_key);

        let deductions = self.advanced_solver.solve(grid, constraints)?;

        if grid.count_empty_cells() == 0 {
            return Ok(deductions);
        }

        let (row, col) = match self.choose_best_cell(grid, constraints) {
            Some(pos) => pos,
            None => return Ok(deductions),
        };

        // Essayer Filled
        let mut test_grid = grid.clone();
        if test_grid.set(row, col, CellState::Filled).is_ok() {
            if self.contradiction_detector.is_valid(&test_grid, constraints) {
                match self.backtrack(&mut test_grid, constraints, depth + 1) {
                    Ok(mut branch_deductions) => {
                        grid.set(row, col, CellState::Filled)?;
                        branch_deductions.push(Deduction {
                            row,
                            col,
                            state: CellState::Filled,
                        });
                        branch_deductions.extend(deductions);
                        return Ok(branch_deductions);
                    }
                    Err(_) => {}
                }
            }
        }

        // Essayer Crossed
        let mut test_grid = grid.clone();
        if test_grid.set(row, col, CellState::Crossed).is_ok() {
            if self.contradiction_detector.is_valid(&test_grid, constraints) {
                match self.backtrack(&mut test_grid, constraints, depth + 1) {
                    Ok(mut branch_deductions) => {
                        grid.set(row, col, CellState::Crossed)?;
                        branch_deductions.push(Deduction {
                            row,
                            col,
                            state: CellState::Crossed,
                        });
                        branch_deductions.extend(deductions);
                        return Ok(branch_deductions);
                    }
                    Err(_) => {}
                }
            }
        }

        Err("Aucune solution trouvée".to_string())
    }

    /// Choisit la meilleure case (MRV heuristic)
    fn choose_best_cell(&self, grid: &Grid, constraints: &Constraints) -> Option<(usize, usize)> {
        let mut best_cell = None;
        let mut best_score = 0;

        for row in 0..grid.height() {
            for col in 0..grid.width() {
                if grid.get(row, col) == Some(CellState::Empty) {
                    let score = self.calculate_cell_score(grid, constraints, row, col);
                    if score > best_score {
                        best_score = score;
                        best_cell = Some((row, col));
                    }
                }
            }
        }

        best_cell
    }

    /// Calcule le score d'une case
    fn calculate_cell_score(&self, grid: &Grid, constraints: &Constraints, row: usize, col: usize) -> usize {
        let mut score = 0;

        if let Some(line) = grid.get_row(row) {
            score += line.iter().filter(|&&c| c == CellState::Filled).count();
        }

        if let Some(column) = grid.get_column(col) {
            score += column.iter().filter(|&&c| c == CellState::Filled).count();
        }

        if let Some(row_constraint) = constraints.get_row_constraint(row) {
            score += row_constraint.len() * 2;
        }

        if let Some(col_constraint) = constraints.get_column_constraint(col) {
            score += col_constraint.len() * 2;
        }

        score
    }

    /// Convertit une grille en string
    fn grid_to_string(&self, grid: &Grid) -> String {
        let mut result = String::new();
        for row in 0..grid.height() {
            if let Some(line) = grid.get_row(row) {
                for cell in line {
                    result.push(match cell {
                        CellState::Empty => '.',
                        CellState::Filled => '#',
                        CellState::Crossed => 'X',
                    });
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backtracking_simple() {
        let mut grid = Grid::new(3, 3);
        let mut constraints = Constraints::new(3, 3);
        
        constraints.set_row_constraint(0, vec![1]);
        constraints.set_row_constraint(1, vec![3]);
        constraints.set_row_constraint(2, vec![1]);
        
        constraints.set_column_constraint(0, vec![1]);
        constraints.set_column_constraint(1, vec![3]);
        constraints.set_column_constraint(2, vec![1]);
        
        let config = BacktrackingConfig {
            max_depth: 5,
            max_states: 100,
            verbose: false,
        };
        
        let mut solver = BacktrackingSolver::with_config(config);
        let result = solver.solve(&mut grid, &constraints);
        
        assert!(result.is_ok());
    }
}
