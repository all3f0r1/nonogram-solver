use crate::grid::{Grid, CellState, Constraints};
use crate::solver::{Deduction, line_solver_optimized::OptimizedLineSolver};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Solveur parallélisé utilisant Rayon
pub struct ParallelSolver {
    verbose: bool,
}

impl ParallelSolver {
    pub fn new() -> Self {
        Self { verbose: false }
    }

    pub fn with_verbose(verbose: bool) -> Self {
        Self { verbose }
    }

    /// Résout la grille en parallèle
    pub fn solve(&self, grid: &mut Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut all_deductions = Vec::new();
        let mut changed = true;
        let mut iteration = 0;
        const MAX_ITERATIONS: usize = 100;

        if self.verbose {
            println!("⚡ Démarrage du solveur parallèle");
        }

        while changed && iteration < MAX_ITERATIONS {
            changed = false;
            iteration += 1;

            if self.verbose {
                println!("   Itération {}", iteration);
            }

            let row_deductions = self.solve_rows_parallel(grid, constraints)?;
            if !row_deductions.is_empty() {
                if self.verbose {
                    println!("      Lignes: {} déductions", row_deductions.len());
                }
                self.apply_deductions(grid, &row_deductions)?;
                all_deductions.extend(row_deductions);
                changed = true;
            }

            let col_deductions = self.solve_columns_parallel(grid, constraints)?;
            if !col_deductions.is_empty() {
                if self.verbose {
                    println!("      Colonnes: {} déductions", col_deductions.len());
                }
                self.apply_deductions(grid, &col_deductions)?;
                all_deductions.extend(col_deductions);
                changed = true;
            }

            if !changed && self.verbose {
                println!("   ✓ Convergence atteinte");
            }
        }

        if self.verbose {
            println!("   Total: {} déductions en {} itérations", all_deductions.len(), iteration);
        }

        Ok(all_deductions)
    }

    /// Résout toutes les lignes en parallèle
    fn solve_rows_parallel(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let deductions = Arc::new(Mutex::new(Vec::new()));
        let height = grid.height();

        (0..height).into_par_iter().try_for_each(|row| {
            let line = grid.get_row(row)
                .ok_or_else(|| format!("Ligne {} non trouvée", row))?;
            let constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;

            let mut solver = OptimizedLineSolver::new();
            let valid_configs = solver.generate_valid_configurations(&line, constraint);

            if valid_configs.is_empty() {
                return Ok(());
            }

            for col in 0..line.len() {
                if line[col] != CellState::Empty {
                    continue;
                }

                let all_filled = valid_configs.iter().all(|config| config[col] == CellState::Filled);
                let all_crossed = valid_configs.iter().all(|config| config[col] == CellState::Crossed);

                if all_filled {
                    deductions.lock().unwrap().push(Deduction {
                        row,
                        col,
                        state: CellState::Filled,
                    });
                } else if all_crossed {
                    deductions.lock().unwrap().push(Deduction {
                        row,
                        col,
                        state: CellState::Crossed,
                    });
                }
            }

            Ok::<(), String>(())
        })?;

        let result = Arc::try_unwrap(deductions)
            .map_err(|_| "Erreur de synchronisation".to_string())?
            .into_inner()
            .map_err(|_| "Erreur de lock".to_string())?;

        Ok(result)
    }

    /// Résout toutes les colonnes en parallèle
    fn solve_columns_parallel(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let deductions = Arc::new(Mutex::new(Vec::new()));
        let width = grid.width();

        (0..width).into_par_iter().try_for_each(|col| {
            let column = grid.get_column(col)
                .ok_or_else(|| format!("Colonne {} non trouvée", col))?;
            let constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;

            let mut solver = OptimizedLineSolver::new();
            let valid_configs = solver.generate_valid_configurations(&column, constraint);

            if valid_configs.is_empty() {
                return Ok(());
            }

            for row in 0..column.len() {
                if column[row] != CellState::Empty {
                    continue;
                }

                let all_filled = valid_configs.iter().all(|config| config[row] == CellState::Filled);
                let all_crossed = valid_configs.iter().all(|config| config[row] == CellState::Crossed);

                if all_filled {
                    deductions.lock().unwrap().push(Deduction {
                        row,
                        col,
                        state: CellState::Filled,
                    });
                } else if all_crossed {
                    deductions.lock().unwrap().push(Deduction {
                        row,
                        col,
                        state: CellState::Crossed,
                    });
                }
            }

            Ok::<(), String>(())
        })?;

        let result = Arc::try_unwrap(deductions)
            .map_err(|_| "Erreur de synchronisation".to_string())?
            .into_inner()
            .map_err(|_| "Erreur de lock".to_string())?;

        Ok(result)
    }

    fn apply_deductions(&self, grid: &mut Grid, deductions: &[Deduction]) -> Result<(), String> {
        for deduction in deductions {
            grid.set(deduction.row, deduction.col, deduction.state)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parallel_solver() {
        let mut grid = Grid::new(5, 5);
        let mut constraints = Constraints::new(5, 5);
        
        constraints.set_row_constraint(0, vec![2]);
        constraints.set_row_constraint(1, vec![1, 1]);
        constraints.set_row_constraint(2, vec![5]);
        constraints.set_row_constraint(3, vec![1, 1]);
        constraints.set_row_constraint(4, vec![2]);
        
        constraints.set_column_constraint(0, vec![2]);
        constraints.set_column_constraint(1, vec![1, 1]);
        constraints.set_column_constraint(2, vec![5]);
        constraints.set_column_constraint(3, vec![1, 1]);
        constraints.set_column_constraint(4, vec![2]);
        
        let solver = ParallelSolver::new();
        let deductions = solver.solve(&mut grid, &constraints).unwrap();
        
        assert!(!deductions.is_empty());
    }
}
