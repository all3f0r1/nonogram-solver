use super::contradiction_detector::ContradictionDetector;
use crate::grid::{CellState, Constraints, Grid};
use crate::solver::{AdvancedSolver, AdvancedSolverConfig, Deduction};
use std::collections::{HashMap, HashSet};

/// Configuration pour le backtracking optimisé
#[derive(Debug, Clone)]
pub struct OptimizedBacktrackingConfig {
    pub max_depth: usize,
    pub max_states: usize,
    pub use_constraint_propagation: bool,
    pub use_naked_singles: bool,
    pub use_hidden_singles: bool,
    pub verbose: bool,
}

impl Default for OptimizedBacktrackingConfig {
    fn default() -> Self {
        Self {
            max_depth: 50,      // Augmenté de 10 à 50
            max_states: 100000, // Augmenté de 10,000 à 100,000
            use_constraint_propagation: true,
            use_naked_singles: true,
            use_hidden_singles: true,
            verbose: false,
        }
    }
}

/// Solveur avec backtracking optimisé
pub struct OptimizedBacktrackingSolver {
    config: OptimizedBacktrackingConfig,
    advanced_solver: AdvancedSolver,
    contradiction_detector: ContradictionDetector,
    states_explored: usize,
    visited_states: HashSet<String>,
    deduction_cache: HashMap<String, Vec<Deduction>>,
}

impl OptimizedBacktrackingSolver {
    pub fn new() -> Self {
        Self::with_config(OptimizedBacktrackingConfig::default())
    }

    pub fn with_config(config: OptimizedBacktrackingConfig) -> Self {
        let advanced_config = AdvancedSolverConfig {
            use_cross_analysis: true,
            use_advanced_heuristics: true,
            max_iterations: 100,
            verbose: false,
        };

        Self {
            config,
            advanced_solver: AdvancedSolver::with_config(advanced_config),
            contradiction_detector: ContradictionDetector::new(),
            states_explored: 0,
            visited_states: HashSet::new(),
            deduction_cache: HashMap::new(),
        }
    }

    /// Résout la grille avec backtracking optimisé
    pub fn solve(
        &mut self,
        grid: &mut Grid,
        constraints: &Constraints,
    ) -> Result<Vec<Deduction>, String> {
        self.states_explored = 0;
        self.visited_states.clear();
        self.deduction_cache.clear();

        if self.config.verbose {
            println!("🔄 Démarrage du backtracking optimisé");
            println!("   - Profondeur max: {}", self.config.max_depth);
            println!("   - États max: {}", self.config.max_states);
            println!(
                "   - Propagation de contraintes: {}",
                if self.config.use_constraint_propagation {
                    "✓"
                } else {
                    "✗"
                }
            );
        }

        // Phase 1: Appliquer le solveur avancé
        let initial_deductions = self.advanced_solver.solve(grid, constraints)?;

        if self.config.verbose {
            println!(
                "   ✓ Solveur avancé: {} déductions",
                initial_deductions.len()
            );
        }

        if grid.count_empty_cells() == 0 {
            if self.config.verbose {
                println!("   ✓ Grille complète sans backtracking");
            }
            return Ok(initial_deductions);
        }

        // Phase 2: Appliquer les techniques avancées
        if self.config.use_naked_singles {
            self.apply_naked_singles(grid, constraints)?;
        }

        if self.config.use_hidden_singles {
            self.apply_hidden_singles(grid, constraints)?;
        }

        if grid.count_empty_cells() == 0 {
            if self.config.verbose {
                println!("   ✓ Grille complète après techniques avancées");
            }
            return self.collect_all_deductions(grid);
        }

        if self.config.verbose {
            println!(
                "   🔍 Lancement du backtracking ({} cases vides)",
                grid.count_empty_cells()
            );
        }

        // Phase 3: Backtracking avec propagation de contraintes
        match self.backtrack(grid, constraints, 0) {
            Ok(_) => {
                if self.config.verbose {
                    println!("   ✅ Backtracking réussi");
                    println!("   - États explorés: {}", self.states_explored);
                }

                self.collect_all_deductions(grid)
            }
            Err(e) => {
                if self.config.verbose {
                    println!("   ⚠️  Backtracking incomplet: {}", e);
                    println!("   - États explorés: {}", self.states_explored);
                }

                self.collect_all_deductions(grid)
            }
        }
    }

