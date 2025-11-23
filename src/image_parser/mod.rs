mod grid_detector;

use image::{DynamicImage, GenericImageView, Rgba};
use crate::grid::{Grid, CellState};
use grid_detector::GridDetector;

/// Configuration pour le parseur d'image
#[derive(Debug, Clone)]
pub struct ParserConfig {
    /// Taille approximative d'une case en pixels
    pub cell_size: u32,
    /// Marge en pixels depuis le bord de l'image jusqu'à la grille
    pub margin_top: u32,
    pub margin_left: u32,
    /// Seuil pour déterminer si une case est noire (0-255)
    pub black_threshold: u8,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            cell_size: 20,
            margin_top: 50,
            margin_left: 50,
            black_threshold: 128,
        }
    }
}

/// Parse une image de nonogramme pour extraire l'état de la grille
pub struct ImageParser {
    config: ParserConfig,
}

impl ImageParser {
    /// Crée un nouveau parseur avec la configuration donnée
    pub fn new(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Crée un nouveau parseur avec la configuration par défaut
    pub fn with_default_config() -> Self {
        Self::new(ParserConfig::default())
    }

    /// Parse une image et extrait la grille
    pub fn parse_image(&self, image: &DynamicImage, width: usize, height: usize) -> Result<Grid, String> {
        let mut grid = Grid::new(width, height);

        for row in 0..height {
            for col in 0..width {
                let cell_state = self.detect_cell_state(image, row, col)?;
                grid.set(row, col, cell_state)?;
            }
        }

        Ok(grid)
    }

    /// Détecte l'état d'une case spécifique dans l'image
    fn detect_cell_state(&self, image: &DynamicImage, row: usize, col: usize) -> Result<CellState, String> {
        // Calculer le centre de la case
        let center_x = self.config.margin_left + (col as u32 * self.config.cell_size) + (self.config.cell_size / 2);
        let center_y = self.config.margin_top + (row as u32 * self.config.cell_size) + (self.config.cell_size / 2);

        // Vérifier que le point est dans l'image
        if center_x >= image.width() || center_y >= image.height() {
            return Err(format!(
                "Position ({}, {}) hors de l'image ({}x{})",
                center_x, center_y, image.width(), image.height()
            ));
        }

        // Échantillonner plusieurs points dans la case pour plus de robustesse
        let sample_points = vec![
            (center_x, center_y),
            (center_x - self.config.cell_size / 4, center_y),
            (center_x + self.config.cell_size / 4, center_y),
            (center_x, center_y - self.config.cell_size / 4),
            (center_x, center_y + self.config.cell_size / 4),
        ];

        let mut black_count = 0;
        let mut crossed_count = 0;
        let mut total_samples = 0;

        for (x, y) in sample_points {
            if x < image.width() && y < image.height() {
                let pixel = image.get_pixel(x, y);
                let state = self.classify_pixel(pixel);
                
                match state {
                    CellState::Filled => black_count += 1,
                    CellState::Crossed => crossed_count += 1,
                    CellState::Empty => {}
                }
                total_samples += 1;
            }
        }

        // Décision basée sur la majorité
        if black_count > total_samples / 2 {
            Ok(CellState::Filled)
        } else if crossed_count > total_samples / 2 {
            Ok(CellState::Crossed)
        } else {
            Ok(CellState::Empty)
        }
    }

    /// Classifie un pixel en état de case
    fn classify_pixel(&self, pixel: Rgba<u8>) -> CellState {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        // Calculer la luminosité
        let brightness = (r as u32 + g as u32 + b as u32) / 3;

        // Si le pixel est sombre, c'est probablement une case noire
        if brightness < self.config.black_threshold as u32 {
            return CellState::Filled;
        }

        // Détecter une croix (rouge ou bleu, par exemple)
        // Pour simplifier, on considère qu'une case barrée a une couleur non-neutre
        let color_variance = ((r as i32 - g as i32).abs() + (g as i32 - b as i32).abs() + (r as i32 - b as i32).abs()) as u32;
        
        if color_variance > 50 && brightness > self.config.black_threshold as u32 {
            return CellState::Crossed;
        }

        CellState::Empty
    }

    /// Charge une image depuis un fichier
    pub fn load_image(path: &str) -> Result<DynamicImage, String> {
        image::open(path).map_err(|e| format!("Erreur lors du chargement de l'image: {}", e))
    }

    /// Détecte automatiquement les paramètres de la grille
    pub fn auto_detect_config(image: &DynamicImage, width: usize, height: usize) -> Result<ParserConfig, String> {
        // Utiliser le détecteur avancé de grille
        match GridDetector::detect_grid_params(image, width, height) {
            Ok((cell_size, margin_left, margin_top)) => {
                Ok(ParserConfig {
                    cell_size,
                    margin_top,
                    margin_left,
                    black_threshold: 128,
                })
            }
            Err(_) => {
                // Fallback sur l'heuristique simple si la détection échoue
                eprintln!("Avertissement: Détection avancée échouée, utilisation de l'heuristique simple");
                
                let img_width = image.width();
                let img_height = image.height();

                let estimated_cell_width = img_width / (width as u32 + 5);
                let estimated_cell_height = img_height / (height as u32 + 5);
                let cell_size = estimated_cell_width.min(estimated_cell_height);

                let margin_left = (img_width - cell_size * width as u32) / 2;
                let margin_top = (img_height - cell_size * height as u32) / 2;

                Ok(ParserConfig {
                    cell_size,
                    margin_top,
                    margin_left,
                    black_threshold: 128,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_config_default() {
        let config = ParserConfig::default();
        assert_eq!(config.cell_size, 20);
        assert_eq!(config.black_threshold, 128);
    }

    #[test]
    fn test_classify_pixel() {
        let parser = ImageParser::with_default_config();
        
        // Pixel noir
        let black_pixel = Rgba([0, 0, 0, 255]);
        assert_eq!(parser.classify_pixel(black_pixel), CellState::Filled);
        
        // Pixel blanc
        let white_pixel = Rgba([255, 255, 255, 255]);
        assert_eq!(parser.classify_pixel(white_pixel), CellState::Empty);
    }
}
