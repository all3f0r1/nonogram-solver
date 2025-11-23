use crate::grid::CellState;
use std::collections::HashMap;

/// Solveur optimisé pour une ligne ou colonne individuelle avec cache
pub struct OptimizedLineSolver {
    /// Cache des configurations valides pour éviter les recalculs
    cache: HashMap<(Vec<CellState>, Vec<usize>), Vec<Vec<CellState>>>,
}

impl OptimizedLineSolver {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    /// Résout une ligne en utilisant la déduction logique avec optimisations
    pub fn solve_line(&mut self, line: &[CellState], constraint: &[usize]) -> Result<Vec<(usize, CellState)>, String> {
        let length = line.len();
        
        // Cas spécial: contrainte vide
        if constraint.is_empty() {
            let mut deductions = Vec::new();
            for (i, &cell) in line.iter().enumerate() {
                if cell == CellState::Empty {
                    deductions.push((i, CellState::Crossed));
                }
            }
            return Ok(deductions);
        }

        // Vérifier le cache
        let cache_key = (line.to_vec(), constraint.to_vec());
        let valid_configs = if let Some(cached) = self.cache.get(&cache_key) {
            cached.clone()
        } else {
            let configs = self.generate_valid_configurations_optimized(line, constraint)?;
            self.cache.insert(cache_key, configs.clone());
            configs
        };

        if valid_configs.is_empty() {
            return Err("Aucune configuration valide trouvée pour cette ligne".to_string());
        }

        // Optimisation: utiliser un compteur pour chaque position
        let mut filled_count = vec![0usize; length];
        let mut crossed_count = vec![0usize; length];
        let total_configs = valid_configs.len();

        for config in &valid_configs {
            for (pos, &state) in config.iter().enumerate() {
                match state {
                    CellState::Filled => filled_count[pos] += 1,
                    CellState::Crossed => crossed_count[pos] += 1,
                    CellState::Empty => {}
                }
            }
        }

        // Déduire les cases qui ont la même valeur dans toutes les configurations
        let mut deductions = Vec::new();
        for pos in 0..length {
            if line[pos] != CellState::Empty {
                continue;
            }

            if filled_count[pos] == total_configs {
                deductions.push((pos, CellState::Filled));
            } else if crossed_count[pos] == total_configs {
                deductions.push((pos, CellState::Crossed));
            }
        }

        Ok(deductions)
    }

    /// Génère les configurations valides avec optimisations
    fn generate_valid_configurations_optimized(&self, line: &[CellState], constraint: &[usize]) -> Result<Vec<Vec<CellState>>, String> {
        let length = line.len();
        let mut configurations = Vec::new();

        // Élagage précoce: vérifier si la contrainte est satisfaisable
        let min_length = self.min_line_length(constraint);
        if min_length > length {
            return Ok(configurations);
        }

        self.generate_recursive_optimized(
            line,
            constraint,
            0,
            0,
            vec![CellState::Empty; length],
            &mut configurations,
        );

        Ok(configurations)
    }

    /// Génération récursive optimisée avec élagage précoce
    fn generate_recursive_optimized(
        &self,
        line: &[CellState],
        constraint: &[usize],
        block_index: usize,
        start_pos: usize,
        mut current: Vec<CellState>,
        results: &mut Vec<Vec<CellState>>,
    ) {
        let length = line.len();

        // Cas de base
        if block_index >= constraint.len() {
            for i in start_pos..length {
                if current[i] == CellState::Empty {
                    current[i] = CellState::Crossed;
                }
            }

            if self.is_compatible(&current, line) {
                results.push(current);
            }
            return;
        }

        let block_size = constraint[block_index];
        let remaining_blocks: usize = constraint[block_index + 1..].iter().sum();
        let remaining_spaces = constraint.len().saturating_sub(block_index + 1);
        let min_space_needed = remaining_blocks + remaining_spaces;

        // Élagage: si pas assez d'espace, arrêter
        if start_pos + block_size + min_space_needed > length {
            return;
        }

        // Essayer de placer le bloc à différentes positions
        let max_pos = length.saturating_sub(block_size + min_space_needed);
        
        for pos in start_pos..=max_pos {
            // Élagage précoce: vérifier la compatibilité avant de continuer
            let mut can_place = true;
            
            // Vérifier que les cases avant peuvent être Crossed
            for i in start_pos..pos {
                if line[i] == CellState::Filled {
                    can_place = false;
                    break;
                }
            }
            
            if !can_place {
                continue;
            }

            // Vérifier que le bloc peut être placé ici
            for i in pos..(pos + block_size) {
                if line[i] == CellState::Crossed {
                    can_place = false;
                    break;
                }
            }

            if !can_place {
                continue;
            }

            // Vérifier l'espace après le bloc
            if block_index + 1 < constraint.len() && pos + block_size < length {
                if line[pos + block_size] == CellState::Filled {
                    continue;
                }
            }

            // Placer le bloc
            let mut new_current = current.clone();

            for i in start_pos..pos {
                if new_current[i] == CellState::Empty {
                    new_current[i] = CellState::Crossed;
                }
            }

            for i in pos..(pos + block_size) {
                new_current[i] = CellState::Filled;
            }

            let next_start = if block_index + 1 < constraint.len() {
                if pos + block_size < length {
                    new_current[pos + block_size] = CellState::Crossed;
                    pos + block_size + 1
                } else {
                    continue;
                }
            } else {
                pos + block_size
            };

            self.generate_recursive_optimized(
                line,
                constraint,
                block_index + 1,
                next_start,
                new_current,
                results,
            );
        }
    }

    /// Vérifie la compatibilité
    fn is_compatible(&self, config: &[CellState], line: &[CellState]) -> bool {
        config.iter().zip(line.iter()).all(|(&config_cell, &line_cell)| {
            match line_cell {
                CellState::Empty => true,
                CellState::Filled => config_cell == CellState::Filled,
                CellState::Crossed => config_cell == CellState::Crossed,
            }
        })
    }

    /// Calcule la longueur minimale nécessaire
    fn min_line_length(&self, constraint: &[usize]) -> usize {
        if constraint.is_empty() {
            return 0;
        }
        constraint.iter().sum::<usize>() + constraint.len() - 1
    }

    /// Vide le cache (utile pour libérer la mémoire)
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for OptimizedLineSolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimized_solver() {
        let mut solver = OptimizedLineSolver::new();
        let line = vec![CellState::Empty; 5];
        let constraint = vec![5];
        let deductions = solver.solve_line(&line, &constraint).unwrap();
        
        assert_eq!(deductions.len(), 5);
        for (_, state) in deductions {
            assert_eq!(state, CellState::Filled);
        }
    }

    #[test]
    fn test_cache() {
        let mut solver = OptimizedLineSolver::new();
        let line = vec![CellState::Empty; 7];
        let constraint = vec![3, 2];
        
        // Première résolution
        let _ = solver.solve_line(&line, &constraint).unwrap();
        assert_eq!(solver.cache.len(), 1);
        
        // Deuxième résolution (devrait utiliser le cache)
        let _ = solver.solve_line(&line, &constraint).unwrap();
        assert_eq!(solver.cache.len(), 1);
        
        // Vider le cache
        solver.clear_cache();
        assert_eq!(solver.cache.len(), 0);
    }
}
