use image::{DynamicImage, GenericImageView, Rgb, ImageBuffer, Luma};
use regex::Regex;
use crate::grid::Constraints;
use imageproc::drawing::draw_line_segment_mut;

#[cfg(feature = "ocr")]
use tesseract::Tesseract;

/// Extracteur avancé de contraintes avec détection automatique
pub struct AdvancedConstraintExtractor;

impl AdvancedConstraintExtractor {
    /// Extrait automatiquement les contraintes d'une image de nonogramme
    /// Détecte automatiquement les régions de contraintes et la taille de la grille
    pub fn extract_auto(image: &DynamicImage) -> Result<Constraints, String> {
        // Étape 1: Détecter la grille
        let (grid_x, grid_y, cell_size, grid_width, grid_height) = Self::detect_grid(image)?;
        
        // Étape 2: Détecter les régions de contraintes
        let row_region = (0, grid_y, grid_x, (grid_height as u32) * cell_size);
        let col_region = (grid_x, 0, (grid_width as u32) * cell_size, grid_y);
        
        // Étape 3: Extraire les contraintes
        Self::extract_from_regions(image, row_region, col_region, grid_width as usize, grid_height as usize)
    }
    
    /// Détecte automatiquement la grille dans l'image
    fn detect_grid(image: &DynamicImage) -> Result<(u32, u32, u32, usize, usize), String> {
        let width = image.width();
        let height = image.height();
        
        // Convertir en niveaux de gris
        let gray = image.to_luma8();
        
        // Détecter les lignes horizontales et verticales
        let horizontal_lines = Self::detect_lines(&gray, true);
        let vertical_lines = Self::detect_lines(&gray, false);
        
        if horizontal_lines.len() < 2 || vertical_lines.len() < 2 {
            return Err("Impossible de détecter la grille".to_string());
        }
        
        // Calculer la taille des cellules
        let cell_size_h = if horizontal_lines.len() > 1 {
            (horizontal_lines[1] - horizontal_lines[0]) as u32
        } else {
            return Err("Pas assez de lignes horizontales détectées".to_string());
        };
        
        let cell_size_v = if vertical_lines.len() > 1 {
            (vertical_lines[1] - vertical_lines[0]) as u32
        } else {
            return Err("Pas assez de lignes verticales détectées".to_string());
        };
        
        // Utiliser la moyenne des deux
        let cell_size = (cell_size_h + cell_size_v) / 2;
        
        // Position de départ de la grille
        let grid_x = vertical_lines[0] as u32;
        let grid_y = horizontal_lines[0] as u32;
        
        // Dimensions de la grille
        let grid_width = vertical_lines.len() - 1;
        let grid_height = horizontal_lines.len() - 1;
        
        Ok((grid_x, grid_y, cell_size, grid_width, grid_height))
    }
    
    /// Détecte les lignes dans l'image
    fn detect_lines(image: &ImageBuffer<Luma<u8>, Vec<u8>>, horizontal: bool) -> Vec<usize> {
        let (width, height) = image.dimensions();
        let mut lines = Vec::new();
        
        if horizontal {
            // Détecter les lignes horizontales
            for y in 0..height {
                let mut dark_pixels = 0;
                for x in 0..width {
                    let pixel = image.get_pixel(x, y);
                    if pixel[0] < 128 {  // Pixel sombre
                        dark_pixels += 1;
                    }
                }
                
                // Si plus de 50% des pixels sont sombres, c'est une ligne
                if dark_pixels as f32 / width as f32 > 0.5 {
                    // Éviter les doublons (lignes épaisses)
                    if lines.is_empty() || y as usize - lines[lines.len() - 1] > 5 {
                        lines.push(y as usize);
                    }
                }
            }
        } else {
            // Détecter les lignes verticales
            for x in 0..width {
                let mut dark_pixels = 0;
                for y in 0..height {
                    let pixel = image.get_pixel(x, y);
                    if pixel[0] < 128 {
                        dark_pixels += 1;
                    }
                }
                
                if dark_pixels as f32 / height as f32 > 0.5 {
                    if lines.is_empty() || x as usize - lines[lines.len() - 1] > 5 {
                        lines.push(x as usize);
                    }
                }
            }
        }
        
        lines
    }
    
