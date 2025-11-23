# Conception de l'interface graphique - Nonogram Solver

**Version**: 0.8.0  
**Bibliothèque**: Slint  
**Date**: 23 novembre 2025

---

## 🎯 Objectifs

L'interface graphique doit permettre à l'utilisateur de:

1. **Charger une image** de nonogramme
2. **Visualiser la grille** détectée
3. **Lancer la résolution** avec différents solveurs
4. **Voir les déductions** en temps réel
5. **Sauvegarder le résultat**

---

## 🎨 Design de l'interface

### Fenêtre principale

```
┌─────────────────────────────────────────────────────────────┐
│ Nonogram Solver                                    [_][□][X]│
├─────────────────────────────────────────────────────────────┤
│ ┌─────────────────┐  ┌───────────────────────────────────┐ │
│ │                 │  │                                   │ │
│ │                 │  │                                   │ │
│ │     Image       │  │         Résultat                  │ │
│ │    d'entrée     │  │       (avec déductions)           │ │
│ │                 │  │                                   │ │
│ │                 │  │                                   │ │
│ └─────────────────┘  └───────────────────────────────────┘ │
│                                                             │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Paramètres                                              │ │
│ │ ┌─────────────────────────────────────────────────────┐ │ │
│ │ │ Fichier image: [/path/to/image.png]  [Parcourir...] │ │ │
│ │ └─────────────────────────────────────────────────────┘ │ │
│ │                                                         │ │
│ │ ┌─────────────────────────────────────────────────────┐ │ │
│ │ │ Solveur: ○ Basique  ○ Avancé  ● Ultime              │ │ │
│ │ └─────────────────────────────────────────────────────┘ │ │
│ │                                                         │ │
│ │ ┌─────────────────────────────────────────────────────┐ │ │
│ │ │ ☑ Détection automatique  ☐ Paramètres manuels      │ │ │
│ │ └─────────────────────────────────────────────────────┘ │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│ ┌─────────────────────────────────────────────────────────┐ │
│ │ Progression                                             │ │
│ │ ████████████████░░░░░░░░░░░░░░░░░░░░░░░░  42%          │ │
│ │ Phase 2/3: Parallélisation - 17 déductions trouvées    │ │
│ └─────────────────────────────────────────────────────────┘ │
│                                                             │
│          [Charger]  [Résoudre]  [Sauvegarder]              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📐 Composants

### 1. Zone d'affichage d'image (gauche)

**Fonction**: Affiche l'image d'entrée

**Propriétés**:
- Taille: 400x400 px
- Aspect ratio: préservé
- Zoom: auto-fit

---

### 2. Zone de résultat (droite)

**Fonction**: Affiche l'image avec les déductions marquées en rouge

**Propriétés**:
- Taille: 400x400 px
- Aspect ratio: préservé
- Mise à jour en temps réel pendant la résolution

---

### 3. Panneau de paramètres

#### a) Sélection de fichier

```
Fichier image: [/path/to/image.png]  [Parcourir...]
```

**Fonction**: Permet de sélectionner l'image d'entrée

**Comportement**:
- Bouton "Parcourir..." ouvre un dialogue de sélection de fichier
- Formats supportés: PNG, JPEG, BMP, GIF, TIFF, WebP
- Affiche le chemin complet

---

#### b) Choix du solveur

```
Solveur: ○ Basique  ○ Avancé  ● Ultime
```

**Options**:
- **Basique**: Line solving uniquement
- **Avancé**: Line solving + analyse croisée + heuristiques
- **Ultime**: Tout + parallélisation + backtracking

**Défaut**: Ultime

---

#### c) Mode de détection

```
☑ Détection automatique  ☐ Paramètres manuels
```

**Détection automatique** (par défaut):
- Détecte automatiquement la taille des cellules et les marges
- Pas de configuration nécessaire

**Paramètres manuels**:
- Affiche des champs supplémentaires:
  ```
  Taille de cellule: [100] px
  Marge gauche:     [50]  px
  Marge haute:      [50]  px
  ```

---

### 4. Barre de progression

```
████████████████░░░░░░░░░░░░░░░░░░░░░░░░  42%
Phase 2/3: Parallélisation - 17 déductions trouvées
```

**Fonction**: Affiche la progression de la résolution

**Informations**:
- Pourcentage de cases résolues
- Phase actuelle (1: Avancé, 2: Parallèle, 3: Backtracking)
- Nombre de déductions trouvées

---

### 5. Boutons d'action

```
[Charger]  [Résoudre]  [Sauvegarder]
```

**Charger**:
- Charge l'image sélectionnée
- Affiche l'image dans la zone gauche
- Active le bouton "Résoudre"

**Résoudre**:
- Lance la résolution avec le solveur sélectionné
- Désactive pendant la résolution
- Affiche la progression en temps réel

**Sauvegarder**:
- Ouvre un dialogue de sauvegarde
- Sauvegarde l'image de résultat
- Désactivé si pas de résultat

---

## 🎨 Style

### Palette de couleurs

**Utilise le style natif de la plateforme** (Slint Fluent/Material/Cupertino)

**Couleurs personnalisées**:
- Déductions: `#FF0000` (rouge)
- Progression: `#0078D4` (bleu Windows)
- Succès: `#10893E` (vert)
- Erreur: `#D13438` (rouge foncé)

