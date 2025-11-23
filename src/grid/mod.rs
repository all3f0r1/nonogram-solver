pub mod constraints;

pub use constraints::Constraints;

/// Représente l'état d'une case dans la grille
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CellState {
    /// Case vide (non résolue)
    Empty,
    /// Case noire (remplie)
    Filled,
    /// Case barrée (définitivement vide)
    Crossed,
}

/// Représente la grille du nonogramme
#[derive(Debug, Clone)]
pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<CellState>>,
}

impl Grid {
    /// Crée une nouvelle grille vide
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![CellState::Empty; width]; height];
        Self {
            width,
            height,
            cells,
        }
    }

    /// Retourne la largeur de la grille
    pub fn width(&self) -> usize {
        self.width
    }

    /// Retourne la hauteur de la grille
    pub fn height(&self) -> usize {
        self.height
    }

    /// Obtient l'état d'une case
    pub fn get(&self, row: usize, col: usize) -> Option<CellState> {
        self.cells.get(row).and_then(|r| r.get(col).copied())
    }

    /// Définit l'état d'une case
    pub fn set(&mut self, row: usize, col: usize, state: CellState) -> Result<(), String> {
        if row >= self.height || col >= self.width {
            return Err(format!("Position ({}, {}) hors limites", row, col));
        }
        self.cells[row][col] = state;
        Ok(())
    }

    /// Obtient une ligne complète
    pub fn get_row(&self, row: usize) -> Option<Vec<CellState>> {
        self.cells.get(row).cloned()
    }

    /// Obtient une colonne complète
    pub fn get_column(&self, col: usize) -> Option<Vec<CellState>> {
        if col >= self.width {
            return None;
        }
        Some(self.cells.iter().map(|row| row[col]).collect())
    }

    /// Définit une ligne complète
    pub fn set_row(&mut self, row: usize, states: Vec<CellState>) -> Result<(), String> {
        if row >= self.height {
            return Err(format!("Ligne {} hors limites", row));
        }
        if states.len() != self.width {
            return Err(format!(
                "La ligne doit contenir {} éléments, {} fournis",
                self.width,
                states.len()
            ));
        }
        self.cells[row] = states;
        Ok(())
    }

    /// Définit une colonne complète
    pub fn set_column(&mut self, col: usize, states: Vec<CellState>) -> Result<(), String> {
        if col >= self.width {
            return Err(format!("Colonne {} hors limites", col));
        }
        if states.len() != self.height {
            return Err(format!(
                "La colonne doit contenir {} éléments, {} fournis",
                self.height,
                states.len()
            ));
        }
        for (row, &state) in states.iter().enumerate() {
            self.cells[row][col] = state;
        }
        Ok(())
    }

    /// Compte le nombre de cases vides
    pub fn count_empty_cells(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == CellState::Empty)
            .count()
    }

    /// Compte le nombre de cases remplies
    pub fn count_filled_cells(&self) -> usize {
        self.cells
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&cell| cell == CellState::Filled)
            .count()
    }

    /// Vérifie si la grille est valide (pas de contradictions évidentes)
    pub fn is_valid(&self) -> bool {
        // Pour l'instant, toujours valide
        // Cette méthode sera étendue avec la détection de contradictions
        true
    }

    /// Crée une copie de la grille
    pub fn clone_grid(&self) -> Self {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_grid() {
        let grid = Grid::new(5, 5);
        assert_eq!(grid.width(), 5);
        assert_eq!(grid.height(), 5);
        assert_eq!(grid.get(0, 0), Some(CellState::Empty));
    }

    #[test]
    fn test_set_get() {
        let mut grid = Grid::new(5, 5);
        grid.set(2, 3, CellState::Filled).unwrap();
        assert_eq!(grid.get(2, 3), Some(CellState::Filled));
    }

    #[test]
    fn test_get_row() {
        let mut grid = Grid::new(5, 5);
        grid.set(1, 2, CellState::Filled).unwrap();
        let row = grid.get_row(1).unwrap();
        assert_eq!(row[2], CellState::Filled);
        assert_eq!(row.len(), 5);
    }

    #[test]
    fn test_get_column() {
        let mut grid = Grid::new(5, 5);
        grid.set(2, 1, CellState::Crossed).unwrap();
        let col = grid.get_column(1).unwrap();
        assert_eq!(col[2], CellState::Crossed);
        assert_eq!(col.len(), 5);
    }
}
