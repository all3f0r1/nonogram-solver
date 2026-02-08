# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Nonogram Solver is a Rust application that solves nonogram puzzles (logimage/hanjie) through logical deduction. It can analyze images of nonogram grids and extract constraints automatically, then output the solution in multiple formats (ASCII, JSON, Array2D).

## Key Design Principle

**Simple workflow**: `nonogram-solver image.png` should work - extract constraints from filled cells, solve, and display the solution.

## Build Commands

```bash
# Development build
cargo build

# Release build (most common)
cargo build --release

# With OCR support (optional, requires Tesseract)
cargo build --release --features ocr
```

## Testing and Code Quality

```bash
# Run all tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## Feature Flags

- `default = []` - No features enabled by default
- `ocr` - Enables Tesseract OCR for constraint extraction from images
- `gui` - Enables Slint-based GUI interface

## Binaries

- `nonogram-solver` (default) - CLI interface
- `nonogram-solver-gui` (requires `gui` feature) - GUI interface

## Quick Start

The simplest possible usage:
```bash
./target/release/nonogram-solver image.png
```

This will:
1. Detect grid dimensions
2. Extract constraints from filled cells (pure Rust, no OCR)
3. Solve the puzzle
4. Display the solved grid in ASCII to console

## Architecture

### Core Modules

- **`grid/`** - Grid representation and constraint validation
  - `CellState`: Empty, Filled, Crossed with Display trait
  - `Grid`: Main grid data structure with Display trait
  - `Constraints`: Row/column constraints with validation (returns Result)

- **`solver/`** - Multiple solving strategies
  - `NonogramSolver`: Basic line solving
  - `AdvancedSolver`: Cross-analysis and heuristics
  - `UltimateSolver`: Backtracking + parallelization (Rayon)

- **`image_parser/`** - Image analysis and grid detection
  - `ImageParser`: Extracts grid state from images
  - `GridDetector`: Canny edge detection for auto-detection

- **`image_generator/`** - Output image generation
  - `ImageGenerator`: Creates marked images with red overlays

- **`grid_output/`** (NEW in v1.0) - Multi-format export
  - `GridOutputFormatter`: ASCII, JSON, Array2D formats
  - Helper functions to avoid DRY violations

- **`interactive/`** (NEW in v1.0) - Interactive constraint input
  - `InteractiveInput`: Terminal-based constraint entry

- **`ocr/`** - Optional constraint extraction
  - `AdvancedConstraintExtractor::extract_constraints_from_filled_cells()` - Pure Rust heuristic (NEW)
  - `AdvancedConstraintExtractor::extract_auto()` - OCR-based

### Entry Points

- `src/main.rs` - CLI interface
- `src/gui_main.rs` - GUI interface (requires `gui` feature)

## Constraint Extraction Flow

1. **No explicit constraints specified** → Try `extract_constraints_from_filled_cells()`
2. **`--extract-filled`** → Force heuristic extraction from filled cells
3. **`--interactive`** → Manual terminal input
4. **`--use-ocr`** → OCR extraction (requires feature)
5. **`--constraints FILE`** → Load from JSON

The heuristic extraction counts consecutive filled cells per row/column to derive constraints.

## Output Formats

### ASCII (default console output)
```
✕ ✕ █ █ █
█ █ █ █ █
```

### JSON
```json
{
  "width": 5,
  "height": 2,
  "cells": [["crossed", "crossed", "filled", "filled", "filled"], ...]
}
```

### Array2D
```
[
  [-1, -1, 1, 1, 1],
  [1, 1, 1, 1, 1]
]
```

## Common Patterns

- Error handling: `Result<T, String>` throughout library code, `anyhow::Result` in main
- French comments and user-facing messages
- Grid dimensions: `width` = columns, `height` = rows
- Cell states: `0`/Empty, `1`/Filled, `-1`/Crossed in Array2D format

## Development Notes

- The project is primarily documented in French
- Tests exist in many modules (`cargo test` to run)
- Use `cargo fmt` before committing
- The codebase has some pre-existing clippy warnings (mostly unused code and stylistic suggestions)