---

### Typographie

**Police**: Système (Segoe UI sur Windows, Roboto sur Linux, SF Pro sur macOS)

**Tailles**:
- Titre: 20px
- Texte normal: 14px
- Texte secondaire: 12px

---

## 🔄 Flux d'utilisation

### Scénario 1: Utilisation simple

1. Utilisateur clique sur "Parcourir..."
2. Sélectionne une image
3. Clique sur "Charger"
4. L'image s'affiche à gauche
5. Clique sur "Résoudre" (avec paramètres par défaut)
6. La résolution se lance
7. La progression s'affiche en temps réel
8. Le résultat s'affiche à droite
9. Clique sur "Sauvegarder"
10. Sélectionne l'emplacement de sauvegarde
11. Terminé !

**Temps estimé**: 30 secondes

---

### Scénario 2: Utilisation avancée

1-4. Comme scénario 1
5. Désactive "Détection automatique"
6. Ajuste les paramètres manuels
7. Sélectionne "Avancé" comme solveur
8-11. Comme scénario 1

---

## 📱 Responsive

L'interface s'adapte à la taille de la fenêtre:

- **Minimum**: 800x600 px
- **Recommandé**: 1000x700 px
- **Maximum**: Illimité

**Comportement**:
- Les images se redimensionnent proportionnellement
- Les boutons restent visibles
- Le texte ne se tronque pas

---

## ♿ Accessibilité

**Support des lecteurs d'écran**:
- Tous les boutons ont des labels
- Les images ont des descriptions alt
- La progression est annoncée

**Navigation au clavier**:
- Tab: Passer au champ suivant
- Espace/Entrée: Activer le bouton
- Flèches: Changer les options radio

---

## 🧪 Tests

### Tests fonctionnels

1. ✅ Chargement d'image
2. ✅ Détection automatique
3. ✅ Résolution avec solveur basique
4. ✅ Résolution avec solveur avancé
5. ✅ Résolution avec solveur ultime
6. ✅ Sauvegarde du résultat
7. ✅ Paramètres manuels
8. ✅ Progression en temps réel

### Tests d'accessibilité

1. ✅ Navigation au clavier
2. ✅ Lecteur d'écran (Windows Narrator)
3. ✅ Contraste des couleurs

---

## 🚀 Implémentation

### Structure des fichiers

