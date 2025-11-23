use crate::grid::{Grid, Constraints};
use crate::solver::{Deduction, NonogramSolver};
use super::cross_analysis::CrossAnalyzer;
use super::advanced_heuristics::AdvancedHeuristics;

/// Configuration pour le solveur avancé
#[derive(Debug, Clone)]
pub struct AdvancedSolverConfig {
    /// Utiliser l'analyse de contraintes croisées
    pub use_cross_analysis: bool,
    /// Utiliser les heuristiques avancées
    pub use_advanced_heuristics: bool,
    /// Nombre maximal d'itérations
    pub max_iterations: usize,
    /// Mode verbeux
    pub verbose: bool,
}

impl Default for AdvancedSolverConfig {
    fn default() -> Self {
        Self {
            use_cross_analysis: true,
            use_advanced_heuristics: true,
            max_iterations: 100,
            verbose: false,
        }
    }
}

/// Solveur avancé qui combine toutes les techniques
pub struct AdvancedSolver {
    config: AdvancedSolverConfig,
    base_solver: NonogramSolver,
    cross_analyzer: CrossAnalyzer,
    heuristics: AdvancedHeuristics,
}

impl AdvancedSolver {
    pub fn new() -> Self {
        Self::with_config(AdvancedSolverConfig::default())
    }

    pub fn with_config(config: AdvancedSolverConfig) -> Self {
        Self {
            config,
            base_solver: NonogramSolver::new(),
            cross_analyzer: CrossAnalyzer::new(),
            heuristics: AdvancedHeuristics::new(),
        }
    }

    pub fn solve(&mut self, grid: &mut Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut all_deductions = Vec::new();
        let mut iteration = 0;
        let mut changed = true;

        if self.config.verbose {
            println!("🚀 Démarrage du solveur avancé");
            println!("   - Analyse croisée: {}", if self.config.use_cross_analysis { "✓" } else { "✗" });
            println!("   - Heuristiques avancées: {}", if self.config.use_advanced_heuristics { "✓" } else { "✗" });
        }

        while changed && iteration < self.config.max_iterations {
            changed = false;
            iteration += 1;

            if self.config.verbose {
                println!("\n📍 Itération {}", iteration);
            }

            // Phase 1: Line solving
            if self.config.verbose {
                println!("   Phase 1: Line solving...");
            }
            let line_deductions = self.base_solver.solve(grid, constraints)?;
            if !line_deductions.is_empty() {
                if self.config.verbose {
                    println!("      → {} déductions", line_deductions.len());
                }
                self.apply_deductions(grid, &line_deductions)?;
                all_deductions.extend(line_deductions);
                changed = true;
            }

            // Phase 2: Analyse croisée
            if self.config.use_cross_analysis {
                if self.config.verbose {
                    println!("   Phase 2: Analyse croisée...");
                }
                let cross_deductions = self.cross_analyzer.analyze(grid, constraints)?;
                let new_deductions: Vec<_> = cross_deductions.into_iter()
                    .filter(|d| grid.get(d.row, d.col) == Some(crate::grid::CellState::Empty))
                    .collect();
                
                if !new_deductions.is_empty() {
                    if self.config.verbose {
                        println!("      → {} déductions", new_deductions.len());
                    }
                    self.apply_deductions(grid, &new_deductions)?;
                    all_deductions.extend(new_deductions);
                    changed = true;
                }
            }

            // Phase 3: Heuristiques avancées
            if self.config.use_advanced_heuristics {
                if self.config.verbose {
                    println!("   Phase 3: Heuristiques avancées...");
                }
                let heuristic_deductions = self.heuristics.apply(grid, constraints)?;
                let new_deductions: Vec<_> = heuristic_deductions.into_iter()
                    .filter(|d| grid.get(d.row, d.col) == Some(crate::grid::CellState::Empty))
                    .collect();
                
                if !new_deductions.is_empty() {
                    if self.config.verbose {
                        println!("      → {} déductions", new_deductions.len());
                    }
                    self.apply_deductions(grid, &new_deductions)?;
                    all_deductions.extend(new_deductions);
                    changed = true;
                }
            }

            if !changed && self.config.verbose {
                println!("   ✓ Convergence atteinte");
            }
        }

        if iteration >= self.config.max_iterations && self.config.verbose {
            println!("⚠️  Nombre maximal d'itérations atteint");
        }

        if self.config.verbose {
            println!("\n✅ Résolution terminée");
            println!("   Total: {} déductions en {} itérations", all_deductions.len(), iteration);
            let empty_cells = grid.count_empty_cells();
            let total_cells = grid.width() * grid.height();
            let progress = ((total_cells - empty_cells) as f64 / total_cells as f64 * 100.0) as usize;
            println!("   Progression: {}% ({}/{} cases résolues)", progress, total_cells - empty_cells, total_cells);
        }

        Ok(all_deductions)
    }

    fn apply_deductions(&self, grid: &mut Grid, deductions: &[Deduction]) -> Result<(), String> {
        for deduction in deductions {
            grid.set(deduction.row, deduction.col, deduction.state)?;
        }
        Ok(())
    }

    pub fn clear_cache(&mut self) {
        self.base_solver.clear_cache();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grid::CellState;

    #[test]
    fn test_advanced_solver_simple() {
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
        
        let mut solver = AdvancedSolver::new();
        let deductions = solver.solve(&mut grid, &constraints).unwrap();
        
        assert!(!deductions.is_empty());
        
        assert_eq!(grid.get(2, 0), Some(CellState::Filled));
        assert_eq!(grid.get(2, 1), Some(CellState::Filled));
        assert_eq!(grid.get(2, 2), Some(CellState::Filled));
        assert_eq!(grid.get(2, 3), Some(CellState::Filled));
        assert_eq!(grid.get(2, 4), Some(CellState::Filled));
    }
}
