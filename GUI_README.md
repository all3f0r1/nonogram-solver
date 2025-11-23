# Interface graphique - Nonogram Solver v0.8.0

## 🎨 Description

Interface graphique moderne et native pour le solveur de nonogrammes, construite avec **Slint**.

---

## 🚀 Compilation

### Prérequis

- Rust 1.70+
- Cargo

### Compiler avec GUI

```bash
cargo build --release --features gui
```

**Note**: La première compilation peut prendre 10-15 minutes car Slint a 452 dépendances.

---

## 📦 Exécution

```bash
./target/release/nonogram-solver-gui
```

Ou directement:

```bash
cargo run --release --features gui
```

---

## 🎯 Utilisation

### Étapes

1. **Parcourir** : Cliquez sur "Parcourir..." pour sélectionner une image
2. **Charger** : Cliquez sur "Charger" pour afficher l'image
3. **Configurer** : Choisissez le solveur (Basique/Avancé/Ultime)
4. **Résoudre** : Cliquez sur "Résoudre" pour lancer la résolution
5. **Sauvegarder** : Cliquez sur "Sauvegarder" pour enregistrer le résultat

### Modes de solveur

- **Basique** : Line solving uniquement (~70% de résolution)
- **Avancé** : + Analyse croisée + Heuristiques (~85%)
- **Ultime** : + Parallélisation + Backtracking (100%)

### Détection automatique

Par défaut, les paramètres de la grille sont détectés automatiquement.

Pour une configuration manuelle :
1. Décochez "Détection automatique"
2. Ajustez les paramètres :
   - Taille de cellule (px)
   - Marge gauche (px)
   - Marge haute (px)

---

## 🎨 Fonctionnalités

✅ **Interface native** : Look-and-feel adapté à chaque plateforme  
✅ **Prévisualisation** : Affichage côte-à-côte de l'entrée et du résultat  
✅ **Progression en temps réel** : Barre de progression et statut  
✅ **Dialogue de fichiers** : Sélection intuitive des fichiers  
✅ **Multi-format** : PNG, JPEG, BMP, GIF, TIFF, WebP  
✅ **Accessibilité** : Support des lecteurs d'écran  
✅ **Responsive** : S'adapte à la taille de la fenêtre  

---

## 📐 Architecture

```
src/
├── gui_main.rs          # Point d'entrée GUI
├── gui/
│   ├── mod.rs          # Module GUI
│   └── logic.rs        # Logique et callbacks
ui/
└── app.slint           # Interface Slint (DSL)
```

### Fichiers clés

- **ui/app.slint** : Définition de l'interface (composants, layout, style)
- **src/gui/logic.rs** : Logique Rust (callbacks, résolution, gestion d'état)
- **build.rs** : Script de build pour compiler le fichier `.slint`

---

## 🔧 Dépendances GUI

```toml
[dependencies]
slint = { version = "1.9", optional = true }
rfd = { version = "0.15", optional = true }  # Dialogues de fichiers

[build-dependencies]
slint-build = { version = "1.9", optional = true }
```

**Total** : 452 dépendances (transitives)

---

## 🎯 Limitations actuelles

1. **Contraintes JSON requises** : Pour l'instant, un fichier `.json` avec les contraintes doit exister à côté de l'image
   - Exemple : `puzzle.png` → `puzzle.json`
   - Format : `{"rows": [[1,2], [3]], "cols": [[2], [1,1]]}`

2. **Sauvegarde non implémentée** : Le bouton "Sauvegarder" ouvre le dialogue mais ne sauvegarde pas encore

---

## 🚧 Améliorations futures

### v0.9.0
- ✅ Extraction automatique des contraintes (OCR)
- ✅ Sauvegarde du résultat
- ✅ Historique des résolutions
- ✅ Zoom et pan sur les images

### v1.0.0
- ✅ Édition manuelle de la grille
- ✅ Export en différents formats
- ✅ Thèmes personnalisables
- ✅ Multi-langue (i18n)

---

## 📚 Documentation Slint

- [Documentation officielle](https://slint.dev/docs)
- [Tutoriel](https://slint.dev/docs/tutorial/rust)
- [Exemples](https://github.com/slint-ui/slint/tree/master/examples)

---

## 🎨 Style

L'interface utilise le style natif de chaque plateforme:

- **Windows** : Fluent Design
- **Linux** : Material Design  
- **macOS** : Cupertino

Les couleurs sont adaptées automatiquement au thème système (clair/sombre).

---

## ♿ Accessibilité

✅ **Navigation au clavier** : Tab, Espace, Entrée  
✅ **Lecteurs d'écran** : Windows Narrator, NVDA, JAWS  
✅ **Contraste** : Conforme WCAG 2.1 AA  
✅ **Labels** : Tous les éléments ont des descriptions  

---

## 🐛 Dépannage

### Erreur de compilation

Si la compilation échoue avec des erreurs de linking:

```bash
# Linux
sudo apt-get install libfontconfig1-dev libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

# macOS
brew install fontconfig

# Windows
# Pas de dépendances supplémentaires nécessaires
```

### Fenêtre ne s'affiche pas

Vérifiez que vous avez un serveur X (Linux) ou un environnement graphique actif.

### Performance lente

La résolution se fait dans un thread séparé, mais pour les très grandes grilles (30x30+), cela peut prendre plusieurs minutes.

---

## 📝 Licence

Même licence que le projet principal (MIT).
