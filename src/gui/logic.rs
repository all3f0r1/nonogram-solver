use slint::{Image, SharedPixelBuffer, Rgb8Pixel, VecModel, ModelRc};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

slint::include_modules!();

pub fn run_gui() -> Result<(), Box<dyn std::error::Error>> {
    let app = MainWindow::new()?;
    
    // État partagé
    let current_file = Arc::new(Mutex::new(Option::<PathBuf>::None));
    let current_constraints = Arc::new(Mutex::new(Option::<crate::grid::Constraints>::None));
    let result_image = Arc::new(Mutex::new(Option::<image::DynamicImage>::None));
    let history = Arc::new(Mutex::new(crate::gui::history::History::load().unwrap_or_default()));
    
    // Callback: Parcourir fichier
    {
        let app_weak = app.as_weak();
        app.on_browse_file(move || {
            let app = app_weak.upgrade().unwrap();
            
            // Utiliser rfd pour le dialogue de fichier
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["png", "jpg", "jpeg", "bmp", "gif", "tiff", "webp"])
                .pick_file()
            {
                app.set_file_path(path.display().to_string().into());
                app.set_can_load(true);
            }
        });
    }
    
    // Callback: Charger image
    {
        let app_weak = app.as_weak();
        let current_file_clone = current_file.clone();
        
        app.on_load_image(move || {
            let app = app_weak.upgrade().unwrap();
            let file_path = app.get_file_path().to_string();
            
            if file_path.is_empty() {
                return;
            }
            
            // Charger l'image
            match image::open(&file_path) {
                Ok(img) => {
                    // Convertir en Slint Image
                    let rgba_img = img.to_rgba8();
                    let (width, height) = rgba_img.dimensions();
                    
                    let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                        &rgba_img.as_raw().chunks(4).map(|chunk| {
                            Rgb8Pixel { r: chunk[0], g: chunk[1], b: chunk[2] }
                        }).collect::<Vec<_>>(),
                        width,
                        height,
                    );
                    
                    let slint_image = Image::from_rgb8(buffer);
                    app.set_input_image(slint_image);
                    app.set_can_solve(true);
                    app.set_status("Image chargée avec succès".into());
                    
                    // Sauvegarder le chemin
                    *current_file_clone.lock().unwrap() = Some(PathBuf::from(file_path));
                }
                Err(e) => {
                    app.set_status(format!("Erreur: {}", e).into());
                }
            }
        });
    }
    
    // Callback: Résoudre
    {
        let app_weak = app.as_weak();
        let current_file_clone = current_file.clone();
        let result_image_clone = result_image.clone();
        let history_clone = history.clone();
        
        app.on_solve(move || {
            let app = app_weak.upgrade().unwrap();
            let file_path_opt = current_file_clone.lock().unwrap().clone();
            
            if file_path_opt.is_none() {
                app.set_status("Veuillez d'abord charger une image".into());
                return;
            }
            
            let file_path = file_path_opt.unwrap();
            let solver_mode = app.get_solver_mode();
            let auto_detect = app.get_auto_detect();
            let cell_size = app.get_cell_size() as u32;
            let margin_left = app.get_margin_left() as u32;
            let margin_top = app.get_margin_top() as u32;
            
            // Marquer comme en cours de résolution
            app.set_is_solving(true);
            app.set_progress(0.0);
            app.set_status("Chargement de l'image...".into());
            
            // Lancer la résolution dans un thread séparé
            let app_weak_clone = app_weak.clone();
            thread::spawn(move || {
                // Charger l'image
                let img = match image::open(&file_path) {
                    Ok(img) => img,
                    Err(e) => {
                        let app = app_weak_clone.upgrade().unwrap();
                        app.set_status(format!("Erreur: {}", e).into());
                        app.set_is_solving(false);
                        return;
                    }
                };
                
                // Charger les contraintes
                // Essayer d'abord l'extraction automatique, puis fallback sur JSON
                let constraints = {
                    // Essayer l'extraction automatique
                    match crate::gui::constraint_extractor::ConstraintExtractor::extract(&img) {
                        Ok(c) => {
                            let app = app_weak_clone.upgrade().unwrap();
                            app.set_status("Contraintes extraites automatiquement".into());
                            c
                        },
                        Err(_) => {
                            // Fallback: essayer de charger depuis JSON
                            let constraints_path = file_path.with_extension("json");
                            match std::fs::read_to_string(&constraints_path) {
                                Ok(json) => match serde_json::from_str(&json) {
                                    Ok(c) => {
                                        let app = app_weak_clone.upgrade().unwrap();
                                        app.set_status("Contraintes chargées depuis JSON".into());
                                        c
                                    },
                                    Err(e) => {
                                        let app = app_weak_clone.upgrade().unwrap();
                                        app.set_status(format!("Erreur: extraction auto échouée et JSON invalide: {}", e).into());
                                        app.set_is_solving(false);
                                        return;
                                    }
                                },
                                Err(_) => {
                                    let app = app_weak_clone.upgrade().unwrap();
                                    app.set_status("Erreur: extraction auto échouée et pas de fichier JSON".into());
                                    app.set_is_solving(false);
                                    return;
                                }
                            }
                        }
                    }
                };
                
                // Mettre à jour le statut
                {
                    let app = app_weak_clone.upgrade().unwrap();
                    app.set_progress(10.0);
                    app.set_status("Analyse de l'image...".into());
                }
                
                // Parser l'image
                let config = if auto_detect {
                    match crate::image_parser::ImageParser::auto_detect_config(&img, &constraints) {
                        Ok(c) => c,
                        Err(e) => {
                            let app = app_weak_clone.upgrade().unwrap();
                            app.set_status(format!("Erreur de détection: {}", e).into());
                            app.set_is_solving(false);
                            return;
                        }
                    }
                } else {
                    crate::image_parser::ImageParserConfig {
                        cell_size,
                        margin_left,
                        margin_top,
                    }
                };
                
                let parser = crate::image_parser::ImageParser::new(config);
                let mut grid = match parser.parse(&img, &constraints) {
                    Ok(g) => g,
                    Err(e) => {
                        let app = app_weak_clone.upgrade().unwrap();
                        app.set_status(format!("Erreur de parsing: {}", e).into());
                        app.set_is_solving(false);
                        return;
                    }
                };
                
                // Mettre à jour le statut
                {
                    let app = app_weak_clone.upgrade().unwrap();
                    app.set_progress(20.0);
                    app.set_status("Résolution en cours...".into());
                }
                
                // Résoudre selon le mode
                let deductions = match solver_mode {
                    0 => {
                        // Basique
                        let mut solver = crate::solver::NonogramSolver::new(constraints.clone());
                        match solver.solve(&mut grid, true) {
                            Ok(d) => d,
                            Err(e) => {
                                let app = app_weak_clone.upgrade().unwrap();
                                app.set_status(format!("Erreur de résolution: {}", e).into());
                                app.set_is_solving(false);
                                return;
                            }
                        }
                    }
                    1 => {
                        // Avancé
                        let mut solver = crate::solver::AdvancedSolver::new(constraints.clone());
                        match solver.solve(&mut grid, true) {
                            Ok(d) => d,
                            Err(e) => {
                                let app = app_weak_clone.upgrade().unwrap();
                                app.set_status(format!("Erreur de résolution: {}", e).into());
                                app.set_is_solving(false);
                                return;
                            }
                        }
                    }
                    2 => {
                        // Ultime
                        let mut solver = crate::solver::UltimateSolver::new(constraints.clone());
                        match solver.solve(&mut grid, true) {
                            Ok(d) => d,
                            Err(e) => {
                                let app = app_weak_clone.upgrade().unwrap();
                                app.set_status(format!("Erreur de résolution: {}", e).into());
                                app.set_is_solving(false);
                                return;
                            }
                        }
                    }
                    _ => {
                        let app = app_weak_clone.upgrade().unwrap();
                        app.set_status("Mode de solveur invalide".into());
                        app.set_is_solving(false);
                        return;
                    }
                };
                
                // Mettre à jour le statut
                {
                    let app = app_weak_clone.upgrade().unwrap();
                    app.set_progress(80.0);
                    app.set_status("Génération de l'image de résultat...".into());
                }
                
                // Générer l'image de résultat
                let generator = crate::image_generator::ImageGenerator::new(
                    crate::image_generator::ImageGeneratorConfig::default()
                );
                
                let result_img = match generator.generate(&img, &grid, &deductions, &config) {
                    Ok(img) => img,
                    Err(e) => {
                        let app = app_weak_clone.upgrade().unwrap();
                        app.set_status(format!("Erreur de génération: {}", e).into());
                        app.set_is_solving(false);
                        return;
                    }
                };
                
                // Stocker l'image de résultat pour la sauvegarde
                *result_image_clone.lock().unwrap() = Some(result_img.clone());
                
                // Convertir en Slint Image
                let rgba_img = result_img.to_rgba8();
                let (width, height) = rgba_img.dimensions();
                
                let buffer = SharedPixelBuffer::<Rgb8Pixel>::clone_from_slice(
                    &rgba_img.as_raw().chunks(4).map(|chunk| {
                        Rgb8Pixel { r: chunk[0], g: chunk[1], b: chunk[2] }
                    }).collect::<Vec<_>>(),
                    width,
                    height,
                );
                
                let slint_image = Image::from_rgb8(buffer);
                
                // Mettre à jour l'interface
                let app = app_weak_clone.upgrade().unwrap();
                app.set_result_image(slint_image);
                app.set_progress(100.0);
                app.set_status(format!("Résolution terminée ! {} déductions trouvées", deductions.len()).into());
                app.set_can_save(true);
                app.set_is_solving(false);
                
                // Ajouter à l'historique
                let solver_mode_name = match solver_mode {
                    0 => "Basique",
                    1 => "Avancé",
                    2 => "Ultime",
                    _ => "Inconnu",
                };
                
                let entry = crate::gui::history::HistoryEntry {
                    file_path: file_path.display().to_string(),
                    timestamp: chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                    solver_mode: solver_mode_name.to_string(),
                    deductions_count: deductions.len(),
                };
                
                let mut hist = history_clone.lock().unwrap();
                hist.add_entry(entry);
                let _ = hist.save();
            });
        });
    }
    
    // Callback: Sauvegarder résultat
    {
        let app_weak = app.as_weak();
        let result_image_clone = result_image.clone();
        
        app.on_save_result(move || {
            let app = app_weak.upgrade().unwrap();
            
            // Ouvrir dialogue de sauvegarde
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("PNG", &["png"])
                .add_filter("JPEG", &["jpg", "jpeg"])
                .set_file_name("result.png")
                .save_file()
            {
                // Récupérer l'image de résultat depuis l'état partagé
                let result_img_opt = result_image_clone.lock().unwrap().clone();
                
                if let Some(result_img) = result_img_opt {
                    // Sauvegarder l'image
                    match result_img.save(&path) {
                        Ok(_) => {
                            app.set_status(format!("Résultat sauvegardé: {}", path.display()).into());
                        }
                        Err(e) => {
                            app.set_status(format!("Erreur de sauvegarde: {}", e).into());
                        }
                    }
                } else {
                    app.set_status("Aucun résultat à sauvegarder".into());
                }
            }
        });
    }
    
    app.run()?;
    Ok(())
}
