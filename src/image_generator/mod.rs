use image::{DynamicImage, Rgba, RgbaImage, Rgb};
use crate::drawing::{draw_filled_circle_mut, draw_cross_mut};
use crate::solver::Deduction;
use crate::grid::CellState;

/// Configuration pour le générateur d'image
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Taille d'une case en pixels
    pub cell_size: u32,
    /// Marge depuis le bord de l'image jusqu'à la grille
    pub margin_top: u32,
    pub margin_left: u32,
    /// Couleur pour marquer les déductions (rouge par défaut)
    pub highlight_color: Rgba<u8>,
    /// Rayon du cercle de marquage (en proportion de la taille de case)
    pub marker_radius_ratio: f32,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            cell_size: 20,
            margin_top: 50,
            margin_left: 50,
            highlight_color: Rgba([255, 0, 0, 180]), // Rouge semi-transparent
            marker_radius_ratio: 0.3,
        }
    }
}

/// Générateur d'image pour marquer les déductions
pub struct ImageGenerator {
    config: GeneratorConfig,
}

impl ImageGenerator {
    /// Crée un nouveau générateur avec la configuration donnée
    pub fn new(config: GeneratorConfig) -> Self {
        Self { config }
    }

    /// Crée un nouveau générateur avec la configuration par défaut
    pub fn with_default_config() -> Self {
        Self::new(GeneratorConfig::default())
    }

    /// Génère une image de sortie avec les déductions marquées en rouge
    pub fn generate_output_image(
        &self,
        input_image: &DynamicImage,
        deductions: &[Deduction],
    ) -> Result<DynamicImage, String> {
        // Créer une copie de l'image d'entrée
        let mut output = input_image.to_rgba8();

        // Marquer chaque déduction
        for deduction in deductions {
            self.mark_deduction(&mut output, deduction)?;
        }

        Ok(DynamicImage::ImageRgba8(output))
    }

    /// Marque une déduction sur l'image
    fn mark_deduction(&self, image: &mut RgbaImage, deduction: &Deduction) -> Result<(), String> {
        // Calculer le centre de la case
        let center_x = self.config.margin_left + (deduction.col as u32 * self.config.cell_size) + (self.config.cell_size / 2);
        let center_y = self.config.margin_top + (deduction.row as u32 * self.config.cell_size) + (self.config.cell_size / 2);

        // Vérifier que le point est dans l'image
        if center_x >= image.width() || center_y >= image.height() {
            return Err(format!(
                "Position ({}, {}) hors de l'image ({}x{})",
                center_x, center_y, image.width(), image.height()
            ));
        }

        // Calculer le rayon du marqueur
        let radius = (self.config.cell_size as f32 * self.config.marker_radius_ratio) as i32;

        // Dessiner le marqueur selon le type de déduction
        match deduction.state {
            CellState::Filled => {
                // Cercle rouge plein pour une case noire déduite
                self.draw_filled_circle_rgba(image, center_x as i32, center_y as i32, radius)?;
            }
            CellState::Crossed => {
                // Croix rouge pour une case barrée déduite
                self.draw_cross_rgba(image, center_x as i32, center_y as i32, radius)?;
            }
            CellState::Empty => {
                // Ne devrait pas arriver, mais on ne fait rien
            }
        }

        Ok(())
    }

    /// Dessine un cercle rempli sur une image RGBA
    fn draw_filled_circle_rgba(&self, image: &mut RgbaImage, center_x: i32, center_y: i32, radius: i32) -> Result<(), String> {
        // Créer une image RGB temporaire
        let mut rgb_image = DynamicImage::ImageRgba8(image.clone()).to_rgb8();
        let rgb_color = Rgb([self.config.highlight_color[0], self.config.highlight_color[1], self.config.highlight_color[2]]);
        
        // Dessiner le cercle sur l'image RGB
        draw_filled_circle_mut(&mut rgb_image, (center_x, center_y), radius, rgb_color);
        
        // Copier le résultat avec alpha
        for y in (center_y - radius).max(0)..(center_y + radius + 1).min(image.height() as i32) {
            for x in (center_x - radius).max(0)..(center_x + radius + 1).min(image.width() as i32) {
                let rgb_pixel = rgb_image.get_pixel(x as u32, y as u32);
                let original_pixel = image.get_pixel(x as u32, y as u32);
                
                // Si le pixel a été modifié par le cercle, appliquer la couleur avec alpha
                if rgb_pixel[0] == rgb_color[0] && rgb_pixel[1] == rgb_color[1] && rgb_pixel[2] == rgb_color[2] {
                    // Blending alpha
                    let alpha = self.config.highlight_color[3] as f32 / 255.0;
                    let blended = Rgba([
                        ((rgb_color[0] as f32 * alpha) + (original_pixel[0] as f32 * (1.0 - alpha))) as u8,
                        ((rgb_color[1] as f32 * alpha) + (original_pixel[1] as f32 * (1.0 - alpha))) as u8,
                        ((rgb_color[2] as f32 * alpha) + (original_pixel[2] as f32 * (1.0 - alpha))) as u8,
                        255,
                    ]);
                    image.put_pixel(x as u32, y as u32, blended);
                }
            }
        }
        
        Ok(())
    }

