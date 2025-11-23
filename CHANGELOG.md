# Changelog

Toutes les modifications notables de ce projet seront documentées dans ce fichier.

Le format est basé sur [Keep a Changelog](https://keepachangelog.com/fr/1.0.0/),
et ce projet adhère au [Semantic Versioning](https://semver.org/lang/fr/).

## [Non publié]

### À venir (v0.4.0)
- Implémentation des techniques avancées documentées
- Backtracking intelligent avec heuristiques
- Parallélisation avec Rayon
- Interface graphique (GUI) avec egui

## [0.3.0] - 2025-11-23

### Ajouté
- 📚 **Documentation complète** des techniques avancées de résolution dans [ADVANCED_TECHNIQUES.md](ADVANCED_TECHNIQUES.md)
- 🏛️ **Architecture modulaire** conçue pour intégrer les techniques avancées:
  - Module `CrossAnalyzer` pour l'analyse de contraintes croisées (overlap analysis, edge forcing)
  - Module `AdvancedHeuristics` pour les heuristiques avancées (glue, mercury, joining/splitting, puncturing)
  - Module `ContradictionDetector` pour la détection de contradictions (test hypothétique, blocs impossibles)
  - Module `BacktrackingSolver` pour le backtracking intelligent avec heuristiques
  - Module `AdvancedSolver` pour orchestrer toutes les techniques
- 🛣️ **Roadmap détaillée** avec plan d'implémentation sur 4 phases (12-15 semaines)
- 📊 **Documentation des algorithmes** avec exemples et pseudocode
- 📖 **Références académiques** et ressources en ligne
- 👥 **Guide de contribution** pour implémenter les techniques

### Modifié
- 📝 README mis à jour avec la roadmap détaillée et les nouveautés v0.3.0
- 🏛️ Structure du projet préparée pour l'ajout de modules avancés
- 📈 Documentation des performances et limitations actuelles

### Notes
- **Taux de résolution actuel**: ~70% des grilles (line solving uniquement)
- **Taux de résolution prévu**: 95%+ avec techniques avancées (v0.4.0+)
- Les modules avancés sont **documentés et conçus** mais pas encore implémentés
- Cette version pose les **fondations architecturales** pour les améliorations futures
- Compilation et tests réussis, application stable

## [0.2.0] - 2025-11-23

### Ajouté
- ✨ Détection automatique améliorée avec analyse de contours Canny
- 🖼️ Support étendu de formats d'image (JPEG, BMP, GIF, TIFF, WebP)
- 🚀 Solveur optimisé avec cache intelligent et élagage précoce
- 🔍 Support OCR pour extraction automatique des contraintes (feature optionnelle)
- 📊 Module `GridDetector` pour analyse avancée de grille
- 🧪 Tests unitaires pour les nouveaux modules

### Modifié
- ⚡ Amélioration des performances de 2-3x sur les grilles complexes
- 📝 Documentation complète mise à jour avec les nouvelles fonctionnalités
- 🎯 Détection automatique maintenant activée par défaut

### Optimisé
- 🔄 Cache des configurations valides pour éviter les recalculs
- ✂️ Élagage précoce dans la génération de configurations
- 📈 Comptage optimisé pour les déductions

## [0.1.0] - 2025-11-23

### Ajouté
- 🎉 Version initiale de l'application
- 🧩 Solveur de nonogramme par déduction logique
- 📸 Parseur d'image pour extraire l'état de la grille
- 🎨 Générateur d'image avec marquage des déductions en rouge
- 💻 Interface CLI avec clap
- 📚 Documentation complète (README.md, GUIDE_UTILISATION.md)
- 🧪 Tests unitaires de base
- 📦 Exemples de grilles 5x5

### Fonctionnalités principales
- Support de grilles 5x5 à 30x30
- Algorithme de line solving
- Détection automatique basique des paramètres
- Support PNG et JPEG
- Mode verbeux pour le débogage

[0.2.0]: https://github.com/votre-username/nonogram-solver/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/votre-username/nonogram-solver/releases/tag/v0.1.0