    /// Collecte toutes les déductions de la grille
    fn collect_all_deductions(&self, grid: &Grid) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        for row in 0..grid.height() {
            for col in 0..grid.width() {
                if let Some(state) = grid.get(row, col) {
                    if state != CellState::Empty {
                        deductions.push(Deduction { row, col, state });
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Applique la technique des "naked singles"
    /// Une case est un naked single si elle ne peut avoir qu'une seule valeur
    fn apply_naked_singles(
        &mut self,
        grid: &mut Grid,
        constraints: &Constraints,
    ) -> Result<(), String> {
        let mut changed = true;

        while changed {
            changed = false;

            for row in 0..grid.height() {
                for col in 0..grid.width() {
                    if grid.get(row, col) == Some(CellState::Empty) {
                        // Tester si Filled est la seule option
                        let mut test_grid = grid.clone();
                        test_grid.set(row, col, CellState::Filled)?;

                        let filled_valid = !self
                            .contradiction_detector
                            .has_contradiction(&test_grid, constraints);

                        // Tester si Crossed est la seule option
                        let mut test_grid = grid.clone();
                        test_grid.set(row, col, CellState::Crossed)?;

                        let crossed_valid = !self
                            .contradiction_detector
                            .has_contradiction(&test_grid, constraints);

                        // Si une seule option est valide, l'appliquer
                        if filled_valid && !crossed_valid {
                            grid.set(row, col, CellState::Filled)?;
                            changed = true;
                        } else if !filled_valid && crossed_valid {
                            grid.set(row, col, CellState::Crossed)?;
                            changed = true;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Applique la technique des "hidden singles"
    /// Une valeur est un hidden single si elle ne peut aller que dans une seule case d'une ligne/colonne
    fn apply_hidden_singles(
        &mut self,
        grid: &mut Grid,
        constraints: &Constraints,
    ) -> Result<(), String> {
        // Pour chaque ligne
        for row in 0..grid.height() {
            let row_constraint = constraints
                .get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;

            // Trouver les positions possibles pour chaque bloc
            for (_block_idx, &block_size) in row_constraint.iter().enumerate() {
                let mut possible_positions = Vec::new();

                for col in 0..grid.width() {
                    // Vérifier si le bloc peut commencer à cette position
                    if self.can_place_block(grid, row, col, block_size, true) {
                        possible_positions.push(col);
                    }
                }

                // Si une seule position est possible, placer le bloc
                if possible_positions.len() == 1 {
                    let col = possible_positions[0];
                    for i in 0..block_size {
                        if grid.get(row, col + i) == Some(CellState::Empty) {
                            grid.set(row, col + i, CellState::Filled)?;
                        }
                    }
                }
            }
        }

        // Pour chaque colonne
        for col in 0..grid.width() {
            let col_constraint = constraints
                .get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;

            for (_block_idx, &block_size) in col_constraint.iter().enumerate() {
                let mut possible_positions = Vec::new();

                for row in 0..grid.height() {
                    if self.can_place_block(grid, row, col, block_size, false) {
                        possible_positions.push(row);
                    }
                }

                if possible_positions.len() == 1 {
                    let row = possible_positions[0];
                    for i in 0..block_size {
                        if grid.get(row + i, col) == Some(CellState::Empty) {
                            grid.set(row + i, col, CellState::Filled)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Vérifie si un bloc peut être placé à une position donnée
    fn can_place_block(
        &self,
        grid: &Grid,
        start_row: usize,
        start_col: usize,
        size: usize,
        horizontal: bool,
    ) -> bool {
        if horizontal {
            // Vérifier si le bloc dépasse la grille
            if start_col + size > grid.width() {
                return false;
            }

            // Vérifier si toutes les cases sont vides ou remplies
            for i in 0..size {
                if let Some(CellState::Crossed) = grid.get(start_row, start_col + i) {
                    return false;
                }
            }

            true
        } else {
            // Vérifier si le bloc dépasse la grille
            if start_row + size > grid.height() {
                return false;
            }

            // Vérifier si toutes les cases sont vides ou remplies
            for i in 0..size {
                if let Some(CellState::Crossed) = grid.get(start_row + i, start_col) {
                    return false;
                }
            }

            true
        }
    }

    /// Backtracking récursif avec optimisations
    fn backtrack(
        &mut self,
        grid: &mut Grid,
        constraints: &Constraints,
        depth: usize,
    ) -> Result<(), String> {
        // Vérifier les limites
        if depth >= self.config.max_depth {
            return Err("Profondeur maximale atteinte".to_string());
        }

        if self.states_explored >= self.config.max_states {
            return Err("Nombre maximal d'états atteints".to_string());
        }

        self.states_explored += 1;

        // Vérifier si la grille est complète
        if grid.count_empty_cells() == 0 {
            return Ok(());
        }

        // Vérifier si cet état a déjà été visité
        let state_key = self.grid_to_string(grid);
        if self.visited_states.contains(&state_key) {
            return Err("État déjà visité".to_string());
        }
        self.visited_states.insert(state_key.clone());

        // Propagation de contraintes après chaque choix
        if self.config.use_constraint_propagation {
            let _deductions = self.advanced_solver.solve(grid, constraints)?;

            if grid.count_empty_cells() == 0 {
                return Ok(());
            }
        }

        // Choisir la meilleure case avec heuristique MRV améliorée
        let (best_row, best_col) = self.choose_best_cell_mrv_plus(grid, constraints)?;

        // Essayer Filled en premier (heuristique: les grilles ont généralement plus de cases noires)
        for &state in &[CellState::Filled, CellState::Crossed] {
            let mut test_grid = grid.clone();
            test_grid.set(best_row, best_col, state)?;

            // Vérifier rapidement les contradictions
            if !self
                .contradiction_detector
                .has_contradiction(&test_grid, constraints)
            {
                // Appliquer le choix
                grid.set(best_row, best_col, state)?;

                // Continuer le backtracking
                match self.backtrack(grid, constraints, depth + 1) {
                    Ok(()) => return Ok(()),
                    Err(_) => {
                        // Annuler le choix
                        grid.set(best_row, best_col, CellState::Empty)?;
                    }
                }
            }
        }

        // Aucun choix n'a fonctionné
        Err("Aucune solution trouvée".to_string())
    }

    /// Heuristique MRV améliorée avec analyse de contraintes
    fn choose_best_cell_mrv_plus(
        &self,
        grid: &Grid,
        constraints: &Constraints,
    ) -> Result<(usize, usize), String> {
        let mut best_score = 0;
        let mut best_cell = None;

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

        best_cell.ok_or_else(|| "Aucune case vide trouvée".to_string())
    }

    /// Calcule le score d'une case pour l'heuristique MRV+
    fn calculate_cell_score(
        &self,
        grid: &Grid,
        constraints: &Constraints,
        row: usize,
        col: usize,
    ) -> usize {
        let mut score = 0;

        // Compter les cases remplies dans la ligne
        let mut filled_in_row = 0;
        for c in 0..grid.width() {
            if grid.get(row, c) == Some(CellState::Filled) {
                filled_in_row += 1;
            }
        }
        score += filled_in_row * 10;

        // Compter les cases remplies dans la colonne
        let mut filled_in_col = 0;
        for r in 0..grid.height() {
            if grid.get(r, col) == Some(CellState::Filled) {
                filled_in_col += 1;
            }
        }
        score += filled_in_col * 10;

        // Ajouter la complexité des contraintes
        if let Some(row_constraint) = constraints.get_row_constraint(row) {
            score += row_constraint.len() * 5;
            score += row_constraint.iter().sum::<usize>();
        }

        if let Some(col_constraint) = constraints.get_column_constraint(col) {
            score += col_constraint.len() * 5;
            score += col_constraint.iter().sum::<usize>();
        }

        // Bonus pour les cases proches des bords
        let distance_to_edge = row.min(grid.height() - row - 1) + col.min(grid.width() - col - 1);
        score += (10 - distance_to_edge.min(10)) * 2;

        score
    }

    /// Convertit la grille en string pour le cache
    fn grid_to_string(&self, grid: &Grid) -> String {
        let mut result = String::new();

        for row in 0..grid.height() {
            for col in 0..grid.width() {
                match grid.get(row, col) {
                    Some(CellState::Empty) => result.push('.'),
                    Some(CellState::Filled) => result.push('#'),
                    Some(CellState::Crossed) => result.push('X'),
                    None => result.push('?'),
                }
            }
        }

        result
    }
}

impl Default for OptimizedBacktrackingSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_backtracking_creation() {
        let solver = OptimizedBacktrackingSolver::new();
        assert_eq!(solver.states_explored, 0);
    }

    #[test]
    fn test_optimized_backtracking_simple() {
        let mut grid = Grid::new(5, 5);
        let rows = vec![vec![5], vec![1], vec![1], vec![1], vec![5]];
        let columns = vec![vec![2], vec![1, 1], vec![1, 1], vec![1, 1], vec![2]];
        let constraints = Constraints::new(5, 5, rows, columns).unwrap();

        let mut solver = OptimizedBacktrackingSolver::new();
        let result = solver.solve(&mut grid, &constraints);

        assert!(result.is_ok());
    }
}