    /// Dessine une croix sur une image RGBA
    fn draw_cross_rgba(&self, image: &mut RgbaImage, center_x: i32, center_y: i32, size: i32) -> Result<(), String> {
        // Créer une image RGB temporaire
        let mut rgb_image = DynamicImage::ImageRgba8(image.clone()).to_rgb8();
        let rgb_color = Rgb([self.config.highlight_color[0], self.config.highlight_color[1], self.config.highlight_color[2]]);
        
        // Dessiner la croix sur l'image RGB
        draw_cross_mut(&mut rgb_image, (center_x, center_y), (size * 2) as u32, rgb_color);
        
        // Épaissir la croix en dessinant plusieurs fois
        for offset in -1..=1 {
            draw_cross_mut(&mut rgb_image, (center_x + offset, center_y), (size * 2) as u32, rgb_color);
            draw_cross_mut(&mut rgb_image, (center_x, center_y + offset), (size * 2) as u32, rgb_color);
        }
        
        // Copier le résultat avec alpha
        for y in (center_y - size).max(0)..(center_y + size + 1).min(image.height() as i32) {
            for x in (center_x - size).max(0)..(center_x + size + 1).min(image.width() as i32) {
                let rgb_pixel = rgb_image.get_pixel(x as u32, y as u32);
                let original_pixel = image.get_pixel(x as u32, y as u32);
                
                // Si le pixel a été modifié par la croix, appliquer la couleur avec alpha
                if rgb_pixel[0] == rgb_color[0] && rgb_pixel[1] == rgb_color[1] && rgb_pixel[2] == rgb_color[2] {
                    // Blending alpha
                    let alpha = self.config.highlight_color[3] as f32 / 255.0;
                    let blended = Rgba([
                        ((rgb_color[0] as f32 * alpha) + (original_pixel[0] as f32 * (1.0 - alpha))) as u8,
                        ((rgb_color[1] as f32 * alpha) + (original_pixel[1] as f32 * (1.0 - alpha))) as u8,
                        ((rgb_color[2] as f32 * alpha) + (original_pixel[2] as f32 * (1.0 - alpha))) as u8,
                        255,
                    ]);
                    image.put_pixel(x as u32, y as u32, blended);
                }
            }
        }
        
        Ok(())
    }

    /// Sauvegarde l'image dans un fichier
    pub fn save_image(image: &DynamicImage, path: &str) -> Result<(), String> {
        image.save(path).map_err(|e| format!("Erreur lors de la sauvegarde de l'image: {}", e))
    }

    /// Crée une configuration à partir d'une configuration de parseur
    pub fn from_parser_config(
        cell_size: u32,
        margin_top: u32,
        margin_left: u32,
    ) -> GeneratorConfig {
        GeneratorConfig {
            cell_size,
            margin_top,
            margin_left,
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let generator = ImageGenerator::with_default_config();
        assert_eq!(generator.config.cell_size, 20);
    }

    #[test]
    fn test_default_config() {
        let config = GeneratorConfig::default();
        assert_eq!(config.highlight_color[0], 255); // Rouge
        assert_eq!(config.highlight_color[1], 0);
        assert_eq!(config.highlight_color[2], 0);
    }

    #[test]
    fn test_from_parser_config() {
        let config = ImageGenerator::from_parser_config(30, 60, 70);
        assert_eq!(config.cell_size, 30);
        assert_eq!(config.margin_top, 60);
        assert_eq!(config.margin_left, 70);
    }
}
