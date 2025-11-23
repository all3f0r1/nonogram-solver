# Techniques de résolution avancées - Nonogram Solver

Ce document décrit l'architecture et la conception des techniques de résolution avancées pour le nonogram-solver. Ces techniques sont conçues pour résoudre les grilles difficiles qui ne peuvent pas être résolues par simple line solving.

## 📋 Table des matières

1. [Vue d'ensemble](#vue-densemble)
2. [Architecture modulaire](#architecture-modulaire)
3. [Techniques implémentables](#techniques-implémentables)
4. [Plan d'implémentation](#plan-dimplémentation)
5. [Références](#références)

---

## Vue d'ensemble

Le solveur actuel (v0.2.0) utilise uniquement la technique de **line solving** : il analyse chaque ligne et colonne indépendamment pour déduire les cases qui peuvent être remplies ou barrées. Cette approche fonctionne bien pour les grilles simples à moyennes, mais atteint ses limites sur les grilles difficiles.

### Limitations du line solving simple

- **Taux de résolution** : ~60-70% des grilles peuvent être complètement résolues
- **Grilles difficiles** : Nécessitent des techniques plus avancées ou du backtracking
- **Performance** : Peut stagner sur certaines configurations

### Objectif des techniques avancées

- **Augmenter le taux de résolution** à 95%+
- **Réduire le besoin de backtracking** (coûteux en calcul)
- **Maintenir la déduction pure** (pas de devinette)

---

## Architecture modulaire

L'architecture proposée sépare les techniques en modules indépendants qui peuvent être combinés :

```
src/solver/
├── mod.rs                          # Solveur de base (line solving)
├── line_solver.rs                  # Algorithme de line solving
├── line_solver_optimized.rs        # Version optimisée avec cache
├── cross_analysis.rs               # Analyse de contraintes croisées
├── backtracking.rs                 # Backtracking intelligent
├── contradiction_detector.rs       # Détection de contradictions
├── advanced_heuristics.rs          # Heuristiques avancées
└── advanced_solver.rs              # Orchestrateur combinant toutes les techniques
```

### Flux de résolution proposé

```
┌─────────────────────────────────────────────────────────────┐
│                    AdvancedSolver                            │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │   Phase 1: Line Solving (base)        │
        │   - Analyse ligne par ligne           │
        │   - Analyse colonne par colonne       │
        └───────────────────────────────────────┘
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │   Phase 2: Heuristiques avancées      │
        │   - Glue method                       │
        │   - Mercury method                    │
        │   - Joining/Splitting                 │
        │   - Puncturing                        │
        └───────────────────────────────────────┘
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │   Phase 3: Analyse croisée            │
        │   - Overlap analysis                  │
        │   - Edge forcing                      │
        │   - Contraintes bidirectionnelles     │
        └───────────────────────────────────────┘
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │   Phase 4: Détection contradictions   │
        │   - Test hypothétique                 │
        │   - Validation de cohérence           │
        │   - Blocs impossibles                 │
        └───────────────────────────────────────┘
                            │
                            ▼
        ┌───────────────────────────────────────┐
        │   Phase 5: Backtracking (dernier      │
        │   recours)                             │
        │   - Recherche avec limite de profondeur│
        │   - Heuristiques de choix de case     │
        └───────────────────────────────────────┘
```

---

## Techniques implémentables

### 1. Analyse de contraintes croisées (Cross Analysis)

**Principe** : Utiliser les informations des lignes ET colonnes simultanément.

#### 1.1 Overlap Analysis

Trouve les cases qui doivent être remplies car toutes les configurations possibles les incluent.

**Algorithme** :
```
Pour chaque bloc de contrainte:
    min_pos = position minimale possible du bloc
    max_pos = position maximale possible du bloc
    
    Si max_pos < min_pos + taille_bloc:
        # Il y a chevauchement
        overlap_start = max_pos
        overlap_end = min_pos + taille_bloc
        
        Remplir toutes les cases de overlap_start à overlap_end
```

**Exemple** :
```
Contrainte: [5]
Longueur: 7
Min pos: 0 (bloc peut commencer à 0)
Max pos: 2 (bloc doit finir avant 7)

Chevauchement: positions 2, 3, 4 doivent être remplies
```

#### 1.2 Edge Forcing

Force les cases aux bords basé sur les contraintes.

**Algorithme** :
```
Si une case remplie est proche du bord:
    distance_au_bord = position de la case
    taille_premier_bloc = première contrainte
    
    Si distance_au_bord < taille_premier_bloc:
        # Le premier bloc doit inclure cette case
        Remplir les cases nécessaires pour compléter le bloc
```

**Exemple** :
```
Contrainte: [3]
Ligne: [_, _, X, _, _, _, _]  (X = case remplie)

La case X est à position 2
Le bloc de 3 doit commencer au plus tard à position 0
Donc: remplir positions 0, 1, 2
```

### 2. Heuristiques avancées (Advanced Heuristics)

#### 2.1 Glue Method

Colle les blocs qui doivent être connectés.

**Principe** : Si un bloc partiellement rempli est proche de la taille de contrainte, étendre le bloc.

**Algorithme** :
```
Pour chaque bloc de cases remplies:
    taille_actuelle = nombre de cases remplies
    taille_contrainte = contrainte correspondante
    
    Si taille_actuelle > taille_contrainte / 2:
        # Le bloc est assez grand pour être "collé"
        cases_manquantes = taille_contrainte - taille_actuelle
        Remplir les cases adjacentes pour atteindre taille_contrainte
```

#### 2.2 Mercury Method

Simule le "coulage" des blocs comme du mercure.

**Principe** : Calculer où les blocs peuvent "couler" en fonction de l'espace disponible.

**Algorithme** :
```
total_cases_necessaires = somme(contraintes) + (nombre_contraintes - 1)
espace_libre = longueur_ligne - total_cases_necessaires

Pour chaque contrainte:
    min_pos = position minimale
    max_pos = longueur - taille_bloc - espace_pour_blocs_suivants
    
    Si max_pos - min_pos < taille_bloc:
        # Chevauchement garanti
        Remplir la zone de chevauchement
```

#### 2.3 Joining and Splitting

Joint ou sépare les blocs selon les contraintes.

**Principe** : Si on a plus de blocs que de contraintes, il faut joindre. Si un bloc est trop grand, il faut le séparer.

**Algorithme** :
```
blocs_actuels = compter les blocs de cases remplies
nombre_contraintes = nombre de contraintes

Si blocs_actuels > nombre_contraintes:
    # Il faut joindre des blocs
    Pour chaque paire de blocs adjacents:
        Si distance_entre_blocs == 1:
            Remplir la case entre les deux blocs
```

#### 2.4 Puncturing

Identifie les cases qui doivent être barrées.

**Principe** : Si tous les blocs sont placés correctement, barrer le reste.

**Algorithme** :
```
Si nombre_blocs_remplis == nombre_contraintes:
    Pour chaque bloc:
        Si taille_bloc == contrainte_correspondante:
            # Tous les blocs sont corrects
            Barrer toutes les cases vides restantes
```

### 3. Détection de contradictions (Contradiction Detection)

#### 3.1 Test hypothétique

Teste si placer un état crée une contradiction.

**Algorithme** :
```
Pour chaque case vide:
    # Tester CellState::Filled
    grille_test = copie de la grille
    grille_test.set(case, Filled)
    
    Si grille_test crée une contradiction:
        # Cette case ne peut pas être remplie
        Déduire: case = Crossed
    
    # Tester CellState::Crossed
    grille_test = copie de la grille
    grille_test.set(case, Crossed)
    
    Si grille_test crée une contradiction:
        # Cette case ne peut pas être barrée
        Déduire: case = Filled
```

**Détection de contradiction** :
- Ligne/colonne n'a plus de configuration valide
- Nombre de cases remplies > somme des contraintes
- Blocs séparés alors qu'ils devraient être joints

#### 3.2 Blocs impossibles

Identifie les segments trop petits pour contenir un bloc.

**Algorithme** :
```
Pour chaque segment de cases vides (séparé par des X):
    taille_segment = nombre de cases dans le segment
    taille_min_bloc = plus petit bloc de contrainte
    
    Si taille_segment < taille_min_bloc:
        # Aucun bloc ne peut tenir ici
        Barrer toutes les cases du segment
```

### 4. Backtracking intelligent

#### 4.1 Heuristique de choix de case

Choisir intelligemment quelle case essayer en premier.

**Score de case** :
```
score = f(contraintes, position, voisins)

Où:
- contraintes: somme des contraintes de la ligne + colonne
- position: distance aux bords (préférer les bords)
- voisins: nombre de cases remplies adjacentes

Meilleur score = plus de contraintes + plus de voisins + plus près des bords
```

#### 4.2 Élagage précoce (Early Pruning)

Arrêter rapidement les branches impossibles.

**Vérifications** :
1. **Validation rapide** : Vérifier que le nombre de blocs ne dépasse pas les contraintes
2. **Espace disponible** : Vérifier qu'il reste assez d'espace pour les blocs restants
3. **Cohérence locale** : Vérifier les lignes/colonnes adjacentes

#### 4.3 Cache des états visités

Éviter de revisiter les mêmes configurations.

**Structure** :
```rust
HashMap<Vec<CellState>, Vec<Vec<CellState>>>
```

**Clé** : État de la ligne + contraintes
**Valeur** : Configurations valides calculées

---

## Plan d'implémentation

### Phase 1 : Fondations (v0.3.0)

**Objectif** : Infrastructure pour techniques avancées

1. **Refactoring de Grid**
   - Ajouter méthodes `get_row()` et `get_column()` retournant `Vec<CellState>`
   - Ajouter méthode `count_empty_cells()` pour suivre la progression
   - Ajouter méthode `is_valid()` pour validation rapide

2. **Module CrossAnalyzer**
   - Implémenter overlap_analysis()
   - Implémenter edge_forcing()
   - Tests unitaires pour chaque technique

3. **Module AdvancedHeuristics**
   - Implémenter glue_method()
   - Implémenter mercury_method()
   - Tests avec grilles connues

**Estimation** : 2-3 semaines

### Phase 2 : Détection avancée (v0.4.0)

**Objectif** : Détecter les contradictions et optimiser

1. **Module ContradictionDetector**
   - Implémenter test hypothétique
   - Implémenter détection de blocs impossibles
   - Optimiser avec cache

2. **Optimisations**
   - Parallélisation avec Rayon (lignes/colonnes en parallèle)
   - SIMD pour comparaisons de vecteurs
   - Profiling et optimisation des hotspots

**Estimation** : 3-4 semaines

### Phase 3 : Backtracking (v0.5.0)

**Objectif** : Résoudre les grilles les plus difficiles

1. **Module BacktrackingSolver**
   - Implémenter heuristique de choix de case
   - Implémenter élagage précoce
   - Limiter la profondeur de recherche

2. **Module AdvancedSolver**
   - Orchestrer toutes les techniques
   - Configuration flexible (activer/désactiver techniques)
   - Mode verbeux pour debugging

**Estimation** : 4-5 semaines

### Phase 4 : Tests et validation (v0.6.0)

**Objectif** : Valider et benchmarker

1. **Suite de tests complète**
   - Grilles de référence (webpbn.com)
   - Grilles de différentes difficultés
   - Tests de régression

2. **Benchmarks**
   - Mesurer le taux de résolution
   - Mesurer les performances
   - Comparer avec d'autres solveurs

3. **Documentation**
   - Tutoriels d'utilisation
   - Documentation des algorithmes
   - Exemples de code

**Estimation** : 2-3 semaines

---

## Références

### Articles académiques

1. **"Solving Nonograms by Combining Relaxations"**
   - Auteurs: K-J. Batenburg, W. Palenstijn
   - Année: 2009
   - Lien: https://www.sciencedirect.com/science/article/pii/S0031320309001046

2. **"An Efficient Approach to Solving Nonograms"**
   - Auteurs: Nobuhisa Ueda, Tadaaki Nagao
   - Année: 1996

### Ressources en ligne

1. **WebPBN - Solving Techniques**
   - URL: https://webpbn.com/solving.html
   - Description: Guide complet des techniques de résolution

2. **Nonogram Solver Algorithms**
   - URL: https://github.com/topics/nonogram-solver
   - Description: Implémentations de référence sur GitHub

3. **Griddlers - Strategy Guide**
   - URL: https://www.griddlers.net/
   - Description: Tutoriels interactifs

### Implémentations de référence

1. **pbnsolve** (C++)
   - Auteur: Jan Wolter
   - URL: https://webpbn.com/pbnsolve.html
   - Techniques: Line solving, probing, contradictions

2. **nonogram-rs** (Rust)
   - URL: https://github.com/tsoding/nonogram-rs
   - Techniques: Backtracking simple

3. **nonogram-solver** (Python)
   - URL: https://github.com/mikix/nonogram-solver
   - Techniques: Constraint propagation

---

## Contribution

Si vous souhaitez contribuer à l'implémentation de ces techniques :

1. **Choisir une technique** dans le plan d'implémentation
2. **Créer une branche** : `git checkout -b feature/technique-name`
3. **Implémenter avec tests** : Suivre l'architecture modulaire
4. **Documenter** : Ajouter des commentaires et exemples
5. **Soumettre une PR** : Avec description détaillée

### Guidelines de contribution

- **Tests obligatoires** : Chaque technique doit avoir des tests unitaires
- **Documentation** : Documenter l'algorithme et les cas d'usage
- **Performance** : Benchmarker avant/après
- **Qualité** : Passer `cargo clippy` sans warnings

---

## Conclusion

Les techniques avancées proposées permettront d'augmenter significativement le taux de résolution du nonogram-solver, passant de ~70% à 95%+ des grilles. L'architecture modulaire facilite l'implémentation progressive et la maintenance.

**Prochaine étape** : Implémenter CrossAnalyzer et AdvancedHeuristics (Phase 1).

---

*Document créé le 23 novembre 2025*  
*Version: 1.0*  
*Auteur: Nonogram Solver Team*
