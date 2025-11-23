use image::DynamicImage;
use regex::Regex;
use crate::grid::Constraints;

#[cfg(feature = "ocr")]
use tesseract::Tesseract;

/// Module d'extraction de contraintes par OCR
pub struct ConstraintExtractor;

impl ConstraintExtractor {
    /// Extrait les contraintes d'une image de nonogramme en utilisant l'OCR
    #[cfg(feature = "ocr")]
    pub fn extract_from_image(image: &DynamicImage, width: usize, height: usize) -> Result<Constraints, String> {
        // Convertir l'image en format compatible avec Tesseract
        let gray = image.to_luma8();
        
        // Initialiser Tesseract
        let mut tess = Tesseract::new(None, Some("eng"))
            .map_err(|e| format!("Erreur d'initialisation de Tesseract: {}", e))?;
        
        // Configurer pour reconnaître uniquement les chiffres
        tess.set_variable("tessedit_char_whitelist", "0123456789 ")
            .map_err(|e| format!("Erreur de configuration Tesseract: {}", e))?;
        
        // Extraire le texte de l'image
        let text = tess
            .set_image_from_mem(&gray.as_raw())
            .map_err(|e| format!("Erreur lors du chargement de l'image: {}", e))?
            .get_text()
            .map_err(|e| format!("Erreur lors de l'extraction du texte: {}", e))?;
        
        // Parser le texte pour extraire les contraintes
        Self::parse_constraints_from_text(&text, width, height)
    }
    
    /// Version sans OCR qui retourne une erreur explicative
    #[cfg(not(feature = "ocr"))]
    pub fn extract_from_image(_image: &DynamicImage, _width: usize, _height: usize) -> Result<Constraints, String> {
        Err("La fonctionnalité OCR n'est pas activée. Recompilez avec --features ocr".to_string())
    }
    
    /// Parse le texte extrait pour obtenir les contraintes
    fn parse_constraints_from_text(text: &str, width: usize, height: usize) -> Result<Constraints, String> {
        // Expression régulière pour trouver les séquences de nombres
        let re = Regex::new(r"(\d+(?:\s+\d+)*)").unwrap();
        
        let mut all_constraints: Vec<Vec<usize>> = Vec::new();
        
        for cap in re.captures_iter(text) {
            if let Some(matched) = cap.get(1) {
                let numbers: Vec<usize> = matched
                    .as_str()
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                
                if !numbers.is_empty() {
                    all_constraints.push(numbers);
                }
            }
        }
        
        // Séparer les contraintes de lignes et de colonnes
        // Heuristique: les premières contraintes sont pour les colonnes, les suivantes pour les lignes
        if all_constraints.len() < width + height {
            return Err(format!(
                "Pas assez de contraintes trouvées: {} au lieu de {}",
                all_constraints.len(),
                width + height
            ));
        }
        
        let columns = all_constraints[..width].to_vec();
        let rows = all_constraints[width..width + height].to_vec();
        
        Constraints::new(width, height, rows, columns)
    }
    
    /// Extrait les contraintes depuis une région spécifique de l'image
    pub fn extract_from_regions(
        image: &DynamicImage,
        row_region: (u32, u32, u32, u32),  // (x, y, width, height)
        col_region: (u32, u32, u32, u32),
        grid_width: usize,
        grid_height: usize,
    ) -> Result<Constraints, String> {
        #[cfg(feature = "ocr")]
        {
            // Extraire les régions
            let row_img = image.crop_imm(row_region.0, row_region.1, row_region.2, row_region.3);
            let col_img = image.crop_imm(col_region.0, col_region.1, col_region.2, col_region.3);
            
            // Extraire le texte de chaque région
            let row_text = Self::extract_text_from_image(&row_img)?;
            let col_text = Self::extract_text_from_image(&col_img)?;
            
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
    
    #[cfg(feature = "ocr")]
    fn extract_text_from_image(image: &DynamicImage) -> Result<String, String> {
        let gray = image.to_luma8();
        
        let mut tess = Tesseract::new(None, Some("eng"))
            .map_err(|e| format!("Erreur d'initialisation de Tesseract: {}", e))?;
        
        tess.set_variable("tessedit_char_whitelist", "0123456789 \n")
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
        
        if constraints.len() != expected_count {
            eprintln!(
                "Avertissement: {} contraintes trouvées au lieu de {}",
                constraints.len(),
                expected_count
            );
        }
        
        Ok(constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_constraints_from_text() {
        let text = "1 2\n3\n4 5\n6\n7\n8";
        let result = ConstraintExtractor::parse_constraints_from_text(text, 3, 3);
        
        assert!(result.is_ok());
        let constraints = result.unwrap();
        assert_eq!(constraints.width, 3);
        assert_eq!(constraints.height, 3);
    }
}
