//! Module pour la saisie interactive des contraintes en ligne de commande
use crate::grid::Constraints;
use std::io::{self, Write};

/// Structure pour la saisie interactive des contraintes
pub struct InteractiveInput;

impl InteractiveInput {
    /// Lance une session interactive pour saisir les contraintes
    pub fn input_constraints(width: usize, height: usize) -> Result<Constraints, String> {
        println!("📏 Dimensions de la grille : {}x{}", width, height);
        println!();
        println!("📝 Saisie des contraintes");
        println!("   - Entrez les nombres séparés par des espaces (ex: '3 1 2' pour [3, 1, 2])");
        println!("   - Appuyez sur Entrée pour une contrainte vide []");
        println!("   - Tapez 'quit' ou 'q' pour annuler");
        println!();

        println!("═══════════════════════════════════════════════════════════");
        println!("  Contraintes des LIGNES");
        println!("═══════════════════════════════════════════════════════════");

        let mut rows = Vec::new();
        for i in 0..height {
            let label = format!("Ligne {:2}", i);
            match Self::input_single_line(label, width) {
                Ok(Some(constraints)) => rows.push(constraints),
                Ok(None) => return Err("Saisie annulée par l'utilisateur".to_string()),
                Err(e) => return Err(e),
            }
        }

        println!();
        println!("═══════════════════════════════════════════════════════════");
        println!("  Contraintes des COLONNES");
        println!("═══════════════════════════════════════════════════════════");

        let mut columns = Vec::new();
        for i in 0..width {
            let label = format!("Colonne {:2}", i);
            match Self::input_single_line(label, height) {
                Ok(Some(constraints)) => columns.push(constraints),
                Ok(None) => return Err("Saisie annulée par l'utilisateur".to_string()),
                Err(e) => return Err(e),
            }
        }

        println!();
        println!("✅ Contraintes saisies avec succès !");

        Constraints::new(width, height, rows, columns).map_err(|e| format!("Contraintes invalides: {}", e))
    }

    /// Saisit une ligne/colonne de contraintes avec validation
    fn input_single_line(label: String, max_size: usize) -> Result<Option<Vec<usize>>, String> {
        loop {
            print!("  {} : ", label);
            io::stdout()
                .flush()
                .map_err(|e| format!("Erreur E/S: {}", e))?;

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => return Ok(None), // EOF (Ctrl+D)
                Err(e) => return Err(format!("Erreur de lecture: {}", e)),
                _ => {}
            }

            let input = input.trim();

            // Commandes de sortie
            if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("q") {
                return Ok(None);
            }

            // Entrée vide = contrainte vide
            if input.is_empty() {
                return Ok(Some(vec![]));
            }

            // Parser les nombres
            let constraints: Result<Vec<usize>, _> = input
                .split_whitespace()
                .map(|s| s.parse::<usize>())
                .collect();

            match constraints {
                Ok(nums) => {
                    // Validation : vérifier que les contraintes sont cohérentes
                    if let Err(e) = Self::validate_constraints(&nums, max_size) {
                        println!("     ⚠️  Attention: {}", e);
                        print!("     Accepter quand même ? [Y/n]: ");
                        io::stdout()
                            .flush()
                            .map_err(|e| format!("Erreur E/S: {}", e))?;

                        let mut confirm = String::new();
                        if io::stdin().read_line(&mut confirm).is_err() {
                            // Si on ne peut pas lire la confirmation, on accepte par défaut
                            return Ok(Some(nums));
                        }
                        let confirm = confirm.trim().to_lowercase();

                        if confirm == "n" || confirm == "no" || confirm == "non" {
                            continue; // Recommencer
                        }
                    }
                    return Ok(Some(nums));
                }
                Err(_) => {
                    println!(
                        "     ❌ Format invalide. Entrez des nombres séparés par des espaces."
                    );
                }
            }
        }
    }

    /// Valide que les contraintes sont cohérentes avec la taille de la grille
    fn validate_constraints(constraints: &[usize], max_size: usize) -> Result<(), String> {
        if constraints.is_empty() {
            return Ok(());
        }

        // Vérifier qu'il n'y a pas de zéros (invalides)
        if constraints.contains(&0) {
            return Err("Les zéros ne sont pas valides dans les contraintes".to_string());
        }

        // Calculer la taille minimale requise avec protection contre le overflow
        let sum: usize = constraints.iter().sum();
        let min_length = sum
            .checked_add(constraints.len().saturating_sub(1))
            .ok_or_else(|| "Contraintes trop grandes (overflow)".to_string())?;

        if min_length > max_size {
            return Err(format!(
                "Les contraintes nécessitent au moins {} cases, mais la grille en a {}",
                min_length, max_size
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_constraints_valid() {
        assert!(InteractiveInput::validate_constraints(&[3, 2], 10).is_ok());
        assert!(InteractiveInput::validate_constraints(&[], 5).is_ok());
    }

    #[test]
    fn test_validate_constraints_too_long() {
        assert!(InteractiveInput::validate_constraints(&[5, 5], 10).is_err());
        assert!(InteractiveInput::validate_constraints(&[10], 5).is_err());
    }

    #[test]
    fn test_validate_constraints_zero() {
        assert!(InteractiveInput::validate_constraints(&[3, 0, 2], 10).is_err());
    }
}
