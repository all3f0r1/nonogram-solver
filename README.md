# Nonogram Solver (Picross/Griddlers Solver)

[![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A Rust application that solves nonogram puzzles (picross/griddlers/hanjie/logimage) through logical deduction. Analyzes an image, automatically extracts constraints, and outputs the solution in ASCII/JSON/Array format or generates a marked image.

## 🎉 What's New in v1.0.0

### ✅ Simplified Workflow: Image → Solution

- **Automatic constraint extraction** from filled cells (no OCR needed!)
- **Multi-format export**: ASCII (console), JSON, Array2D
- **Interactive mode** for manual constraint entry
- **Simplified command**: `nonogram-solver image.png` just works
- **Console output** by default with solved grid
- **100% pure Rust**: No non-Rust dependencies for main features

## 🎯 Features

- **✨ Automatic extraction**: Analyzes filled cells to derive constraints
- **🖼️ Multi-format support**: JPEG, PNG, BMP, GIF, TIFF, WebP
- **📤 Multi-format export**: ASCII (█/✕/·), JSON structured, Array2D (1/0/-1)
- **🎮 Interactive mode**: Manual constraint entry in terminal
- **🧩 Pure logical deduction**: Solving algorithms without guessing
- **🎨 Visual marking** (optional): Generate image with deductions marked in red
- **🏆 Three solvers**: Basic, Advanced, Ultimate (backtracking + parallelization)

## 📋 Prerequisites

- Rust 1.91 or higher
- C compiler (gcc/clang) for native dependencies

## 🚀 Installation

```bash
# Clone the project
git clone https://github.com/your-username/nonogram-solver.git
cd nonogram-solver

# Build in release mode
cargo build --release

# Binary will be in target/release/nonogram-solver
```

## 📖 Usage

### Simplest usage

```bash
# Automatic analysis + console ASCII output
./target/release/nonogram-solver image.png
```

### Export the solution

```bash
# Export as JSON
./target/release/nonogram-solver -i image.png --export solution.json

# Export as 2D array
./target/release/nonogram-solver -i image.png --export solution.txt

# Export as ASCII explicitly
./target/release/nonogram-solver -i image.png --export solution.txt --export-format ascii
```

### Constraint extraction modes

```bash
# Automatic extraction from filled cells (default)
./target/release/nonogram-solver -i image.png --extract-filled

# Interactive mode (manual entry)
./target/release/nonogram-solver -i image.png --interactive

# With JSON file (classic method)
./target/release/nonogram-solver -i image.png -c constraints.json
```

### Main options

| Option | Description |
|--------|-------------|
| `-i, --input <FILE>` | Input nonogram image |
| `-o, --output <FILE>` | Output image (optional) |
| `-e, --export <FILE>` | Export grid to file |
| `--export-format <FMT>` | Format: ascii, json, array |
| `--extract-filled` | Auto-extract from filled cells |
| `--interactive` | Interactive constraint input |
| `--extract-only` | Extract and display without solving |
| `--advanced` | Advanced solver |
| `--ultimate` | Ultimate solver (backtracking + parallel) |
| `-v, --verbose` | Verbose mode |

### Complete examples

```bash
# Auto extraction + ultimate solver + JSON export
./target/release/nonogram-solver \
  -i puzzle.png \
  --extract-filled \
  --ultimate \
  --export solution.json \
  --verbose

# Interactive mode with output image
./target/release/nonogram-solver \
  -i puzzle.png \
  --interactive \
  -o solution.png

# Extract only (no solving)
./target/release/nonogram-solver \
  -i puzzle.png \
  --extract-only \
  --export grid.txt
```

## 📤 Export Formats

### ASCII (console)
```
✕ ✕ █ █ █
✕ ✕ █ █ █
█ █ █ █ █
```

### JSON
```json
{
  "width": 5,
  "height": 3,
  "cells": [
    ["crossed", "crossed", "filled", "filled", "filled"],
    ...
  ]
}
```

### Array2D
```
[
  [-1, -1, 1, 1, 1],
  [-1, -1, 1, 1, 1],
  [1, 1, 1, 1, 1]
]
```

## 🏗️ Architecture

### Core modules

- **`grid/`**: Grid representation (`CellState`, `Grid`, `Constraints`)
- **`solver/`**: Solving algorithms (`NonogramSolver`, `AdvancedSolver`, `UltimateSolver`)
- **`image_parser/`**: Image analysis and grid detection
- **`image_generator/`**: Image generation with markings
- **`grid_output/`**: Multi-format export (ASCII, JSON, Array2D)
- **`interactive/`**: Interactive constraint input
- **`ocr`** (optional): Tesseract-based constraint extraction

### Available solvers

1. **Basic**: Line-by-line deduction with cache
2. **Advanced**: Cross-analysis + advanced heuristics
3. **Ultimate**: All techniques + backtracking + parallelization

## 🔧 Development

```bash
# Build
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Run linter
cargo clippy

# Release build
cargo build --release
```

## 📊 Performance

- 5x5 grid: < 0.1 seconds
- 10x10 grid: < 0.5 seconds
- 20x20 grid: < 3 seconds
- 30x30 grid: < 10 seconds

## 📝 Supported image formats

JPEG, PNG, BMP, GIF, TIFF, WebP

## 📄 License

MIT

---

**Built with ❤️ in Rust**
