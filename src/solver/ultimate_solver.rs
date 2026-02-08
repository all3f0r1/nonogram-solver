use super::backtracking_optimized::{OptimizedBacktrackingConfig, OptimizedBacktrackingSolver};
use super::parallel_solver::ParallelSolver;
use crate::grid::{Constraints, Grid};
use crate::solver::{AdvancedSolver, AdvancedSolverConfig, Deduction};

/// Configuration pour le solveur ultime
#[derive(Debug, Clone)]
pub struct UltimateSolverConfig {
    pub use_parallel: bool,
    pub use_backtracking: bool,
    pub backtracking_depth: usize,
    pub verbose: bool,
}

impl Default for UltimateSolverConfig {
    fn default() -> Self {
        Self {
            use_parallel: true,
            use_backtracking: true,
            backtracking_depth: 10,
            verbose: false,
        }
    }
}

/// Solveur ultime combinant toutes les techniques
pub struct UltimateSolver {
    config: UltimateSolverConfig,
}

impl UltimateSolver {
    pub fn new() -> Self {
        Self::with_config(UltimateSolverConfig::default())
    }

    pub fn with_config(config: UltimateSolverConfig) -> Self {
        Self { config }
    }

    /// Résout la grille avec toutes les techniques disponibles
    pub fn solve(
        &mut self,
        grid: &mut Grid,
        constraints: &Constraints,
    ) -> Result<Vec<Deduction>, String> {
        if self.config.verbose {
            println!("🌟 Démarrage du solveur ultime");
            println!("   Configuration:");
            println!(
                "   - Parallélisation: {}",
                if self.config.use_parallel {
                    "✓"
                } else {
                    "✗"
                }
            );
            println!(
                "   - Backtracking: {}",
                if self.config.use_backtracking {
                    "✓"
                } else {
                    "✗"
                }
            );
        }

        let mut all_deductions = Vec::new();

        // Phase 1: Solveur avancé
        if self.config.verbose {
            println!("\n📍 Phase 1: Solveur avancé");
        }

        let advanced_config = AdvancedSolverConfig {
            use_cross_analysis: true,
            use_advanced_heuristics: true,
            max_iterations: 100,
            verbose: self.config.verbose,
        };

        let mut advanced_solver = AdvancedSolver::with_config(advanced_config);
        let advanced_deductions = advanced_solver.solve(grid, constraints)?;

        if self.config.verbose {
            println!("   ✓ {} déductions", advanced_deductions.len());
        }

        all_deductions.extend(advanced_deductions);

        if grid.count_empty_cells() == 0 {
            if self.config.verbose {
                println!("\n✅ Grille complète après le solveur avancé");
            }
            return Ok(all_deductions);
        }

        // Phase 2: Parallélisation
        if self.config.use_parallel {
            if self.config.verbose {
                println!("\n📍 Phase 2: Solveur parallèle");
            }

            let parallel_solver = ParallelSolver::with_verbose(self.config.verbose);
            let parallel_deductions = parallel_solver.solve(grid, constraints)?;

            if !parallel_deductions.is_empty() {
                if self.config.verbose {
                    println!(
                        "   ✓ {} déductions supplémentaires",
                        parallel_deductions.len()
                    );
                }
                all_deductions.extend(parallel_deductions);
            }

            if grid.count_empty_cells() == 0 {
                if self.config.verbose {
                    println!("\n✅ Grille complète après parallélisation");
                }
                return Ok(all_deductions);
            }
        }

        // Phase 3: Backtracking
        if self.config.use_backtracking && grid.count_empty_cells() > 0 {
            if self.config.verbose {
                println!("\n📍 Phase 3: Backtracking intelligent");
                println!("   Cases restantes: {}", grid.count_empty_cells());
            }

            let backtracking_config = OptimizedBacktrackingConfig {
                max_depth: self.config.backtracking_depth,
                max_states: 100000,
                use_constraint_propagation: true,
                use_naked_singles: true,
                use_hidden_singles: true,
                verbose: self.config.verbose,
            };

            let mut backtracking_solver =
                OptimizedBacktrackingSolver::with_config(backtracking_config);
            let backtracking_deductions = backtracking_solver.solve(grid, constraints)?;

            if !backtracking_deductions.is_empty() {
                if self.config.verbose {
                    println!(
                        "   ✓ {} déductions supplémentaires",
                        backtracking_deductions.len()
                    );
                }
                all_deductions.extend(backtracking_deductions);
            }
        }

        // Résumé final
        if self.config.verbose {
            let empty_cells = grid.count_empty_cells();
            let total_cells = grid.width() * grid.height();
            let solved_cells = total_cells - empty_cells;
            let progress = (solved_cells as f64 / total_cells as f64 * 100.0) as usize;

            println!("\n✅ Résolution terminée");
            println!("   Total déductions: {}", all_deductions.len());
            println!(
                "   Progression: {}% ({}/{} cases)",
                progress, solved_cells, total_cells
            );

            if empty_cells > 0 {
                println!("   ⚠️  {} cases non résolues", empty_cells);
            } else {
                println!("   🎉 Grille complètement résolue!");
            }
        }

        Ok(all_deductions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::CellState;

    #[test]
    fn test_ultimate_solver_simple() {
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

        let mut solver = UltimateSolver::new();
        let deductions = solver.solve(&mut grid, &constraints).unwrap();

        assert!(!deductions.is_empty());

        assert_eq!(grid.get(2, 0), Some(CellState::Filled));
        assert_eq!(grid.get(2, 2), Some(CellState::Filled));
        assert_eq!(grid.get(2, 4), Some(CellState::Filled));
    }
}
