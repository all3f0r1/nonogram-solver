use crate::grid::{Grid, CellState, Constraints};
use crate::solver::Deduction;

/// Analyseur de contraintes croisées
/// 
/// Utilise les informations des lignes ET colonnes simultanément
/// pour trouver des déductions que le line solving simple ne peut pas trouver.
pub struct CrossAnalyzer;

impl CrossAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Analyse la grille avec des contraintes croisées
    pub fn analyze(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        // Overlap analysis pour toutes les lignes
        for row in 0..grid.height() {
            let row_constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
            
            let overlap_deductions = self.overlap_analysis_row(grid, row, row_constraint)?;
            deductions.extend(overlap_deductions);
        }

        // Overlap analysis pour toutes les colonnes
        for col in 0..grid.width() {
            let col_constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
            
            let overlap_deductions = self.overlap_analysis_column(grid, col, col_constraint)?;
            deductions.extend(overlap_deductions);
        }

        // Edge forcing pour toutes les lignes
        for row in 0..grid.height() {
            let row_constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
            
            let edge_deductions = self.edge_forcing_row(grid, row, row_constraint)?;
            deductions.extend(edge_deductions);
        }

        // Edge forcing pour toutes les colonnes
        for col in 0..grid.width() {
            let col_constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
            
            let edge_deductions = self.edge_forcing_column(grid, col, col_constraint)?;
            deductions.extend(edge_deductions);
        }

        // Dédupliquer les déductions
        deductions.sort_by_key(|d| (d.row, d.col));
        deductions.dedup_by_key(|d| (d.row, d.col));

        Ok(deductions)
    }

    /// Overlap analysis pour une ligne
    /// 
    /// Trouve les cases qui doivent être remplies car toutes les configurations
    /// possibles les incluent.
    fn overlap_analysis_row(&self, grid: &Grid, row: usize, constraint: &[usize]) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();
        let line = grid.get_row(row)
            .ok_or_else(|| format!("Ligne {} non trouvée", row))?;

        if constraint.is_empty() {
            return Ok(deductions);
        }

        let length = line.len();
        
        for (block_idx, &block_size) in constraint.iter().enumerate() {
            // Calculer la position minimale du bloc
            let min_pos = self.calculate_min_position(constraint, block_idx);
            
            // Calculer la position maximale du bloc
            let max_pos = self.calculate_max_position(constraint, block_idx, length);
            
            // S'il y a chevauchement, ces cases doivent être remplies
            if max_pos < min_pos + block_size {
                let overlap_start = max_pos;
                let overlap_end = min_pos + block_size;
                
                for col in overlap_start..overlap_end {
                    if col < length && line[col] == CellState::Empty {
                        deductions.push(Deduction {
                            row,
                            col,
                            state: CellState::Filled,
                        });
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Overlap analysis pour une colonne
    fn overlap_analysis_column(&self, grid: &Grid, col: usize, constraint: &[usize]) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();
        let column = grid.get_column(col)
            .ok_or_else(|| format!("Colonne {} non trouvée", col))?;

        if constraint.is_empty() {
            return Ok(deductions);
        }

        let length = column.len();
        
        for (block_idx, &block_size) in constraint.iter().enumerate() {
            let min_pos = self.calculate_min_position(constraint, block_idx);
            let max_pos = self.calculate_max_position(constraint, block_idx, length);
            
            if max_pos < min_pos + block_size {
                let overlap_start = max_pos;
                let overlap_end = min_pos + block_size;
                
                for row in overlap_start..overlap_end {
                    if row < length && column[row] == CellState::Empty {
                        deductions.push(Deduction {
                            row,
                            col,
                            state: CellState::Filled,
                        });
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Edge forcing pour une ligne
    /// 
    /// Force les cases aux bords basé sur les contraintes.
    fn edge_forcing_row(&self, grid: &Grid, row: usize, constraint: &[usize]) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();
        let line = grid.get_row(row)
            .ok_or_else(|| format!("Ligne {} non trouvée", row))?;

        if constraint.is_empty() {
            return Ok(deductions);
        }

        // Forcer depuis le début
        let first_block = constraint[0];
        for (col, &cell) in line.iter().enumerate() {
            if cell == CellState::Filled {
                // Une case remplie proche du début force le premier bloc
                if col < first_block {
                    // Remplir les cases nécessaires pour compléter le bloc
                    for fill_col in 0..first_block {
                        if line[fill_col] == CellState::Empty {
                            deductions.push(Deduction {
                                row,
                                col: fill_col,
                                state: CellState::Filled,
                            });
                        }
                    }
                    // Barrer la case après le bloc
                    if first_block < line.len() && line[first_block] == CellState::Empty {
                        deductions.push(Deduction {
                            row,
                            col: first_block,
                            state: CellState::Crossed,
                        });
                    }
                    break;
                }
            }
        }

        // Forcer depuis la fin
        let last_block = *constraint.last().unwrap();
        let length = line.len();
        for col in (0..length).rev() {
            if line[col] == CellState::Filled {
                // Une case remplie proche de la fin force le dernier bloc
                if col >= length - last_block {
                    // Remplir les cases nécessaires pour compléter le bloc
                    for fill_col in (length - last_block)..length {
                        if line[fill_col] == CellState::Empty {
                            deductions.push(Deduction {
                                row,
                                col: fill_col,
                                state: CellState::Filled,
                            });
                        }
                    }
                    // Barrer la case avant le bloc
                    if length > last_block && line[length - last_block - 1] == CellState::Empty {
                        deductions.push(Deduction {
                            row,
                            col: length - last_block - 1,
                            state: CellState::Crossed,
                        });
                    }
                    break;
                }
            }
        }

        Ok(deductions)
    }

    /// Edge forcing pour une colonne
    fn edge_forcing_column(&self, grid: &Grid, col: usize, constraint: &[usize]) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();
        let column = grid.get_column(col)
            .ok_or_else(|| format!("Colonne {} non trouvée", col))?;

        if constraint.is_empty() {
            return Ok(deductions);
        }

        // Forcer depuis le haut
        let first_block = constraint[0];
        for (row, &cell) in column.iter().enumerate() {
            if cell == CellState::Filled {
                if row < first_block {
                    for fill_row in 0..first_block {
                        if column[fill_row] == CellState::Empty {
                            deductions.push(Deduction {
                                row: fill_row,
                                col,
                                state: CellState::Filled,
                            });
                        }
                    }
                    if first_block < column.len() && column[first_block] == CellState::Empty {
                        deductions.push(Deduction {
                            row: first_block,
                            col,
                            state: CellState::Crossed,
                        });
                    }
                    break;
                }
            }
        }

        // Forcer depuis le bas
        let last_block = *constraint.last().unwrap();
        let length = column.len();
        for row in (0..length).rev() {
            if column[row] == CellState::Filled {
                if row >= length - last_block {
                    for fill_row in (length - last_block)..length {
                        if column[fill_row] == CellState::Empty {
                            deductions.push(Deduction {
                                row: fill_row,
                                col,
                                state: CellState::Filled,
                            });
                        }
                    }
                    if length > last_block && column[length - last_block - 1] == CellState::Empty {
                        deductions.push(Deduction {
                            row: length - last_block - 1,
                            col,
                            state: CellState::Crossed,
                        });
                    }
                    break;
                }
            }
        }

        Ok(deductions)
    }

    /// Calcule la position minimale d'un bloc
    fn calculate_min_position(&self, constraint: &[usize], block_idx: usize) -> usize {
        let mut pos = 0;
        for i in 0..block_idx {
            pos += constraint[i] + 1; // Bloc + espace minimum
        }
        pos
    }

    /// Calcule la position maximale d'un bloc
    fn calculate_max_position(&self, constraint: &[usize], block_idx: usize, length: usize) -> usize {
        let mut pos = length;
        for i in (block_idx + 1)..constraint.len() {
            pos = pos.saturating_sub(constraint[i] + 1);
        }
        pos.saturating_sub(constraint[block_idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overlap_analysis_simple() {
        let mut grid = Grid::new(7, 1);
        let mut constraints = Constraints::new(7, 1);
        constraints.set_row_constraint(0, vec![5]);
        
        let analyzer = CrossAnalyzer::new();
        let deductions = analyzer.analyze(&grid, &constraints).unwrap();
        
        // Avec un bloc de 5 dans une ligne de 7, les positions 2, 3, 4 doivent être remplies
        assert!(deductions.iter().any(|d| d.col == 2 && d.state == CellState::Filled));
        assert!(deductions.iter().any(|d| d.col == 3 && d.state == CellState::Filled));
        assert!(deductions.iter().any(|d| d.col == 4 && d.state == CellState::Filled));
    }

    #[test]
    fn test_edge_forcing() {
        let mut grid = Grid::new(5, 1);
        let mut constraints = Constraints::new(5, 1);
        constraints.set_row_constraint(0, vec![3]);
        
        // Placer une case remplie près du début
        grid.set(0, 1, CellState::Filled).unwrap();
        
        let analyzer = CrossAnalyzer::new();
        let deductions = analyzer.analyze(&grid, &constraints).unwrap();
        
        // Le bloc de 3 doit être forcé à partir de la position 0
        assert!(deductions.iter().any(|d| d.col == 0 && d.state == CellState::Filled));
        assert!(deductions.iter().any(|d| d.col == 2 && d.state == CellState::Filled));
    }
}
