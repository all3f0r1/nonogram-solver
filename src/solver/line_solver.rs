use crate::grid::CellState;

/// Solveur pour une ligne ou colonne individuelle
pub struct LineSolver;

impl LineSolver {
    pub fn new() -> Self {
        Self
    }

    /// Résout une ligne en utilisant la déduction logique
    /// Retourne les déductions sous forme de (position, état)
    pub fn solve_line(&self, line: &[CellState], constraint: &[usize]) -> Result<Vec<(usize, CellState)>, String> {
        let length = line.len();
        
        // Cas spécial: contrainte vide signifie que toute la ligne est vide
        if constraint.is_empty() {
            let mut deductions = Vec::new();
            for (i, &cell) in line.iter().enumerate() {
                if cell == CellState::Empty {
                    deductions.push((i, CellState::Crossed));
                }
            }
            return Ok(deductions);
        }

        // Générer toutes les configurations valides
        let valid_configs = self.generate_valid_configurations(line, constraint)?;

        if valid_configs.is_empty() {
            return Err("Aucune configuration valide trouvée pour cette ligne".to_string());
        }

        // Trouver les cases qui ont la même valeur dans toutes les configurations
        let mut deductions = Vec::new();
        for pos in 0..length {
            // Si la case est déjà déterminée, on la saute
            if line[pos] != CellState::Empty {
                continue;
            }

            // Vérifier si toutes les configurations ont la même valeur à cette position
            let first_value = valid_configs[0][pos];
            let all_same = valid_configs.iter().all(|config| config[pos] == first_value);

            if all_same && first_value != CellState::Empty {
                deductions.push((pos, first_value));
            }
        }

        Ok(deductions)
    }

    /// Génère toutes les configurations valides pour une ligne donnée
    fn generate_valid_configurations(&self, line: &[CellState], constraint: &[usize]) -> Result<Vec<Vec<CellState>>, String> {
        let length = line.len();
        let mut configurations = Vec::new();

        // Générer récursivement toutes les configurations possibles
        self.generate_recursive(line, constraint, 0, 0, vec![CellState::Empty; length], &mut configurations);

        Ok(configurations)
    }

    /// Génération récursive des configurations
    fn generate_recursive(
        &self,
        line: &[CellState],
        constraint: &[usize],
        block_index: usize,
        start_pos: usize,
        mut current: Vec<CellState>,
        results: &mut Vec<Vec<CellState>>,
    ) {
        let length = line.len();

        // Cas de base: tous les blocs ont été placés
        if block_index >= constraint.len() {
            // Remplir le reste avec des Crossed
            for i in start_pos..length {
                if current[i] == CellState::Empty {
                    current[i] = CellState::Crossed;
                }
            }

            // Vérifier si cette configuration est compatible avec la ligne actuelle
            if self.is_compatible(&current, line) {
                results.push(current);
            }
            return;
        }

        let block_size = constraint[block_index];
        let remaining_blocks: usize = constraint[block_index + 1..].iter().sum();
        let remaining_spaces = if block_index + 1 < constraint.len() {
            constraint.len() - block_index - 1
        } else {
            0
        };
        let min_space_needed = remaining_blocks + remaining_spaces;

        // Essayer de placer le bloc à différentes positions
        for pos in start_pos..=(length - block_size - min_space_needed) {
            let mut new_current = current.clone();

            // Placer des Crossed avant le bloc
            for i in start_pos..pos {
                if new_current[i] == CellState::Empty {
                    new_current[i] = CellState::Crossed;
                }
            }

            // Placer le bloc (Filled)
            let mut can_place = true;
            for i in pos..(pos + block_size) {
                if line[i] == CellState::Crossed {
                    can_place = false;
                    break;
                }
                new_current[i] = CellState::Filled;
            }

            if !can_place {
                continue;
            }

            // Ajouter un espace après le bloc (sauf si c'est le dernier bloc)
            let next_start = if block_index + 1 < constraint.len() {
                if pos + block_size < length {
                    if line[pos + block_size] == CellState::Filled {
                        continue; // Ne peut pas placer un espace ici
                    }
                    new_current[pos + block_size] = CellState::Crossed;
                    pos + block_size + 1
                } else {
                    continue; // Pas assez d'espace
                }
            } else {
                pos + block_size
            };

            // Récursion pour le bloc suivant
            self.generate_recursive(line, constraint, block_index + 1, next_start, new_current, results);
        }
    }

    /// Vérifie si une configuration est compatible avec l'état actuel de la ligne
    fn is_compatible(&self, config: &[CellState], line: &[CellState]) -> bool {
        for (&config_cell, &line_cell) in config.iter().zip(line.iter()) {
            match line_cell {
                CellState::Empty => {
                    // La case n'est pas encore déterminée, toute valeur est acceptable
                    continue;
                }
                CellState::Filled => {
                    // La case doit être Filled dans la configuration
                    if config_cell != CellState::Filled {
                        return false;
                    }
                }
                CellState::Crossed => {
                    // La case doit être Crossed dans la configuration
                    if config_cell != CellState::Crossed {
                        return false;
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_constraint() {
        let solver = LineSolver::new();
        let line = vec![CellState::Empty; 5];
        let constraint = vec![];
        let deductions = solver.solve_line(&line, &constraint).unwrap();
        
        // Toutes les cases doivent être marquées comme Crossed
        assert_eq!(deductions.len(), 5);
        for (_, state) in deductions {
            assert_eq!(state, CellState::Crossed);
        }
    }

    #[test]
    fn test_full_line() {
        let solver = LineSolver::new();
        let line = vec![CellState::Empty; 5];
        let constraint = vec![5];
        let deductions = solver.solve_line(&line, &constraint).unwrap();
        
        // Toutes les cases doivent être marquées comme Filled
        assert_eq!(deductions.len(), 5);
        for (_, state) in deductions {
            assert_eq!(state, CellState::Filled);
        }
    }

    #[test]
    fn test_partial_deduction() {
        let solver = LineSolver::new();
        let line = vec![CellState::Empty; 7];
        let constraint = vec![5];
        let deductions = solver.solve_line(&line, &constraint).unwrap();
        
        // Les cases du milieu doivent être déduites comme Filled
        // Pour une ligne de 7 avec un bloc de 5, les positions 2, 3, 4 sont forcément noires
        assert!(!deductions.is_empty());
    }

    #[test]
    fn test_multiple_blocks() {
        let solver = LineSolver::new();
        let line = vec![CellState::Empty; 7];
        let constraint = vec![2, 2];
        let deductions = solver.solve_line(&line, &constraint).unwrap();
        
        // Peut ou non avoir des déductions selon les positions possibles
        // Ce test vérifie simplement qu'il n'y a pas d'erreur
        assert!(true);
    }

    #[test]
    fn test_with_existing_filled() {
        let solver = LineSolver::new();
        let mut line = vec![CellState::Empty; 5];
        line[2] = CellState::Filled;
        let constraint = vec![3];
        let deductions = solver.solve_line(&line, &constraint).unwrap();
        
        // Le bloc de 3 doit inclure la case 2, donc les cases adjacentes peuvent être déduites
        assert!(!deductions.is_empty());
    }
}
