use image::{Rgb, RgbImage};

/// Module de dessin pur Rust (remplace imageproc)
/// Implémente les algorithmes de dessin nécessaires sans dépendances externes

/// Dessine un cercle rempli sur une image
/// Utilise l'algorithme de Bresenham pour les cercles
pub fn draw_filled_circle_mut(image: &mut RgbImage, center: (i32, i32), radius: i32, color: Rgb<u8>) {
    let (cx, cy) = center;
    
    // Dessiner le cercle rempli en utilisant l'algorithme de scan-line
    for y in (cy - radius)..=(cy + radius) {
        if y < 0 || y >= image.height() as i32 {
            continue;
        }
        
        let dy = y - cy;
        let dx = ((radius * radius - dy * dy) as f32).sqrt() as i32;
        
        for x in (cx - dx)..=(cx + dx) {
            if x >= 0 && x < image.width() as i32 {
                image.put_pixel(x as u32, y as u32, color);
            }
        }
    }
}

/// Dessine une croix sur une image
/// Dessine deux lignes diagonales formant une croix
pub fn draw_cross_mut(image: &mut RgbImage, center: (i32, i32), size: u32, color: Rgb<u8>) {
    let (cx, cy) = center;
    let half_size = (size / 2) as i32;
    
    // Ligne diagonale de haut-gauche à bas-droite
    draw_line_mut(
        image,
        (cx - half_size, cy - half_size),
        (cx + half_size, cy + half_size),
        color
    );
    
    // Ligne diagonale de haut-droite à bas-gauche
    draw_line_mut(
        image,
        (cx + half_size, cy - half_size),
        (cx - half_size, cy + half_size),
        color
    );
}

/// Dessine une ligne entre deux points
/// Utilise l'algorithme de Bresenham pour les lignes
pub fn draw_line_mut(image: &mut RgbImage, start: (i32, i32), end: (i32, i32), color: Rgb<u8>) {
    let (mut x0, mut y0) = start;
    let (x1, y1) = end;
    
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    
    let mut err = dx - dy;
    
    loop {
        // Dessiner le pixel si dans les limites
        if x0 >= 0 && x0 < image.width() as i32 && y0 >= 0 && y0 < image.height() as i32 {
            image.put_pixel(x0 as u32, y0 as u32, color);
        }
        
        if x0 == x1 && y0 == y1 {
            break;
        }
        
        let e2 = 2 * err;
        
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgb;
    
    #[test]
    fn test_draw_filled_circle() {
        let mut img = RgbImage::new(100, 100);
        let white = Rgb([255, 255, 255]);
        
        draw_filled_circle_mut(&mut img, (50, 50), 20, white);
        
        // Vérifier que le centre est blanc
        assert_eq!(img.get_pixel(50, 50), &white);
        
        // Vérifier qu'un point sur le bord est blanc
        assert_eq!(img.get_pixel(70, 50), &white);
    }
    
    #[test]
    fn test_draw_cross() {
        let mut img = RgbImage::new(100, 100);
        let white = Rgb([255, 255, 255]);
        
        draw_cross_mut(&mut img, (50, 50), 20, white);
        
        // Vérifier que le centre est blanc
        assert_eq!(img.get_pixel(50, 50), &white);
    }
    
    #[test]
    fn test_draw_line() {
        let mut img = RgbImage::new(100, 100);
        let white = Rgb([255, 255, 255]);
        
        draw_line_mut(&mut img, (10, 10), (90, 90), white);
        
        // Vérifier que les points de départ et d'arrivée sont blancs
        assert_eq!(img.get_pixel(10, 10), &white);
        assert_eq!(img.get_pixel(90, 90), &white);
    }
}