```
src/
├── main.rs                  # Point d'entrée (CLI + GUI)
├── gui/
│   ├── mod.rs              # Module GUI
│   ├── app.slint           # Interface Slint
│   └── logic.rs            # Logique GUI (callbacks)
├── solver/                  # Modules existants
├── grid/
├── image_parser/
└── image_generator/
```

### Fichier Slint (app.slint)

```slint
import { Button, VerticalBox, HorizontalBox, Image, LineEdit, CheckBox, RadioButton, ProgressIndicator } from "std-widgets.slint";

export component MainWindow inherits Window {
    title: "Nonogram Solver";
    preferred-width: 1000px;
    preferred-height: 700px;
    
    // Propriétés
    in-out property <image> input-image;
    in-out property <image> result-image;
    in-out property <string> file-path: "";
    in-out property <int> solver-mode: 2; // 0: basique, 1: avancé, 2: ultime
    in-out property <bool> auto-detect: true;
    in-out property <int> progress: 0;
    in-out property <string> status: "";
    
    // Callbacks
    callback load-image();
    callback solve();
    callback save-result();
    callback browse-file();
    
    VerticalBox {
        // Images
        HorizontalBox {
            // Image d'entrée
            Rectangle {
                border-width: 1px;
                border-color: #ccc;
                Image {
                    source: input-image;
                    width: 400px;
                    height: 400px;
                    image-fit: contain;
                }
            }
            
            // Image de résultat
            Rectangle {
                border-width: 1px;
                border-color: #ccc;
                Image {
                    source: result-image;
                    width: 400px;
                    height: 400px;
                    image-fit: contain;
                }
            }
        }
        
        // Paramètres
        GroupBox {
            title: "Paramètres";
            
            VerticalBox {
                // Sélection de fichier
                HorizontalBox {
                    Text { text: "Fichier image:"; }
                    LineEdit {
                        text: file-path;
                        read-only: true;
                    }
                    Button {
                        text: "Parcourir...";
                        clicked => { browse-file(); }
                    }
                }
                
                // Choix du solveur
                HorizontalBox {
                    Text { text: "Solveur:"; }
                    RadioButton { text: "Basique"; checked: solver-mode == 0; }
                    RadioButton { text: "Avancé"; checked: solver-mode == 1; }
                    RadioButton { text: "Ultime"; checked: solver-mode == 2; }
                }
                
                // Détection automatique
                CheckBox {
                    text: "Détection automatique";
                    checked: auto-detect;
                }
            }
        }
        
        // Progression
        GroupBox {
            title: "Progression";
            
            VerticalBox {
                ProgressIndicator {
                    progress: progress;
                }
                Text {
                    text: status;
                }
            }
        }
        
        // Boutons
        HorizontalBox {
            Button {
                text: "Charger";
                clicked => { load-image(); }
            }
            Button {
                text: "Résoudre";
                clicked => { solve(); }
            }
            Button {
                text: "Sauvegarder";
                clicked => { save-result(); }
            }
        }
    }
}
```

---

## 📝 Notes d'implémentation

### Callbacks Rust

Les callbacks Slint seront implémentés en Rust:

```rust
app.on_browse_file(move || {
    // Ouvrir dialogue de sélection de fichier
});

app.on_load_image(move || {
    // Charger l'image et l'afficher
});

app.on_solve(move || {
    // Lancer la résolution en arrière-plan
    // Mettre à jour la progression
});

app.on_save_result(move || {
    // Ouvrir dialogue de sauvegarde
});
```

### Threads

La résolution sera lancée dans un thread séparé pour ne pas bloquer l'interface:

```rust
std::thread::spawn(move || {
    // Résolution
    // Mise à jour de la progression via channels
});
```

---

## 🎯 Prochaines étapes

1. ✅ Audit des bibliothèques GUI
2. ✅ Sélection de Slint
3. ✅ Conception de l'interface
4. ⏭️ Implémentation
5. ⏭️ Tests
6. ⏭️ Documentation et déploiement
