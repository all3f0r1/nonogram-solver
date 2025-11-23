use image::{GrayImage, Luma, ImageBuffer};

/// Module de détection de contours pur Rust (remplace imageproc::edges::canny)
/// Implémente l'algorithme de détection de contours de Canny

/// Détection de contours Canny simplifiée
/// Retourne une image binaire avec les contours détectés
pub fn canny(image: &GrayImage, low_threshold: f32, high_threshold: f32) -> GrayImage {
    let (width, height) = image.dimensions();
    
    // Étape 1: Flou gaussien (simplifié avec moyenne 3x3)
    let blurred = gaussian_blur(image);
    
    // Étape 2: Calcul du gradient (Sobel)
    let (magnitude, direction) = sobel_gradients(&blurred);
    
    // Étape 3: Suppression des non-maxima
    let suppressed = non_maximum_suppression(&magnitude, &direction);
    
    // Étape 4: Seuillage par hystérésis
    hysteresis_thresholding(&suppressed, low_threshold, high_threshold)
}

/// Flou gaussien simplifié (moyenne 3x3)
fn gaussian_blur(image: &GrayImage) -> GrayImage {
    let (width, height) = image.dimensions();
    let mut blurred = ImageBuffer::new(width, height);
    
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut sum = 0u32;
            
            // Moyenne 3x3
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = image.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                    sum += px[0] as u32;
                }
            }
            
            blurred.put_pixel(x, y, Luma([(sum / 9) as u8]));
        }
    }
    
    blurred
}

/// Calcul des gradients avec l'opérateur de Sobel
fn sobel_gradients(image: &GrayImage) -> (Vec<Vec<f32>>, Vec<Vec<f32>>) {
    let (width, height) = image.dimensions();
    let mut magnitude = vec![vec![0.0; width as usize]; height as usize];
    let mut direction = vec![vec![0.0; width as usize]; height as usize];
    
    // Noyaux de Sobel
    let sobel_x = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let sobel_y = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let mut gx = 0.0;
            let mut gy = 0.0;
            
            // Appliquer les noyaux de Sobel
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = image.get_pixel((x as i32 + dx) as u32, (y as i32 + dy) as u32);
                    let val = px[0] as f32;
                    
                    gx += val * sobel_x[(dy + 1) as usize][(dx + 1) as usize] as f32;
                    gy += val * sobel_y[(dy + 1) as usize][(dx + 1) as usize] as f32;
                }
            }
            
            magnitude[y as usize][x as usize] = (gx * gx + gy * gy).sqrt();
            direction[y as usize][x as usize] = gy.atan2(gx);
        }
    }
    
    (magnitude, direction)
}

/// Suppression des non-maxima
fn non_maximum_suppression(magnitude: &[Vec<f32>], direction: &[Vec<f32>]) -> Vec<Vec<f32>> {
    let height = magnitude.len();
    let width = magnitude[0].len();
    let mut suppressed = vec![vec![0.0; width]; height];
    
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let angle = direction[y][x];
            let mag = magnitude[y][x];
            
            // Convertir l'angle en direction (0, 45, 90, 135 degrés)
            let angle_deg = angle.to_degrees();
            let angle_norm = ((angle_deg + 180.0) % 180.0) as i32;
            
            let (dx, dy) = match angle_norm {
                0..=22 | 158..=180 => (1, 0),      // Horizontal
                23..=67 => (1, 1),                  // Diagonal /
                68..=112 => (0, 1),                 // Vertical
                113..=157 => (-1, 1),               // Diagonal \
                _ => (0, 0),
            };
            
            let mag1 = magnitude[(y as i32 + dy) as usize][(x as i32 + dx) as usize];
            let mag2 = magnitude[(y as i32 - dy) as usize][(x as i32 - dx) as usize];
            
            // Garder seulement si c'est un maximum local
            if mag >= mag1 && mag >= mag2 {
                suppressed[y][x] = mag;
            }
        }
    }
    
    suppressed
}

/// Seuillage par hystérésis
fn hysteresis_thresholding(suppressed: &[Vec<f32>], low: f32, high: f32) -> GrayImage {
    let height = suppressed.len();
    let width = suppressed[0].len();
    let mut result = ImageBuffer::new(width as u32, height as u32);
    
    // Marquer les pixels forts et faibles
    let mut strong = vec![vec![false; width]; height];
    let mut weak = vec![vec![false; width]; height];
    
    for y in 0..height {
        for x in 0..width {
            let mag = suppressed[y][x];
            
            if mag >= high {
                strong[y][x] = true;
                result.put_pixel(x as u32, y as u32, Luma([255]));
            } else if mag >= low {
                weak[y][x] = true;
            }
        }
    }
    
    // Connecter les pixels faibles aux pixels forts
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            if weak[y][x] {
                // Vérifier si connecté à un pixel fort
                let mut connected = false;
                
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if strong[(y as i32 + dy) as usize][(x as i32 + dx) as usize] {
                            connected = true;
                            break;
                        }
                    }
                    if connected {
                        break;
                    }
                }
                
                if connected {
                    result.put_pixel(x as u32, y as u32, Luma([255]));
                }
            }
        }
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Luma;
    
    #[test]
    fn test_gaussian_blur() {
        let mut img = GrayImage::new(10, 10);
        
        // Mettre un pixel blanc au centre
        img.put_pixel(5, 5, Luma([255]));
        
        let blurred = gaussian_blur(&img);
        
        // Le pixel central devrait être moins intense après le flou
        assert!(blurred.get_pixel(5, 5)[0] < 255);
    }
    
    #[test]
    fn test_canny() {
        let mut img = GrayImage::new(100, 100);
        
        // Créer une ligne verticale
        for y in 0..100 {
            img.put_pixel(50, y, Luma([255]));
        }
        
        let edges = canny(&img, 50.0, 100.0);
        
        // Il devrait y avoir des contours détectés
        let mut has_edges = false;
        for y in 0..100 {
            for x in 0..100 {
                if edges.get_pixel(x, y)[0] > 0 {
                    has_edges = true;
                    break;
                }
            }
        }
        
        assert!(has_edges);
    }
}
