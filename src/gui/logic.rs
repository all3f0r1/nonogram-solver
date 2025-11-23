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
                
                // Charger les contraintes (pour l'instant, utiliser un fichier JSON)
                // TODO: Implémenter l'extraction automatique
                let constraints_path = file_path.with_extension("json");
                let constraints = match std::fs::read_to_string(&constraints_path) {
                    Ok(json) => match serde_json::from_str(&json) {
                        Ok(c) => c,
                        Err(e) => {
                            let app = app_weak_clone.upgrade().unwrap();
                            app.set_status(format!("Erreur de parsing JSON: {}", e).into());
                            app.set_is_solving(false);
                            return;
                        }
                    },
                    Err(_) => {
                        let app = app_weak_clone.upgrade().unwrap();
                        app.set_status(format!("Fichier de contraintes non trouvé: {}", constraints_path.display()).into());
                        app.set_is_solving(false);
                        return;
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
            });
        });
    }
    
    // Callback: Sauvegarder résultat
    {
        let app_weak = app.as_weak();
        
        app.on_save_result(move || {
            let app = app_weak.upgrade().unwrap();
            
            // Ouvrir dialogue de sauvegarde
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("PNG", &["png"])
                .add_filter("JPEG", &["jpg", "jpeg"])
                .set_file_name("result.png")
                .save_file()
            {
                // TODO: Sauvegarder l'image de résultat
                app.set_status(format!("Résultat sauvegardé: {}", path.display()).into());
            }
        });
    }
    
    app.run()?;
    Ok(())
}
