//! Module pour formater et exporter la grille dans différents formats
use crate::grid::{CellState, Grid};
use crate::solver::Deduction;
use serde_json::json;

/// Format de sortie pour la grille
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Format ASCII avec caractères (█, ✕, ·)
    Ascii,
    /// Format JSON structuré
    Json,
    /// Format tableau 2D (0, 1, -1)
    Array2D,
}

/// Formateur de grille pour différents formats de sortie
pub struct GridOutputFormatter;

impl GridOutputFormatter {
    /// Formate une grille selon le format spécifié
    pub fn format_grid(grid: &Grid, format: OutputFormat) -> String {
        match format {
            OutputFormat::Ascii => Self::format_ascii(grid),
            OutputFormat::Json => Self::format_json(grid),
            OutputFormat::Array2D => Self::format_array_2d(grid),
        }
    }

    /// Formate une grille avec les déductions appliquées
    pub fn format_grid_with_deductions(
        grid: &Grid,
        deductions: &[Deduction],
        format: OutputFormat,
    ) -> String {
        // Créer une copie de la grille et appliquer les déductions
        // Les déductions viennent du solveur et sont garanties valides
        let mut result_grid = grid.clone();
        for deduction in deductions {
            // Ignorer silencieusement les erreurs - les déductions devraient toujours être valides
            let _ = result_grid.set(deduction.row, deduction.col, deduction.state);
        }
        Self::format_grid(&result_grid, format)
    }

    /// Formate en ASCII avec caractères Unicode
    fn format_ascii(grid: &Grid) -> String {
        Self::for_each_cell_formatted(
            grid,
            |cell| match cell {
                Some(CellState::Filled) => "█".to_string(),
                Some(CellState::Crossed) => "✕".to_string(),
                Some(CellState::Empty) => "·".to_string(),
                None => "?".to_string(),
            },
            " ",
            "\n",
        )
    }

    /// Formate en JSON avec métadonnées
    fn format_json(grid: &Grid) -> String {
        let cells: Vec<Vec<&str>> = Self::for_each_cell(grid, |cell| match cell {
            Some(CellState::Filled) => "filled",
            Some(CellState::Crossed) => "crossed",
            Some(CellState::Empty) => "empty",
            None => "unknown",
        });

        let json_output = json!({
            "width": grid.width(),
            "height": grid.height(),
            "cells": cells,
            "stats": {
                "filled": grid.count_filled_cells(),
                "empty": grid.count_empty_cells(),
            }
        });

        serde_json::to_string_pretty(&json_output).unwrap_or_else(|e| {
            // En cas d'erreur de sérialisation, retourner un JSON valide avec un message d'erreur
            format!(r#"{{"error": "JSON serialization failed: {}"}}"#, e)
        })
    }

    /// Formate en tableau 2D simple (1 = filled, 0 = empty, -1 = crossed)
    fn format_array_2d(grid: &Grid) -> String {
        let rows: Vec<String> = (0..grid.height())
            .map(|row| {
                let cells: Vec<String> = (0..grid.width())
                    .map(|col| match grid.get(row, col) {
                        Some(CellState::Filled) => "1",
                        Some(CellState::Crossed) => "-1",
                        Some(CellState::Empty) => "0",
                        None => "?",
                    })
                    .map(|s| s.to_string())
                    .collect();
                format!("  [{}]", cells.join(", "))
            })
            .collect();

        format!("[\n{}\n]", rows.join(",\n"))
    }

    /// Helper générique pour itérer sur chaque cellule avec un formateur
    fn for_each_cell<F, T>(grid: &Grid, mut f: F) -> Vec<Vec<T>>
    where
        F: FnMut(Option<CellState>) -> T,
    {
        (0..grid.height())
            .map(|row| (0..grid.width()).map(|col| f(grid.get(row, col))).collect())
            .collect()
    }

    /// Helper générique pour formater avec des séparateurs personnalisés
    fn for_each_cell_formatted<F>(grid: &Grid, mut f: F, cell_sep: &str, row_sep: &str) -> String
    where
        F: FnMut(Option<CellState>) -> String,
    {
        let mut output = String::new();

        for row in 0..grid.height() {
            for col in 0..grid.width() {
                output.push_str(&f(grid.get(row, col)));
                if col < grid.width() - 1 {
                    output.push_str(cell_sep);
                }
            }
            output.push_str(row_sep);
        }

        output
    }

    /// Détecte le format depuis une extension de fichier
    pub fn detect_format_from_path(path: &str) -> OutputFormat {
        let path_lower = path.to_lowercase();

        if path_lower.ends_with(".json") {
            OutputFormat::Json
        } else if path_lower.ends_with(".array") {
            OutputFormat::Array2D
        } else if path_lower.ends_with(".txt") {
            OutputFormat::Array2D
        } else {
            // Défaut : ASCII pour .ascii ou extension inconnue
            OutputFormat::Ascii
        }
    }

    /// Détecte le format depuis une chaîne de caractères
    pub fn parse_format_str(format: &str) -> Option<OutputFormat> {
        match format.to_lowercase().as_str() {
            "ascii" | "text" | "txt" => Some(OutputFormat::Ascii),
            "json" => Some(OutputFormat::Json),
            "array" | "2d" | "matrix" => Some(OutputFormat::Array2D),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_ascii() {
        let mut grid = Grid::new(3, 3);
        grid.set(0, 0, CellState::Filled).unwrap();
        grid.set(1, 1, CellState::Crossed).unwrap();

        let output = GridOutputFormatter::format_ascii(&grid);
        assert!(output.contains("█"));
        assert!(output.contains("✕"));
        assert!(output.contains("·"));
    }

    #[test]
    fn test_format_json() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, CellState::Filled).unwrap();

        let output = GridOutputFormatter::format_json(&grid);
        assert!(output.contains("\"width\": 2"));
        assert!(output.contains("\"height\": 2"));
        assert!(output.contains("\"filled\""));
    }

    #[test]
    fn test_format_array_2d() {
        let mut grid = Grid::new(2, 2);
        grid.set(0, 0, CellState::Filled).unwrap();
        grid.set(0, 1, CellState::Crossed).unwrap();

        let output = GridOutputFormatter::format_array_2d(&grid);
        assert!(output.contains("[1, -1]"));
        assert!(output.contains("[0, 0]"));
    }

    #[test]
    fn test_detect_format_from_path() {
        assert_eq!(
            GridOutputFormatter::detect_format_from_path("output.json"),
            OutputFormat::Json
        );
        assert_eq!(
            GridOutputFormatter::detect_format_from_path("output.array"),
            OutputFormat::Array2D
        );
        assert_eq!(
            GridOutputFormatter::detect_format_from_path("output.txt"),
            OutputFormat::Array2D
        );
        // Correction : .ascii devrait retourner Ascii (le défaut)
        assert_eq!(
            GridOutputFormatter::detect_format_from_path("output.ascii"),
            OutputFormat::Ascii
        );
        assert_eq!(
            GridOutputFormatter::detect_format_from_path("output.unknown"),
            OutputFormat::Ascii
        );
    }

    #[test]
    fn test_parse_format_str() {
        assert_eq!(
            GridOutputFormatter::parse_format_str("ascii"),
            Some(OutputFormat::Ascii)
        );
        assert_eq!(
            GridOutputFormatter::parse_format_str("json"),
            Some(OutputFormat::Json)
        );
        assert_eq!(
            GridOutputFormatter::parse_format_str("array"),
            Some(OutputFormat::Array2D)
        );
        assert_eq!(GridOutputFormatter::parse_format_str("invalid"), None);
    }
}
