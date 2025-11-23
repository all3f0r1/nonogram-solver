use crate::grid::{Grid, CellState, Constraints};
use crate::solver::Deduction;

/// Heuristiques avancées pour la résolution de nonogrammes
/// 
/// Implémente des techniques comme glue method, mercury method, etc.
pub struct AdvancedHeuristics;

impl AdvancedHeuristics {
    pub fn new() -> Self {
        Self
    }

    /// Applique toutes les heuristiques avancées
    pub fn apply(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        // Glue method
        deductions.extend(self.glue_method(grid, constraints)?);

        // Mercury method
        deductions.extend(self.mercury_method(grid, constraints)?);

        // Joining and splitting
        deductions.extend(self.joining_splitting(grid, constraints)?);

        // Puncturing
        deductions.extend(self.puncturing(grid, constraints)?);

        // Dédupliquer
        deductions.sort_by_key(|d| (d.row, d.col));
        deductions.dedup_by_key(|d| (d.row, d.col));

        Ok(deductions)
    }

    /// Glue Method: Colle les blocs qui doivent être connectés
    fn glue_method(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        // Pour chaque ligne
        for row in 0..grid.height() {
            let line = grid.get_row(row)
                .ok_or_else(|| format!("Ligne {} non trouvée", row))?;
            let row_constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
            
            let row_deductions = self.glue_method_line(row, &line, row_constraint, true)?;
            deductions.extend(row_deductions);
        }

        // Pour chaque colonne
        for col in 0..grid.width() {
            let column = grid.get_column(col)
                .ok_or_else(|| format!("Colonne {} non trouvée", col))?;
            let col_constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
            
            let col_deductions = self.glue_method_line(col, &column, col_constraint, false)?;
            deductions.extend(col_deductions);
        }

        Ok(deductions)
    }

    /// Glue method pour une ligne ou colonne
    fn glue_method_line(&self, index: usize, line: &[CellState], constraint: &[usize], is_row: bool) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        if constraint.is_empty() {
            return Ok(deductions);
        }

        let blocks = self.find_filled_blocks(line);

