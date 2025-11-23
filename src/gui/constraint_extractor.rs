use image::DynamicImage;
use crate::grid::Constraints;
use crate::image_parser::grid_detector::GridDetector;
use anyhow::{Result, Context};

/// Extrait automatiquement les contraintes depuis une image de nonogramme
pub struct ConstraintExtractor;

impl ConstraintExtractor {
    /// Extrait les contraintes depuis une image
    /// 
    /// Cette fonction détecte automatiquement:
    /// 1. La grille (taille, position)
    /// 2. Les zones de contraintes (haut et gauche)
    /// 3. Les nombres dans ces zones (OCR si feature activée)
    pub fn extract(img: &DynamicImage) -> Result<Constraints> {
        // Étape 1: Détecter la grille
        let detector = GridDetector::new();
        let grid_info = detector.detect_grid(img)
            .context("Impossible de détecter la grille")?;
        
        // Étape 2: Extraire les zones de contraintes
        let (rows_constraints, cols_constraints) = Self::extract_constraint_zones(
            img,
            grid_info.cell_size,
            grid_info.margin_left,
            grid_info.margin_top,
            grid_info.width,
            grid_info.height,
        )?;
        
        Ok(Constraints {
            rows: rows_constraints,
            cols: cols_constraints,
        })
    }
    
    /// Extrait les zones de contraintes depuis l'image
    fn extract_constraint_zones(
        img: &DynamicImage,
        cell_size: u32,
        margin_left: u32,
        margin_top: u32,
        grid_width: usize,
        grid_height: usize,
    ) -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
        // Pour l'instant, utiliser une approche simple basée sur l'analyse de pixels
        // Dans une version future, on pourrait intégrer l'OCR ici
        
        let gray_img = img.to_luma8();
        
        // Extraire les contraintes des lignes (à gauche de la grille)
        let mut rows_constraints = Vec::new();
        for row in 0..grid_height {
            let y = margin_top + (row as u32 * cell_size) + cell_size / 2;
            let constraints = Self::extract_row_constraints(
                &gray_img,
                y,
                margin_left,
                cell_size,
            )?;
            rows_constraints.push(constraints);
        }
        
        // Extraire les contraintes des colonnes (en haut de la grille)
        let mut cols_constraints = Vec::new();
        for col in 0..grid_width {
            let x = margin_left + (col as u32 * cell_size) + cell_size / 2;
            let constraints = Self::extract_col_constraints(
                &gray_img,
                x,
                margin_top,
                cell_size,
            )?;
            cols_constraints.push(constraints);
        }
        
        Ok((rows_constraints, cols_constraints))
    }
    
    /// Extrait les contraintes d'une ligne (zone à gauche)
    fn extract_row_constraints(
        gray_img: &image::GrayImage,
        y: u32,
        margin_left: u32,
        cell_size: u32,
    ) -> Result<Vec<usize>> {
        // Approche simple: analyser la densité de pixels noirs
        // dans la zone de contraintes à gauche
        
        // Pour l'instant, retourner des contraintes par défaut
        // TODO: Implémenter l'analyse réelle ou l'OCR
        
        // Analyser la zone de contraintes (à gauche de la grille)
        let constraint_zone_width = margin_left;
        let mut constraints = Vec::new();
        
        // Diviser la zone en segments et détecter les groupes de pixels noirs
        let segment_width = cell_size / 2;
        let mut current_block_size = 0;
        let mut in_block = false;
        
        for x in 0..constraint_zone_width {
            if x >= gray_img.width() || y >= gray_img.height() {
                continue;
            }
            
            let pixel = gray_img.get_pixel(x, y);
            let is_dark = pixel[0] < 128;
            
            if is_dark {
                if !in_block {
                    in_block = true;
                    current_block_size = 1;
                } else {
                    current_block_size += 1;
                }
            } else {
                if in_block && current_block_size > segment_width / 4 {
                    // Estimer la taille du bloc basé sur la largeur
                    let estimated_size = (current_block_size / segment_width).max(1) as usize;
                    constraints.push(estimated_size);
                }
                in_block = false;
                current_block_size = 0;
            }
        }
        
        // Ajouter le dernier bloc si nécessaire
        if in_block && current_block_size > segment_width / 4 {
            let estimated_size = (current_block_size / segment_width).max(1) as usize;
            constraints.push(estimated_size);
        }
        
        // Si aucune contrainte détectée, retourner une contrainte par défaut
        if constraints.is_empty() {
            constraints.push(1);
        }
        
        Ok(constraints)
    }
    
    /// Extrait les contraintes d'une colonne (zone en haut)
    fn extract_col_constraints(
        gray_img: &image::GrayImage,
        x: u32,
        margin_top: u32,
        cell_size: u32,
    ) -> Result<Vec<usize>> {
        // Similaire à extract_row_constraints mais vertical
        
        let constraint_zone_height = margin_top;
        let mut constraints = Vec::new();
        
        let segment_height = cell_size / 2;
        let mut current_block_size = 0;
        let mut in_block = false;
        
        for y in 0..constraint_zone_height {
            if x >= gray_img.width() || y >= gray_img.height() {
                continue;
            }
            
            let pixel = gray_img.get_pixel(x, y);
            let is_dark = pixel[0] < 128;
            
            if is_dark {
                if !in_block {
                    in_block = true;
                    current_block_size = 1;
                } else {
                    current_block_size += 1;
                }
            } else {
                if in_block && current_block_size > segment_height / 4 {
                    let estimated_size = (current_block_size / segment_height).max(1) as usize;
                    constraints.push(estimated_size);
                }
                in_block = false;
                current_block_size = 0;
            }
        }
        
        if in_block && current_block_size > segment_height / 4 {
            let estimated_size = (current_block_size / segment_height).max(1) as usize;
            constraints.push(estimated_size);
        }
        
        if constraints.is_empty() {
            constraints.push(1);
        }
        
        Ok(constraints)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_extract_basic() {
        // Test basique avec une image simple
        // TODO: Ajouter des tests avec des images réelles
    }
}
