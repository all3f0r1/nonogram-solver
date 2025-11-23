mod grid;
mod solver;
mod image_parser;
mod image_generator;
mod ocr;
mod drawing;
mod edge_detection;

use clap::Parser;
use anyhow::Result;

use grid::Constraints;
use solver::{NonogramSolver, AdvancedSolver, AdvancedSolverConfig, UltimateSolver, UltimateSolverConfig};
use image_parser::ImageParser;
use image_generator::ImageGenerator;
use ocr::AdvancedConstraintExtractor;

/// Solveur de nonogramme (logimage/hanjie) par déduction logique
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chemin vers l'image d'entrée du nonogramme
    #[arg(short, long)]
    input: String,

    /// Chemin vers le fichier JSON contenant les contraintes (optionnel si --auto est activé)
    #[arg(short, long)]
    constraints: Option<String>,

    /// Extraction automatique des contraintes depuis l'image (sans OCR, par détection de grille)
    #[arg(long)]
    auto: bool,

    /// Utiliser l'OCR pour extraire automatiquement les contraintes de l'image (nécessite --features ocr)
    #[arg(long)]
    use_ocr: bool,

    /// Chemin vers l'image de sortie avec les déductions marquées
    #[arg(short, long)]
    output: String,

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

    // Charger ou extraire les contraintes
    let constraints = if args.auto || args.use_ocr {
        if args.verbose {
            if args.use_ocr {
                println!("🔍 Extraction des contraintes par OCR...");
            } else {
                println!("🤖 Détection automatique de la grille et extraction des contraintes...");
            }
        }
        
        // Charger l'image d'abord
        let input_image = ImageParser::load_image(&args.input)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;
        
        // Extraire automatiquement les contraintes
        if args.use_ocr {
            #[cfg(feature = "ocr")]
            {
                AdvancedConstraintExtractor::extract_auto(&input_image)
                    .map_err(|e| anyhow::anyhow!("Erreur lors de l'extraction OCR: {}", e))?
            }
            #[cfg(not(feature = "ocr"))]
            {
                return Err(anyhow::anyhow!("La fonctionnalité OCR n'est pas activée. Recompilez avec --features ocr"));
            }
        } else {
            // Mode auto: détecter la grille sans OCR
            #[cfg(feature = "ocr")]
            {
                AdvancedConstraintExtractor::extract_auto(&input_image)
                    .map_err(|e| anyhow::anyhow!("Erreur lors de l'extraction automatique: {}. Essayez avec --constraints", e))?
            }
            #[cfg(not(feature = "ocr"))]
            {
                AdvancedConstraintExtractor::extract_from_image_heuristic(&input_image)
                    .map_err(|e| anyhow::anyhow!("Erreur lors de la détection de grille: {}. Utilisez --constraints", e))?
            }
        }
    } else {
        let constraints_file = args.constraints
            .ok_or_else(|| anyhow::anyhow!("Vous devez spécifier --constraints, --auto ou --use-ocr"))?;
        
        if args.verbose {
            println!("🔍 Chargement des contraintes depuis: {}", constraints_file);
        }
        
        Constraints::from_json_file(&constraints_file)
            .map_err(|e| anyhow::anyhow!("Erreur lors du chargement des contraintes: {}", e))?
    };

    if args.verbose {
        println!("✓ Contraintes chargées: {}x{}", constraints.width, constraints.height);
    }

    // Charger l'image
    if args.verbose {
        println!("🔍 Chargement de l'image depuis: {}", args.input);
    }

    let input_image = ImageParser::load_image(&args.input)
        .map_err(|e| anyhow::anyhow!("Erreur lors du chargement de l'image: {}", e))?;

    if args.verbose {
        println!("✓ Image chargée: {}x{} pixels", input_image.width(), input_image.height());
    }

    // Configurer le parseur
    let parser_config = if let (Some(cell_size), Some(margin_left), Some(margin_top)) = 
        (args.cell_size, args.margin_left, args.margin_top) {
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
        let config = ImageParser::auto_detect_config(&input_image, constraints.width, constraints.height)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la détection automatique de la configuration: {}", e))?;
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
    let mut grid = parser.parse_image(&input_image, constraints.width, constraints.height)
        .map_err(|e| anyhow::anyhow!("Erreur lors du parsing de l'image: {}", e))?;

    if args.verbose {
        println!("✓ Grille extraite");
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
        ultimate_solver.solve(&mut grid, &constraints)
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
        advanced_solver.solve(&mut grid, &constraints)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la résolution: {}", e))?
    } else {
        if args.verbose {
            println!("🧩 Résolution de la grille par déduction logique...");
        }

        let mut solver = NonogramSolver::new();
        let deductions = solver.solve(&mut grid, &constraints)
            .map_err(|e| anyhow::anyhow!("Erreur lors de la résolution: {}", e))?;

        if args.verbose {
            println!("✓ Résolution terminée: {} déductions trouvées", deductions.len());
            let filled_count = deductions.iter().filter(|d| d.state == grid::CellState::Filled).count();
            let crossed_count = deductions.iter().filter(|d| d.state == grid::CellState::Crossed).count();
            println!("   - Cases noires déduites: {}", filled_count);
            println!("   - Cases barrées déduites: {}", crossed_count);
        }

        deductions
    };

    if deductions.is_empty() {
        println!("ℹ️  Aucune nouvelle déduction possible avec la logique actuelle.");
        println!("   La grille est soit complète, soit nécessite des techniques avancées.");
    }

    // Générer l'image de sortie
    if args.verbose {
        println!("🎨 Génération de l'image de sortie...");
    }

    let generator_config = ImageGenerator::from_parser_config(
        parser_config.cell_size,
        parser_config.margin_top,
        parser_config.margin_left,
    );
    let generator = ImageGenerator::new(generator_config);
    let output_image = generator.generate_output_image(&input_image, &deductions)
        .map_err(|e| anyhow::anyhow!("Erreur lors de la génération de l'image de sortie: {}", e))?;

    // Sauvegarder l'image
    if args.verbose {
        println!("💾 Sauvegarde de l'image vers: {}", args.output);
    }

    ImageGenerator::save_image(&output_image, &args.output)
        .map_err(|e| anyhow::anyhow!("Erreur lors de la sauvegarde de l'image: {}", e))?;

    println!("✅ Terminé! Image sauvegardée: {}", args.output);
    if !deductions.is_empty() {
        println!("   {} cases ont été marquées en rouge", deductions.len());
    }

    Ok(())
}
