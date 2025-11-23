use image::{DynamicImage, GrayImage};
use imageproc::edges::canny;

/// Détecteur de grille avancé pour l'analyse automatique
pub struct GridDetector;

impl GridDetector {
    /// Détecte automatiquement les paramètres de la grille dans une image
    pub fn detect_grid_params(image: &DynamicImage, expected_width: usize, expected_height: usize) -> Result<(u32, u32, u32), String> {
        // Convertir en niveaux de gris
        let gray = image.to_luma8();
        
        // Détecter les lignes de la grille
        let (horizontal_lines, vertical_lines) = Self::detect_grid_lines(&gray)?;
        
        if horizontal_lines.len() < 2 || vertical_lines.len() < 2 {
            return Err("Impossible de détecter suffisamment de lignes de grille".to_string());
        }
        
        // Calculer la taille moyenne des cases
        let cell_width = Self::calculate_average_spacing(&vertical_lines);
        let cell_height = Self::calculate_average_spacing(&horizontal_lines);
        let cell_size = ((cell_width + cell_height) / 2.0) as u32;
        
        // Calculer les marges (position de la première ligne)
        let margin_left = vertical_lines.first().copied().unwrap_or(0) as u32;
        let margin_top = horizontal_lines.first().copied().unwrap_or(0) as u32;
        
        // Vérifier la cohérence avec les dimensions attendues
        let detected_cols = vertical_lines.len().saturating_sub(1);
        let detected_rows = horizontal_lines.len().saturating_sub(1);
        
        if detected_cols != expected_width || detected_rows != expected_height {
            eprintln!("Avertissement: Dimensions détectées ({}x{}) différentes des dimensions attendues ({}x{})",
                     detected_cols, detected_rows, expected_width, expected_height);
        }
        
        Ok((cell_size, margin_left, margin_top))
    }
    
    /// Détecte les lignes horizontales et verticales de la grille
    fn detect_grid_lines(gray: &GrayImage) -> Result<(Vec<usize>, Vec<usize>), String> {
        // Appliquer la détection de contours Canny
        let edges = canny(gray, 50.0, 100.0);
        
        // Détecter les lignes horizontales
        let horizontal_lines = Self::find_horizontal_lines(&edges);
        
        // Détecter les lignes verticales
        let vertical_lines = Self::find_vertical_lines(&edges);
        
        Ok((horizontal_lines, vertical_lines))
    }
    
    /// Trouve les lignes horizontales dans une image de contours
    fn find_horizontal_lines(edges: &GrayImage) -> Vec<usize> {
        let (width, height) = edges.dimensions();
        let mut line_scores: Vec<(usize, u32)> = Vec::new();
        
        // Analyser chaque ligne
        for y in 0..height as usize {
            let mut score = 0u32;
            for x in 0..width as usize {
                if edges.get_pixel(x as u32, y as u32)[0] > 128 {
                    score += 1;
                }
            }
            
            // Si la ligne a suffisamment de pixels blancs, c'est probablement une ligne de grille
            if score > width / 3 {
                line_scores.push((y, score));
            }
        }
        
        // Filtrer les lignes proches (garder celle avec le meilleur score)
        Self::filter_close_lines(line_scores, 5)
    }
    
    /// Trouve les lignes verticales dans une image de contours
    fn find_vertical_lines(edges: &GrayImage) -> Vec<usize> {
        let (width, height) = edges.dimensions();
        let mut line_scores: Vec<(usize, u32)> = Vec::new();
        
        // Analyser chaque colonne
        for x in 0..width as usize {
            let mut score = 0u32;
            for y in 0..height as usize {
                if edges.get_pixel(x as u32, y as u32)[0] > 128 {
                    score += 1;
                }
            }
            
            // Si la colonne a suffisamment de pixels blancs, c'est probablement une ligne de grille
            if score > height / 3 {
                line_scores.push((x, score));
            }
        }
        
        // Filtrer les lignes proches
        Self::filter_close_lines(line_scores, 5)
    }
    
    /// Filtre les lignes trop proches les unes des autres
    fn filter_close_lines(mut lines: Vec<(usize, u32)>, min_distance: usize) -> Vec<usize> {
        if lines.is_empty() {
            return Vec::new();
        }
        
        // Trier par position
        lines.sort_by_key(|&(pos, _)| pos);
        
        let mut filtered = Vec::new();
        let mut last_pos = lines[0].0;
        let mut best_score = lines[0].1;
        let mut best_pos = lines[0].0;
        
        for &(pos, score) in &lines {
            if pos - last_pos < min_distance {
                // Lignes proches, garder celle avec le meilleur score
                if score > best_score {
                    best_score = score;
                    best_pos = pos;
                }
            } else {
                // Nouvelle ligne, sauvegarder la précédente
                filtered.push(best_pos);
                last_pos = pos;
                best_score = score;
                best_pos = pos;
            }
        }
        
        // Ajouter la dernière ligne
        filtered.push(best_pos);
        
        filtered
    }
    
    /// Calcule l'espacement moyen entre les lignes
    fn calculate_average_spacing(lines: &[usize]) -> f32 {
        if lines.len() < 2 {
            return 0.0;
        }
        
        let mut spacings = Vec::new();
        for i in 1..lines.len() {
            spacings.push((lines[i] - lines[i - 1]) as f32);
        }
        
        // Calculer la médiane pour être robuste aux valeurs aberrantes
        spacings.sort_by(|a, b| a.partial_cmp(b).unwrap());
        spacings[spacings.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_filter_close_lines() {
        let lines = vec![(10, 100), (12, 90), (30, 110), (50, 95)];
        let filtered = GridDetector::filter_close_lines(lines, 5);
        
        // Les lignes 10 et 12 sont proches, on garde celle avec le meilleur score (10)
        assert_eq!(filtered.len(), 3);
        assert_eq!(filtered[0], 10);
        assert_eq!(filtered[1], 30);
        assert_eq!(filtered[2], 50);
    }
    
    #[test]
    fn test_calculate_average_spacing() {
        let lines = vec![10, 30, 50, 70];
        let spacing = GridDetector::calculate_average_spacing(&lines);
        assert_eq!(spacing, 20.0);
    }
}