    /// Extrait les contraintes depuis des régions spécifiques
    pub fn extract_from_regions(
        image: &DynamicImage,
        row_region: (u32, u32, u32, u32),
        col_region: (u32, u32, u32, u32),
        grid_width: usize,
        grid_height: usize,
    ) -> Result<Constraints, String> {
        #[cfg(feature = "ocr")]
        {
            // Extraire les régions
            let row_img = image.crop_imm(row_region.0, row_region.1, row_region.2, row_region.3);
            let col_img = image.crop_imm(col_region.0, col_region.1, col_region.2, col_region.3);
            
            // Prétraiter les images pour améliorer l'OCR
            let row_img_processed = Self::preprocess_for_ocr(&row_img);
            let col_img_processed = Self::preprocess_for_ocr(&col_img);
            
            // Extraire le texte
            let row_text = Self::extract_text_from_image(&row_img_processed)?;
            let col_text = Self::extract_text_from_image(&col_img_processed)?;
            
            // Parser les contraintes
            let rows = Self::parse_constraint_list(&row_text, grid_height)?;
            let columns = Self::parse_constraint_list(&col_text, grid_width)?;
            
            Constraints::new(grid_width, grid_height, rows, columns)
        }
        
        #[cfg(not(feature = "ocr"))]
        {
            let _ = (image, row_region, col_region, grid_width, grid_height);
            Err("La fonctionnalité OCR n'est pas activée. Recompilez avec --features ocr".to_string())
        }
    }
    
    /// Prétraite l'image pour améliorer la reconnaissance OCR
    fn preprocess_for_ocr(image: &DynamicImage) -> DynamicImage {
        // Convertir en niveaux de gris
        let gray = image.to_luma8();
        
        // Binarisation (seuil adaptatif)
        let mut binary = ImageBuffer::new(gray.width(), gray.height());
        
        for (x, y, pixel) in gray.enumerate_pixels() {
            let value = if pixel[0] > 128 { 255 } else { 0 };
            binary.put_pixel(x, y, Luma([value]));
        }
        
        // Augmenter le contraste
        let mut enhanced = ImageBuffer::new(binary.width(), binary.height());
        
        for (x, y, pixel) in binary.enumerate_pixels() {
            // Appliquer un filtre de contraste
            let value = if pixel[0] > 200 {
                255
            } else if pixel[0] < 50 {
                0
            } else {
                pixel[0]
            };
            enhanced.put_pixel(x, y, Luma([value]));
        }
        
        DynamicImage::ImageLuma8(enhanced)
    }
    
    #[cfg(feature = "ocr")]
    fn extract_text_from_image(image: &DynamicImage) -> Result<String, String> {
        let gray = image.to_luma8();
        
        let mut tess = Tesseract::new(None, Some("eng"))
            .map_err(|e| format!("Erreur d'initialisation de Tesseract: {}", e))?;
        
        // Configuration optimisée pour les chiffres
        tess.set_variable("tessedit_char_whitelist", "0123456789 \n")
            .map_err(|e| format!("Erreur de configuration Tesseract: {}", e))?;
        
        // Mode de segmentation: traiter l'image comme un seul bloc de texte
        tess.set_variable("tessedit_pageseg_mode", "6")
            .map_err(|e| format!("Erreur de configuration Tesseract: {}", e))?;
        
        tess.set_image_from_mem(&gray.as_raw())
            .map_err(|e| format!("Erreur lors du chargement de l'image: {}", e))?
            .get_text()
            .map_err(|e| format!("Erreur lors de l'extraction du texte: {}", e))
    }
    
    #[cfg(feature = "ocr")]
    fn parse_constraint_list(text: &str, expected_count: usize) -> Result<Vec<Vec<usize>>, String> {
        let re = Regex::new(r"(\d+(?:\s+\d+)*)").unwrap();
        let mut constraints = Vec::new();
        
        for cap in re.captures_iter(text) {
            if let Some(matched) = cap.get(1) {
                let numbers: Vec<usize> = matched
                    .as_str()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                if !numbers.is_empty() {
                    constraints.push(numbers);
                }
            }
        }
        
        // Si pas assez de contraintes, ajouter des contraintes vides
        while constraints.len() < expected_count {
            constraints.push(vec![]);
        }
        
        // Si trop de contraintes, tronquer
        if constraints.len() > expected_count {
            constraints.truncate(expected_count);
        }
        
        Ok(constraints)
    }
    
    /// Méthode simplifiée sans OCR qui utilise une heuristique basée sur l'image
    #[cfg(not(feature = "ocr"))]
    pub fn extract_from_image_heuristic(image: &DynamicImage) -> Result<Constraints, String> {
        // Détecter la grille
        let (grid_x, grid_y, cell_size, grid_width, grid_height) = Self::detect_grid(image)?;
        
        // Créer des contraintes vides (l'utilisateur devra les fournir)
        let rows = vec![vec![]; grid_height];
        let columns = vec![vec![]; grid_width];
        
        Constraints::new(grid_width, grid_height, rows, columns)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_detect_lines() {
        // Créer une image de test avec des lignes
        let mut img = ImageBuffer::new(100, 100);
        
        // Dessiner des lignes horizontales
        for x in 0..100 {
            img.put_pixel(x, 10, Luma([0]));
            img.put_pixel(x, 50, Luma([0]));
            img.put_pixel(x, 90, Luma([0]));
        }
        
        let lines = AdvancedConstraintExtractor::detect_lines(&img, true);
        assert!(lines.len() >= 3);
    }
}
