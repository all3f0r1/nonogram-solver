#!/usr/bin/env python3
"""
Script pour générer une image de nonogramme de test
"""

from PIL import Image, ImageDraw, ImageFont
import json

def generate_nonogram_image(constraints_file, output_file, cell_size=40, margin=100):
    """
    Génère une image de nonogramme vide à partir d'un fichier de contraintes
    """
    # Charger les contraintes
    with open(constraints_file, 'r') as f:
        constraints = json.load(f)
    
    width = constraints['width']
    height = constraints['height']
    rows = constraints['rows']
    columns = constraints['columns']
    
    # Calculer les dimensions de l'image
    img_width = margin + width * cell_size + 20
    img_height = margin + height * cell_size + 20
    
    # Créer l'image
    img = Image.new('RGB', (img_width, img_height), color='white')
    draw = ImageDraw.Draw(img)
    
    # Dessiner la grille
    for i in range(height + 1):
        y = margin + i * cell_size
        draw.line([(margin, y), (margin + width * cell_size, y)], fill='black', width=2)
    
    for i in range(width + 1):
        x = margin + i * cell_size
        draw.line([(x, margin), (x, margin + height * cell_size)], fill='black', width=2)
    
    # Dessiner les contraintes des lignes (à gauche)
    try:
        font = ImageFont.truetype("/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf", 16)
    except:
        font = ImageFont.load_default()
    
    for i, row_constraint in enumerate(rows):
        text = ' '.join(map(str, row_constraint))
        y = margin + i * cell_size + cell_size // 2
        # Centrer le texte verticalement
        bbox = draw.textbbox((0, 0), text, font=font)
        text_height = bbox[3] - bbox[1]
        draw.text((margin - 60, y - text_height // 2), text, fill='black', font=font)
    
    # Dessiner les contraintes des colonnes (en haut)
    for i, col_constraint in enumerate(columns):
        text = '\n'.join(map(str, col_constraint))
        x = margin + i * cell_size + cell_size // 2
        # Centrer le texte horizontalement
        lines = text.split('\n')
        y_offset = margin - 70
        for line in lines:
            bbox = draw.textbbox((0, 0), line, font=font)
            text_width = bbox[2] - bbox[0]
            draw.text((x - text_width // 2, y_offset), line, fill='black', font=font)
            y_offset += 20
    
    # Sauvegarder l'image
    img.save(output_file)
    print(f"Image générée: {output_file}")
    print(f"Dimensions: {img_width}x{img_height} pixels")
    print(f"Grille: {width}x{height}")
    print(f"Taille de case: {cell_size} pixels")
    print(f"Marge: {margin} pixels")

def add_partial_solution(input_image, output_image, filled_cells, crossed_cells, cell_size=40, margin=100):
    """
    Ajoute une solution partielle à une image de nonogramme
    filled_cells: liste de tuples (row, col) pour les cases noires
    crossed_cells: liste de tuples (row, col) pour les cases barrées
    """
    img = Image.open(input_image)
    draw = ImageDraw.Draw(img)
    
    # Dessiner les cases noires
    for row, col in filled_cells:
        x1 = margin + col * cell_size + 3
        y1 = margin + row * cell_size + 3
        x2 = margin + (col + 1) * cell_size - 3
        y2 = margin + (row + 1) * cell_size - 3
        draw.rectangle([x1, y1, x2, y2], fill='black')
    
    # Dessiner les croix pour les cases barrées
    for row, col in crossed_cells:
        x1 = margin + col * cell_size + 5
        y1 = margin + row * cell_size + 5
        x2 = margin + (col + 1) * cell_size - 5
        y2 = margin + (row + 1) * cell_size - 5
        draw.line([x1, y1, x2, y2], fill='blue', width=2)
        draw.line([x1, y2, x2, y1], fill='blue', width=2)
    
    img.save(output_image)
    print(f"Image avec solution partielle générée: {output_image}")

if __name__ == "__main__":
    # Générer l'image vide pour l'exemple 5x5
    generate_nonogram_image(
        'simple_5x5.json',
        'simple_5x5_empty.png',
        cell_size=40,
        margin=100
    )
    
    # Générer une version avec quelques cases remplies
    # Pour le motif en croix, on remplit quelques cases stratégiques
    filled_cells = []
    crossed_cells = []
    
    add_partial_solution(
        'simple_5x5_empty.png',
        'simple_5x5_partial.png',
        filled_cells,
        crossed_cells,
        cell_size=40,
        margin=100
    )
    
    print("\nPour tester l'application:")
    print("./target/release/nonogram-solver \\")
    print("  --input examples/simple_5x5_empty.png \\")
    print("  --constraints examples/simple_5x5.json \\")
    print("  --output examples/simple_5x5_solution.png \\")
    print("  --cell-size 40 \\")
    print("  --margin-left 100 \\")
    print("  --margin-top 100 \\")
    print("  --verbose")
