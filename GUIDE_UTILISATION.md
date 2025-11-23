# Guide d'utilisation rapide - Solveur de Nonogramme

## 🎯 Objectif

Cette application vous aide lorsque vous êtes bloqué sur une grille de logimage (nonogramme/hanjie). Elle identifie les cases qui peuvent être complétées par simple déduction logique, sans avoir à deviner.

## 📥 Préparation de vos fichiers

### 1. Image de votre grille

Prenez une photo ou une capture d'écran de votre grille de nonogramme. L'image doit montrer:
- La grille avec ses cases
- Les cases déjà remplies (noires) ou barrées
- Les contraintes numériques (en haut et à gauche)

### 2. Fichier de contraintes

Créez un fichier JSON avec les contraintes de votre grille. Par exemple, pour une grille 5x5:

```json
{
  "width": 5,
  "height": 5,
  "rows": [
    [2],
    [1, 1],
    [5],
    [1, 1],
    [2]
  ],
  "columns": [
    [2],
    [1, 1],
    [5],
    [1, 1],
    [2]
  ]
}
```

**Explication**:
- `width` et `height`: Dimensions de la grille
- `rows`: Une liste de contraintes pour chaque ligne (de haut en bas)
- `columns`: Une liste de contraintes pour chaque colonne (de gauche à droite)
- Chaque contrainte est une liste de nombres indiquant les blocs de cases noires consécutives

## 🚀 Utilisation

### Commande de base

```bash
./target/release/nonogram-solver \
  --input ma_grille.png \
  --constraints ma_grille.json \
  --output solution.png \
  --verbose
```

### Avec configuration manuelle

Si la détection automatique ne fonctionne pas bien, spécifiez les paramètres:

```bash
./target/release/nonogram-solver \
  --input ma_grille.png \
  --constraints ma_grille.json \
  --output solution.png \
  --cell-size 30 \
  --margin-left 80 \
  --margin-top 80 \
  --verbose
```

**Comment trouver ces valeurs?**
- `cell-size`: Mesurez la largeur d'une case en pixels dans votre image
- `margin-left` et `margin-top`: Mesurez la distance en pixels depuis le bord de l'image jusqu'au début de la grille

## 📤 Résultat

L'application génère une nouvelle image (`solution.png`) qui est identique à votre image d'entrée, mais avec:
- **Cercles rouges**: Sur les cases qui devraient être noires
- **Croix rouges**: Sur les cases qui devraient être barrées

Ces marquages vous indiquent ce que vous pouvez remplir avec certitude, sans deviner!

## 💡 Conseils

1. **Qualité de l'image**: Plus votre image est nette et claire, meilleure sera la détection
2. **Grille régulière**: Les cases doivent être de taille uniforme
3. **Contraste**: Assurez-vous que les lignes de la grille et les cases remplies sont bien visibles
4. **Mode verbeux**: Utilisez `--verbose` pour voir les détails du traitement

## ❓ Que faire si aucune déduction n'est trouvée?

Si l'application indique "Aucune nouvelle déduction possible", cela signifie:
- Soit votre grille est complète
- Soit elle nécessite des techniques de résolution plus avancées (hypothèses, essais-erreurs)
- Soit il y a une erreur dans les contraintes ou l'état actuel de la grille

## 🔍 Exemple complet

Imaginons que vous avez une grille 5x5 bloquée:

1. **Créez le fichier de contraintes** `ma_grille.json`:
```json
{
  "width": 5,
  "height": 5,
  "rows": [[2], [1, 1], [5], [1, 1], [2]],
  "columns": [[2], [1, 1], [5], [1, 1], [2]]
}
```

2. **Prenez une photo** de votre grille et sauvegardez-la comme `ma_grille.png`

3. **Exécutez l'application**:
```bash
./target/release/nonogram-solver \
  --input ma_grille.png \
  --constraints ma_grille.json \
  --output solution.png \
  --verbose
```

4. **Ouvrez `solution.png`** pour voir les cases que vous pouvez remplir!

## 🆘 Dépannage

### "Erreur lors du chargement de l'image"
- Vérifiez que le chemin vers l'image est correct
- Assurez-vous que l'image est au format PNG ou JPG

### "Erreur lors du chargement des contraintes"
- Vérifiez la syntaxe JSON (virgules, crochets, accolades)
- Assurez-vous que le nombre de contraintes correspond aux dimensions

### "Aucune configuration valide trouvée"
- Vérifiez que les contraintes correspondent bien à la grille
- Vérifiez que l'état actuel de la grille (cases remplies) est compatible avec les contraintes
- Il peut y avoir une erreur dans votre grille actuelle

### "Position hors de l'image"
- Ajustez les paramètres `--cell-size`, `--margin-left`, `--margin-top`
- Mesurez précisément ces valeurs dans votre image avec un éditeur d'image

## 📞 Support

Pour toute question ou problème, consultez le fichier README.md ou ouvrez une issue sur le dépôt du projet.
