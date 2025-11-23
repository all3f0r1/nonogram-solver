pub mod line_solver;
pub mod line_solver_optimized;
pub mod cross_analysis;
pub mod advanced_heuristics;
pub mod advanced_solver;
pub mod contradiction_detector;
pub mod backtracking;
pub mod backtracking_optimized;
pub mod parallel_solver;
pub mod ultimate_solver;

pub use line_solver_optimized::OptimizedLineSolver;
pub use cross_analysis::CrossAnalyzer;
pub use advanced_heuristics::AdvancedHeuristics;
pub use advanced_solver::{AdvancedSolver, AdvancedSolverConfig};
pub use contradiction_detector::ContradictionDetector;
pub use backtracking::{BacktrackingSolver, BacktrackingConfig};
pub use backtracking_optimized::{OptimizedBacktrackingSolver, OptimizedBacktrackingConfig};
pub use parallel_solver::ParallelSolver;
pub use ultimate_solver::{UltimateSolver, UltimateSolverConfig};

use crate::grid::{Grid, CellState, Constraints};

/// Représente une déduction faite par le solveur
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Deduction {
    pub row: usize,
    pub col: usize,
    pub state: CellState,
}

/// Solveur de nonogramme utilisant la déduction logique
pub struct NonogramSolver {
    line_solver: OptimizedLineSolver,
}

impl NonogramSolver {
    /// Crée un nouveau solveur
    pub fn new() -> Self {
        Self {
            line_solver: OptimizedLineSolver::new(),
        }
    }

    /// Résout la grille autant que possible en utilisant la déduction logique
    /// Retourne la liste des déductions effectuées
    pub fn solve(&mut self, grid: &mut Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut all_deductions = Vec::new();
        let mut changed = true;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 1000;

        while changed && iteration < MAX_ITERATIONS {
            changed = false;
            iteration += 1;

            // Résoudre toutes les lignes
            for row in 0..grid.height() {
                let row_constraint = constraints.get_row_constraint(row)
                    .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
                
                let current_line = grid.get_row(row)
                    .ok_or_else(|| format!("Ligne {} non trouvée", row))?;

                let deductions = self.line_solver.solve_line(&current_line, row_constraint)?;

                if !deductions.is_empty() {
                    changed = true;
                    for (col, state) in deductions {
                        grid.set(row, col, state)?;
                        all_deductions.push(Deduction { row, col, state });
                    }
                }
            }

            // Résoudre toutes les colonnes
            for col in 0..grid.width() {
                let col_constraint = constraints.get_column_constraint(col)
                    .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
                
                let current_column = grid.get_column(col)
                    .ok_or_else(|| format!("Colonne {} non trouvée", col))?;

                let deductions = self.line_solver.solve_line(&current_column, col_constraint)?;

                if !deductions.is_empty() {
                    changed = true;
                    for (row, state) in deductions {
                        grid.set(row, col, state)?;
                        all_deductions.push(Deduction { row, col, state });
                    }
                }
            }
        }

        if iteration >= MAX_ITERATIONS {
            return Err("Nombre maximal d'itérations atteint".to_string());
        }

        Ok(all_deductions)
    }

    /// Trouve uniquement les nouvelles déductions possibles sans modifier la grille
    /// Utile pour marquer les cases qui peuvent être déduites
    pub fn find_next_deductions(&mut self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut grid_copy = grid.clone();
        self.solve(&mut grid_copy, constraints)
    }

    /// Vide le cache du line solver
    pub fn clear_cache(&mut self) {
        self.line_solver.clear_cache();
    }
}

impl Default for NonogramSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solver_creation() {
        let solver = NonogramSolver::new();
        assert!(true); // Test basique de création
    }

    #[test]
    fn test_simple_solve() {
        let mut grid = Grid::new(5, 5);
        let rows = vec![
            vec![5],  // Toute la ligne est noire
            vec![1],
            vec![1],
            vec![1],
            vec![5],  // Toute la ligne est noire
        ];
        let columns = vec![
            vec![2],
            vec![1, 1],
            vec![1, 1],
            vec![1, 1],
            vec![2],
        ];
        let constraints = Constraints::new(5, 5, rows, columns).unwrap();
        
        let solver = NonogramSolver::new();
        let deductions = solver.solve(&mut grid, &constraints).unwrap();
        
        // Au moins les lignes complètes devraient être déduites
        assert!(!deductions.is_empty());
    }
}
