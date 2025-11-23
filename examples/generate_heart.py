#!/usr/bin/env python3
"""
Script pour générer une image de nonogramme 10x10 (motif coeur)
"""

import sys
sys.path.insert(0, '.')
from generate_test_image import generate_nonogram_image

if __name__ == "__main__":
    generate_nonogram_image(
        'heart_10x10.json',
        'heart_10x10_empty.png',
        cell_size=30,
        margin=80
    )
    
    print("\nPour tester l'application:")
    print("./target/release/nonogram-solver \\")
    print("  --input examples/heart_10x10_empty.png \\")
    print("  --constraints examples/heart_10x10.json \\")
    print("  --output examples/heart_10x10_solution.png \\")
    print("  --cell-size 30 \\")
    print("  --margin-left 80 \\")
    print("  --margin-top 80 \\")
    print("  --verbose")
