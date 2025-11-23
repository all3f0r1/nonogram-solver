use serde::{Deserialize, Serialize};

/// Représente les contraintes d'un nonogramme
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraints {
    /// Largeur de la grille
    pub width: usize,
    /// Hauteur de la grille
    pub height: usize,
    /// Contraintes pour chaque ligne (nombre de cases noires consécutives)
    pub rows: Vec<Vec<usize>>,
    /// Contraintes pour chaque colonne (nombre de cases noires consécutives)
    pub columns: Vec<Vec<usize>>,
}

impl Constraints {
    /// Crée de nouvelles contraintes
    pub fn new(width: usize, height: usize, rows: Vec<Vec<usize>>, columns: Vec<Vec<usize>>) -> Result<Self, String> {
        if rows.len() != height {
            return Err(format!(
                "Le nombre de contraintes de lignes ({}) ne correspond pas à la hauteur ({})",
                rows.len(),
                height
            ));
        }
        if columns.len() != width {
            return Err(format!(
                "Le nombre de contraintes de colonnes ({}) ne correspond pas à la largeur ({})",
                columns.len(),
                width
            ));
        }

        // Vérifier que les contraintes sont valides
        for (i, row_constraint) in rows.iter().enumerate() {
            let min_width = Self::min_line_length(row_constraint);
            if min_width > width {
                return Err(format!(
                    "La contrainte de la ligne {} ({:?}) nécessite au moins {} cases, mais la grille n'a que {} colonnes",
                    i, row_constraint, min_width, width
                ));
            }
        }

        for (i, col_constraint) in columns.iter().enumerate() {
            let min_height = Self::min_line_length(col_constraint);
            if min_height > height {
                return Err(format!(
                    "La contrainte de la colonne {} ({:?}) nécessite au moins {} cases, mais la grille n'a que {} lignes",
                    i, col_constraint, min_height, height
                ));
            }
        }

        Ok(Self {
            width,
            height,
            rows,
            columns,
        })
    }

    /// Charge les contraintes depuis un fichier JSON
    pub fn from_json_file(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Erreur lors de la lecture du fichier: {}", e))?;
        let constraints: Constraints = serde_json::from_str(&content)
            .map_err(|e| format!("Erreur lors du parsing JSON: {}", e))?;
        
        // Valider les contraintes
        Self::new(
            constraints.width,
            constraints.height,
            constraints.rows,
            constraints.columns,
        )
    }

    /// Calcule la longueur minimale nécessaire pour une ligne avec les contraintes données
    fn min_line_length(constraint: &[usize]) -> usize {
        if constraint.is_empty() {
            return 0;
        }
        // Somme des blocs + espaces minimaux entre eux (1 espace entre chaque bloc)
        constraint.iter().sum::<usize>() + constraint.len() - 1
    }

    /// Obtient les contraintes d'une ligne
    pub fn get_row_constraint(&self, row: usize) -> Option<&Vec<usize>> {
        self.rows.get(row)
    }

    /// Obtient les contraintes d'une colonne
    pub fn get_column_constraint(&self, col: usize) -> Option<&Vec<usize>> {
        self.columns.get(col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_constraints() {
        let rows = vec![vec![3], vec![1, 1], vec![2]];
        let columns = vec![vec![1], vec![2], vec![1, 1]];
        let constraints = Constraints::new(3, 3, rows, columns).unwrap();
        assert_eq!(constraints.width, 3);
        assert_eq!(constraints.height, 3);
    }

    #[test]
    fn test_invalid_constraints() {
        let rows = vec![vec![10]]; // Nécessite 10 cases
        let columns = vec![vec![1], vec![1], vec![1]];
        let result = Constraints::new(3, 1, rows, columns);
        assert!(result.is_err());
    }

    #[test]
    fn test_min_line_length() {
        assert_eq!(Constraints::min_line_length(&[3]), 3);
        assert_eq!(Constraints::min_line_length(&[3, 2]), 6); // 3 + 1 + 2
        assert_eq!(Constraints::min_line_length(&[1, 1, 1]), 5); // 1 + 1 + 1 + 1 + 1
        assert_eq!(Constraints::min_line_length(&[]), 0);
    }
}
