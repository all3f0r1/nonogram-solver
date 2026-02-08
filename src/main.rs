mod drawing;
mod edge_detection;
mod grid;
mod grid_output;
mod image_generator;
mod image_parser;
mod interactive;
mod ocr;
mod solver;

use anyhow::Result;
use clap::Parser;

use grid::Constraints;
use grid_output::{GridOutputFormatter, OutputFormat};
use image_generator::ImageGenerator;
use image_parser::ImageParser;
use interactive::InteractiveInput;
use ocr::AdvancedConstraintExtractor;
use solver::{
    AdvancedSolver, AdvancedSolverConfig, NonogramSolver, UltimateSolver, UltimateSolverConfig,
};

/// Solveur de nonogramme (logimage/hanjie) par déduction logique
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chemin vers l'image d'entrée du nonogramme
    #[arg(short, long)]
    input: String,

    /// Chemin vers le fichier JSON contenant les contraintes (optionnel, utilisez --extract-filled pour extraction auto)
    #[arg(short, long)]
    constraints: Option<String>,

    /// Extraction automatique des contraintes depuis l'image (sans OCR, par analyse des cases remplies)
    #[arg(long)]
    extract_filled: bool,

    /// Utiliser l'OCR pour extraire automatiquement les contraintes de l'image (nécessite --features ocr)
    #[arg(long)]
    use_ocr: bool,

    /// Mode interactif pour saisir les contraintes manuellement
    #[arg(long)]
    interactive: bool,

    /// Chemin vers l'image de sortie avec les déductions marquées (optionnel)
    #[arg(short, long)]
    output: Option<String>,

    /// Chemin vers le fichier d'export de la grille (optionnel, format détecté depuis l'extension)
    #[arg(short = 'e', long)]
    export: Option<String>,

    /// Format d'export de la grille (ascii, json, array). Détection auto depuis l'extension si non spécifié
    #[arg(long)]
    export_format: Option<String>,

    /// Extraire et afficher la grille sans résoudre
    #[arg(long)]
    extract_only: bool,

    /// Taille d'une case en pixels (optionnel, détection automatique si non spécifié)
    #[arg(long)]
    cell_size: Option<u32>,

    /// Marge gauche en pixels (optionnel, détection automatique si non spécifié)
    #[arg(long)]
    margin_left: Option<u32>,

    /// Marge haute en pixels (optionnel, détection automatique si non spécifié)
    #[arg(long)]
    margin_top: Option<u32>,

    /// Mode verbeux pour afficher les détails
    #[arg(short, long)]
    verbose: bool,

    /// Utiliser le solveur avancé (techniques avancées)
    #[arg(long)]
    advanced: bool,

    /// Utiliser le solveur ultime (toutes les techniques + backtracking + parallélisation)
    #[arg(long)]
    ultimate: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Déterminer le format d'export
    let output_format = if let Some(ref format_str) = args.export_format {
        GridOutputFormatter::parse_format_str(format_str).ok_or_else(|| {
            anyhow::anyhow!(
                "Format d'export invalide: '{}'. Options: ascii, json, array",
                format_str
            )
        })?
    } else if let Some(ref export_path) = args.export {
        GridOutputFormatter::detect_format_from_path(export_path)
    } else {
        OutputFormat::Ascii // Défaut
    };

    // Charger ou extraire les contraintes
    let constraints = if args.interactive {
        // Mode interactif
        if args.verbose {
            println!("🎮 Mode interactif: saisie des contraintes");
        }

        // Charger l'image pour détecter la taille de la grille
        let input_image = ImageParser::load_image(&args.input)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;

        // Détecter la taille de la grille
        let (_grid_x, _grid_y, _cell_size, grid_width, grid_height) =
            AdvancedConstraintExtractor::detect_grid(&input_image)
                .map_err(|e| anyhow::anyhow!("Impossible de détecter la grille: {}. Vérifiez que l'image contient une grille claire.", e))?;

        InteractiveInput::input_constraints(grid_width, grid_height)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la saisie interactive: {}", e))?
    } else if args.extract_filled {
        // Extraction depuis les cases remplies
        if args.verbose {
            println!("🔍 Extraction des contraintes depuis les cases remplies...");
        }

        let input_image = ImageParser::load_image(&args.input)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;

        // Détecter ou utiliser la configuration manuelle
        let parser_config = if let (Some(cell_size), Some(margin_left), Some(margin_top)) =
            (args.cell_size, args.margin_left, args.margin_top)
        {
            image_parser::ParserConfig {
                cell_size,
                margin_left,
                margin_top,
                ..Default::default()
            }
        } else {
            ImageParser::auto_detect_config(&input_image, 10, 10).unwrap_or_default()
        };

        match AdvancedConstraintExtractor::extract_constraints_from_filled_cells(
            &input_image,
            &parser_config,
        ) {
            Ok(constraints) => {
                if args.verbose {
                    println!(
                        "✓ Contraintes extraites: {}x{}",
                        constraints.width, constraints.height
                    );
                }
                constraints
            }
            Err(e) => {
                eprintln!("⚠️  Extraction automatique échouée: {}", e);
                eprintln!();
                eprintln!("💡 Suggestions:");
                eprintln!("   1. Utilisez --interactive pour saisir les contraintes manuellement");
                eprintln!("   2. Vérifiez que l'image contient une grille clairement visible");
                eprintln!(
                    "   3. Essayez avec --use-ocr si l'image contient les contraintes numériques"
                );
                eprintln!();
                return Err(anyhow::anyhow!(
                    "Extraction échouée. Essayez avec --interactive."
                ));
            }
        }
    } else if args.use_ocr {
        // Mode OCR
        if args.verbose {
            println!("🔍 Extraction des contraintes par OCR...");
        }

        let _input_image = ImageParser::load_image(&args.input)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;

        #[cfg(feature = "ocr")]
        {
            AdvancedConstraintExtractor::extract_auto(&input_image)
                .map_err(|e| anyhow::anyhow!("Erreur lors de l'extraction OCR: {}", e))?
        }
        #[cfg(not(feature = "ocr"))]
        {
            return Err(anyhow::anyhow!("La fonctionnalité OCR n'est pas activée. Recompilez avec --features ocr ou utilisez --extract-filled"));
        }
    } else if let Some(ref constraints_file) = args.constraints {
        // Charger depuis un fichier JSON
        if args.verbose {
            println!("🔍 Chargement des contraintes depuis: {}", constraints_file);
        }

        Constraints::from_json_file(constraints_file)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement des contraintes: {}", e))?
    } else {
        // Aucune source de contraintes spécifiée - essayer extraction auto
        if args.verbose {
            println!("🔍 Aucune contrainte spécifiée, tentative d'extraction automatique...");
        }

        let input_image = ImageParser::load_image(&args.input)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;

        // Détecter la configuration
        let parser_config = if let (Some(cell_size), Some(margin_left), Some(margin_top)) =
            (args.cell_size, args.margin_left, args.margin_top)
        {
            image_parser::ParserConfig {
                cell_size,
                margin_left,
                margin_top,
                ..Default::default()
            }
        } else {
            ImageParser::auto_detect_config(&input_image, 10, 10).unwrap_or_default()
        };

        // Essayer l'extraction depuis les cases remplies
        match AdvancedConstraintExtractor::extract_constraints_from_filled_cells(
            &input_image,
            &parser_config,
        ) {
            Ok(constraints) => {
                if args.verbose {
                    println!(
                        "✓ Contraintes extraites automatiquement: {}x{}",
                        constraints.width, constraints.height
                    );
                }
                constraints
            }
            Err(e) => {
                eprintln!("⚠️  Extraction automatique échouée: {}", e);
                eprintln!();
                eprintln!("💡 Veuillez spécifier une source de contraintes:");
                eprintln!("   --extract-filled   : Extrait depuis les cases remplies de la grille");
                eprintln!("   --interactive      : Saisie manuelle des contraintes");
                eprintln!("   --use-ocr          : Extraction OCR (nécessite --features ocr)");
                eprintln!("   --constraints FILE : Fichier JSON de contraintes");
                eprintln!();
                return Err(anyhow::anyhow!("Veuillez spécifier une source de contraintes avec --extract-filled, --interactive, --use-ocr ou --constraints"));
            }
        }
    };

    if args.verbose {
        println!(
            "✓ Contraintes chargées: {}x{}",
            constraints.width, constraints.height
        );
    }

    // Charger l'image
    if args.verbose {
        println!("🔍 Chargement de l'image depuis: {}", args.input);
    }

    let input_image = ImageParser::load_image(&args.input)
        .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;

    if args.verbose {
        println!(
            "✓ Image chargée: {}x{} pixels",
            input_image.width(),
            input_image.height()
        );
    }

    // Configurer le parseur
    let parser_config = if let (Some(cell_size), Some(margin_left), Some(margin_top)) =
        (args.cell_size, args.margin_left, args.margin_top)
    {
        if args.verbose {
            println!("📐 Utilisation de la configuration manuelle:");
            println!("   - Taille de case: {} px", cell_size);
            println!("   - Marge gauche: {} px", margin_left);
            println!("   - Marge haute: {} px", margin_top);
        }
        image_parser::ParserConfig {
            cell_size,
            margin_left,
            margin_top,
            ..Default::default()
        }
    } else {
        if args.verbose {
            println!("🤖 Détection automatique de la configuration...");
        }
        let config =
            ImageParser::auto_detect_config(&input_image, constraints.width, constraints.height)
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Erreur lors de la détection automatique de la configuration: {}",
                        e
                    )
                })?;
        if args.verbose {
            println!("✓ Configuration détectée:");
            println!("   - Taille de case: {} px", config.cell_size);
            println!("   - Marge gauche: {} px", config.margin_left);
            println!("   - Marge haute: {} px", config.margin_top);
        }
        config
    };

    // Parser l'image pour extraire la grille
    if args.verbose {
        println!("🔍 Analyse de l'image pour extraire la grille...");
    }

    let parser = ImageParser::new(parser_config.clone());
    let mut grid = parser
        .parse_image(&input_image, constraints.width, constraints.height)
        .map_err(|e| anyhow::anyhow!("Erreur lors du parsing de l'image: {}", e))?;

    if args.verbose {
        println!("✓ Grille extraite");
    }

    // Mode extract-only: afficher la grille et terminer
    if args.extract_only {
        let output = GridOutputFormatter::format_grid(&grid, output_format);
        println!();
        println!("═══════════════════════════════════════════════════════════");
        println!("  GRILLE EXTRAITE (sans résolution)");
        println!("═══════════════════════════════════════════════════════════");
        println!();
        println!("{}", output);

        // Exporter vers un fichier si demandé
        if let Some(ref export_path) = args.export {
            std::fs::write(export_path, &output)
                .map_err(|e| anyhow::anyhow!("Erreur lors de l'écriture du fichier: {}", e))?;
            println!("✅ Grille exportée vers: {}", export_path);
        }

        return Ok(());
    }

    // Choisir le solveur en fonction des options
    let deductions = if args.ultimate {
        if args.verbose {
            println!("🌟 Résolution avec le solveur ultime...");
        }

        let config = UltimateSolverConfig {
            use_parallel: true,
            use_backtracking: true,
            backtracking_depth: 10,
            verbose: args.verbose,
        };

        let mut ultimate_solver = UltimateSolver::with_config(config);
        ultimate_solver
            .solve(&mut grid, &constraints)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la résolution: {}", e))?
    } else if args.advanced {
        if args.verbose {
            println!("🚀 Résolution avec le solveur avancé...");
        }

        let config = AdvancedSolverConfig {
            use_cross_analysis: true,
            use_advanced_heuristics: true,
            max_iterations: 100,
            verbose: args.verbose,
        };

        let mut advanced_solver = AdvancedSolver::with_config(config);
        advanced_solver
            .solve(&mut grid, &constraints)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la résolution: {}", e))?
    } else {
        if args.verbose {
            println!("🧩 Résolution de la grille par déduction logique...");
        }

        let mut solver = NonogramSolver::new();
        let deductions = solver
            .solve(&mut grid, &constraints)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la résolution: {}", e))?;

        if args.verbose {
            println!(
                "✓ Résolution terminée: {} déductions trouvées",
                deductions.len()
            );
            let filled_count = deductions
                .iter()
                .filter(|d| d.state == grid::CellState::Filled)
                .count();
            let crossed_count = deductions
                .iter()
                .filter(|d| d.state == grid::CellState::Crossed)
                .count();
            println!("   - Cases noires déduites: {}", filled_count);
            println!("   - Cases barrées déduites: {}", crossed_count);
        }

        deductions
    };

    if deductions.is_empty() {
        println!("ℹ️  Aucune nouvelle déduction possible avec la logique actuelle.");
        println!("   La grille est soit complète, soit nécessite des techniques avancées.");
    }

    // Afficher la grille résolue si pas de sortie image spécifiée ou si export demandé
    let show_grid_output = args.output.is_none() || args.export.is_some();

    if show_grid_output {
        let output =
            GridOutputFormatter::format_grid_with_deductions(&grid, &deductions, output_format);
        println!();
        println!("═══════════════════════════════════════════════════════════");
        println!("  GRILLE RÉSOLUE");
        println!("═══════════════════════════════════════════════════════════");
        println!();
        println!("{}", output);
    }

    // Exporter vers un fichier si demandé
    if let Some(ref export_path) = args.export {
        let output =
            GridOutputFormatter::format_grid_with_deductions(&grid, &deductions, output_format);
        std::fs::write(export_path, &output)
            .map_err(|e| anyhow::anyhow!("Erreur lors de l'écriture du fichier: {}", e))?;
        println!("✅ Grille exportée vers: {}", export_path);
    }

    // Générer l'image de sortie si demandé
    if let Some(ref output_path) = args.output {
        if args.verbose {
            println!("🎨 Génération de l'image de sortie...");
        }

        let generator_config = ImageGenerator::from_parser_config(
            parser_config.cell_size,
            parser_config.margin_top,
            parser_config.margin_left,
        );
        let generator = ImageGenerator::new(generator_config);
        let output_image = generator
            .generate_output_image(&input_image, &deductions)
            .map_err(|e| {
                anyhow::anyhow!("Erreur lors de la génération de l'image de sortie: {}", e)
            })?;

        // Sauvegarder l'image
        if args.verbose {
            println!("💾 Sauvegarde de l'image vers: {}", output_path);
        }

        ImageGenerator::save_image(&output_image, output_path)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la sauvegarde de l'image: {}", e))?;

        println!("✅ Terminé! Image sauvegardée: {}", output_path);
        if !deductions.is_empty() {
            println!("   {} cases ont été marquées en rouge", deductions.len());
        }
    } else {
        println!("✅ Terminé!");
        if !deductions.is_empty() {
            println!("   {} déductions effectuées", deductions.len());
        }
    }

    Ok(())
}