        for (block_start, block_size) in blocks {
            for &constraint_size in constraint.iter() {
                if block_size > constraint_size / 2 && block_size < constraint_size {
                    let missing = constraint_size - block_size;
                    
                    // Essayer d'étendre à gauche
                    if block_start >= missing {
                        let can_extend_left = (block_start - missing..block_start)
                            .all(|i| line[i] == CellState::Empty || line[i] == CellState::Filled);
                        
                        if can_extend_left {
                            for i in block_start - missing..block_start {
                                if line[i] == CellState::Empty {
                                    if is_row {
                                        deductions.push(Deduction {
                                            row: index,
                                            col: i,
                                            state: CellState::Filled,
                                        });
                                    } else {
                                        deductions.push(Deduction {
                                            row: i,
                                            col: index,
                                            state: CellState::Filled,
                                        });
                                    }
                                }
                            }
                        }
                    }

                    // Essayer d'étendre à droite
                    let block_end = block_start + block_size;
                    if block_end + missing <= line.len() {
                        let can_extend_right = (block_end..block_end + missing)
                            .all(|i| line[i] == CellState::Empty || line[i] == CellState::Filled);
                        
                        if can_extend_right {
                            for i in block_end..block_end + missing {
                                if line[i] == CellState::Empty {
                                    if is_row {
                                        deductions.push(Deduction {
                                            row: index,
                                            col: i,
                                            state: CellState::Filled,
                                        });
                                    } else {
                                        deductions.push(Deduction {
                                            row: i,
                                            col: index,
                                            state: CellState::Filled,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Mercury Method: Simule le "coulage" des blocs
    fn mercury_method(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        // Pour chaque ligne
        for row in 0..grid.height() {
            let line = grid.get_row(row)
                .ok_or_else(|| format!("Ligne {} non trouvée", row))?;
            let row_constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
            
            let row_deductions = self.mercury_method_line(row, &line, row_constraint, true)?;
            deductions.extend(row_deductions);
        }

        // Pour chaque colonne
        for col in 0..grid.width() {
            let column = grid.get_column(col)
                .ok_or_else(|| format!("Colonne {} non trouvée", col))?;
            let col_constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
            
            let col_deductions = self.mercury_method_line(col, &column, col_constraint, false)?;
            deductions.extend(col_deductions);
        }

        Ok(deductions)
    }

    /// Mercury method pour une ligne ou colonne
    fn mercury_method_line(&self, index: usize, line: &[CellState], constraint: &[usize], is_row: bool) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        if constraint.is_empty() {
            return Ok(deductions);
        }

        let total_required = constraint.iter().sum::<usize>() + constraint.len().saturating_sub(1);
        if total_required > line.len() {
            return Ok(deductions);
        }

        // Pour chaque contrainte, calculer les positions min et max
        for (block_idx, &block_size) in constraint.iter().enumerate() {
            let mut min_pos = 0;
            for i in 0..block_idx {
                min_pos += constraint[i] + 1;
            }

            let mut max_pos = line.len() - block_size;
            for i in (block_idx + 1)..constraint.len() {
                max_pos = max_pos.saturating_sub(constraint[i] + 1);
            }

            // Si le bloc ne peut pas "couler" beaucoup, il y a chevauchement
            if max_pos < min_pos + block_size {
                let overlap_start = max_pos;
                let overlap_end = (min_pos + block_size).min(line.len());
                
                for pos in overlap_start..overlap_end {
                    if line[pos] == CellState::Empty {
                        if is_row {
                            deductions.push(Deduction {
                                row: index,
                                col: pos,
                                state: CellState::Filled,
                            });
                        } else {
                            deductions.push(Deduction {
                                row: pos,
                                col: index,
                                state: CellState::Filled,
                            });
                        }
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Joining and Splitting: Joint ou sépare les blocs
    fn joining_splitting(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        // Pour chaque ligne
        for row in 0..grid.height() {
            let line = grid.get_row(row)
                .ok_or_else(|| format!("Ligne {} non trouvée", row))?;
            let row_constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
            
            let blocks = self.find_filled_blocks(&line);
            
            if blocks.len() > row_constraint.len() {
                for i in 0..blocks.len() - 1 {
                    let (start1, size1) = blocks[i];
                    let (start2, _) = blocks[i + 1];
                    let block1_end = start1 + size1;
                    
                    if start2 == block1_end + 1 {
                        if line[block1_end] == CellState::Empty {
                            deductions.push(Deduction {
                                row,
                                col: block1_end,
                                state: CellState::Filled,
                            });
                        }
                    }
                }
            }
        }

        // Pour chaque colonne
        for col in 0..grid.width() {
            let column = grid.get_column(col)
                .ok_or_else(|| format!("Colonne {} non trouvée", col))?;
            let col_constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
            
            let blocks = self.find_filled_blocks(&column);
            
            if blocks.len() > col_constraint.len() {
                for i in 0..blocks.len() - 1 {
                    let (start1, size1) = blocks[i];
                    let (start2, _) = blocks[i + 1];
                    let block1_end = start1 + size1;
                    
                    if start2 == block1_end + 1 {
                        if column[block1_end] == CellState::Empty {
                            deductions.push(Deduction {
                                row: block1_end,
                                col,
                                state: CellState::Filled,
                            });
                        }
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Puncturing: Identifie les cases qui doivent être barrées
    fn puncturing(&self, grid: &Grid, constraints: &Constraints) -> Result<Vec<Deduction>, String> {
        let mut deductions = Vec::new();

        // Pour chaque ligne
        for row in 0..grid.height() {
            let line = grid.get_row(row)
                .ok_or_else(|| format!("Ligne {} non trouvée", row))?;
            let row_constraint = constraints.get_row_constraint(row)
                .ok_or_else(|| format!("Contrainte de ligne {} non trouvée", row))?;
            
            let blocks = self.find_filled_blocks(&line);
            
            if blocks.len() == row_constraint.len() {
                let all_match = blocks.iter().zip(row_constraint.iter())
                    .all(|((_, size), &constraint)| *size == constraint);
                
                if all_match {
                    for col in 0..line.len() {
                        if line[col] == CellState::Empty {
                            deductions.push(Deduction {
                                row,
                                col,
                                state: CellState::Crossed,
                            });
                        }
                    }
                }
            }
        }

        // Pour chaque colonne
        for col in 0..grid.width() {
            let column = grid.get_column(col)
                .ok_or_else(|| format!("Colonne {} non trouvée", col))?;
            let col_constraint = constraints.get_column_constraint(col)
                .ok_or_else(|| format!("Contrainte de colonne {} non trouvée", col))?;
            
            let blocks = self.find_filled_blocks(&column);
            
            if blocks.len() == col_constraint.len() {
                let all_match = blocks.iter().zip(col_constraint.iter())
                    .all(|((_, size), &constraint)| *size == constraint);
                
                if all_match {
                    for row in 0..column.len() {
                        if column[row] == CellState::Empty {
                            deductions.push(Deduction {
                                row,
                                col,
                                state: CellState::Crossed,
                            });
                        }
                    }
                }
            }
        }

        Ok(deductions)
    }

    /// Trouve tous les blocs de cases remplies dans une ligne
    fn find_filled_blocks(&self, line: &[CellState]) -> Vec<(usize, usize)> {
        let mut blocks = Vec::new();
        let mut in_block = false;
        let mut block_start = 0;
        let mut block_size = 0;

        for (i, &cell) in line.iter().enumerate() {
            if cell == CellState::Filled {
                if !in_block {
                    in_block = true;
                    block_start = i;
                    block_size = 1;
                } else {
                    block_size += 1;
                }
            } else if in_block {
                blocks.push((block_start, block_size));
                in_block = false;
            }
        }

        if in_block {
            blocks.push((block_start, block_size));
        }

        blocks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_filled_blocks() {
        let heuristics = AdvancedHeuristics::new();
        let line = vec![
            CellState::Empty,
            CellState::Filled,
            CellState::Filled,
            CellState::Empty,
            CellState::Filled,
        ];
        
        let blocks = heuristics.find_filled_blocks(&line);
        assert_eq!(blocks.len(), 2);
        assert_eq!(blocks[0], (1, 2));
        assert_eq!(blocks[1], (4, 1));
    }

    #[test]
    fn test_puncturing() {
        let mut grid = Grid::new(5, 1);
        let mut constraints = Constraints::new(5, 1);
        constraints.set_row_constraint(0, vec![2]);
        
        // Placer un bloc complet de 2
        grid.set(0, 1, CellState::Filled).unwrap();
        grid.set(0, 2, CellState::Filled).unwrap();
        
        let heuristics = AdvancedHeuristics::new();
        let deductions = heuristics.apply(&grid, &constraints).unwrap();
        
        // Les autres cases devraient être barrées
        assert!(deductions.iter().any(|d| d.col == 0 && d.state == CellState::Crossed));
        assert!(deductions.iter().any(|d| d.col == 3 && d.state == CellState::Crossed));
        assert!(deductions.iter().any(|d| d.col == 4 && d.state == CellState::Crossed));
    }
}
