# Audit des bibliothèques GUI Rust - Look-and-feel natif

**Date**: 23 novembre 2025  
**Objectif**: Trouver la meilleure bibliothèque GUI Rust offrant un look-and-feel natif sur Linux, Windows et Mac

---

## 🎯 Critères d'évaluation

1. **Look-and-feel natif** : Utilise les widgets natifs de chaque plateforme
2. **Support multi-plateforme** : Windows, Linux, macOS
3. **Accessibilité** : Support des lecteurs d'écran
4. **IME** : Support des méthodes de saisie (japonais, chinois, etc.)
5. **Maturité** : Stabilité, documentation, communauté
6. **100% Rust** : Pas de dépendances C/C++ si possible

---

## 📊 Résultats de l'audit

### Bibliothèques avec widgets natifs

#### 1. **WinSafe** ⭐⭐⭐⭐⭐
- **Type**: Bindings Win32 API (Windows uniquement)
- **Look-and-feel**: ✅ **100% natif Windows**
- **Accessibilité**: ✅ Oui (Windows Narrator)
- **IME**: ✅ Oui (support complet)
- **100% Rust**: ✅ Oui
- **Limitations**: ❌ **Windows uniquement**

**Verdict**: Parfait pour Windows, mais pas cross-platform.

---

#### 2. **FLTK-rs** ⭐⭐⭐⭐
- **Type**: Bindings FLTK (C++)
- **Look-and-feel**: ⚠️ Thèmes natifs disponibles mais pas par défaut
- **Accessibilité**: ✅ Oui (avec crate supplémentaire)
- **IME**: ✅ Oui (support complet)
- **100% Rust**: ❌ Non (dépendance C++)
- **Cross-platform**: ✅ Windows, Linux, macOS
- **Avantages**:
  - Léger et rapide
  - Thèmes natifs disponibles: https://github.com/fltk-rs/fltk-theme
  - Binaire statique possible
  - Bonne documentation

**Verdict**: Bon compromis, thèmes natifs disponibles mais nécessite configuration.

---

#### 3. **GTK 4 (gtk4-rs)** ⭐⭐⭐
- **Type**: Bindings GTK 4 (C)
- **Look-and-feel**: ⚠️ Natif sur Linux, émulé sur Windows/Mac
- **Accessibilité**: ❌ Non (selon l'audit 2025)
- **IME**: ✅ Oui
- **100% Rust**: ❌ Non (dépendance C)
- **Cross-platform**: ✅ Windows, Linux, macOS
- **Limitations**:
  - Pas d'accessibilité
  - Look non-natif sur Windows/Mac
  - Dépendances lourdes

**Verdict**: Bon pour Linux, moins bon pour Windows/Mac.

---

#### 4. **Relm4** ⭐⭐⭐
- **Type**: Framework au-dessus de GTK 4
- **Look-and-feel**: ⚠️ Comme GTK 4
- **Accessibilité**: ❌ Non
- **IME**: ✅ Oui
- **100% Rust**: ❌ Non (via GTK 4)
- **Avantages**:
  - API Rust idiomatique
  - Réactivité

**Verdict**: Même limitations que GTK 4.

---

### Bibliothèques avec rendu custom (pas de widgets natifs)

#### 5. **Slint** ⭐⭐⭐⭐⭐
- **Type**: Framework déclaratif avec DSL
- **Look-and-feel**: ⚠️ Rendu custom mais **peut imiter le natif**
- **Accessibilité**: ✅ Oui (lecteurs d'écran)
- **IME**: ✅ Oui (quelques glitches mais fonctionne)
- **100% Rust**: ✅ Oui (backend Rust disponible)
- **Cross-platform**: ✅ Windows, Linux, macOS, Embedded, Web
- **Avantages**:
  - Excellent tooling (LSP, live preview)
  - Styles natifs disponibles (Fluent, Material, Cupertino)
  - Performance excellente
  - Documentation complète
  - Entreprise-backed (SixtyFPS GmbH)

**Verdict**: **MEILLEUR CHOIX** pour look-and-feel "proche du natif" cross-platform.

---

#### 6. **Dioxus** ⭐⭐⭐⭐
- **Type**: Framework React-like
- **Look-and-feel**: ⚠️ Rendu custom (CSS)
- **Accessibilité**: ✅ Oui
- **IME**: ✅ Oui
- **100% Rust**: ✅ Oui
- **Cross-platform**: ✅ Windows, Linux, macOS, Web, Mobile
- **Avantages**:
  - Syntaxe familière (React)
  - Excellent pour le web
  - Hot reload

**Verdict**: Excellent mais look-and-feel web, pas natif.

---

#### 7. **egui** ⭐⭐⭐⭐
- **Type**: Immediate mode GUI
- **Look-and-feel**: ⚠️ Rendu custom (style propre)
- **Accessibilité**: ✅ Oui
- **IME**: ⚠️ Partiel (Tab volé par le converter)
- **100% Rust**: ✅ Oui
- **Cross-platform**: ✅ Windows, Linux, macOS, Web
- **Avantages**:
  - Très simple à utiliser
  - Pas de DSL/macros
  - Excellent pour outils/debug

**Verdict**: Excellent mais look-and-feel custom, pas natif.

---

#### 8. **Iced** ⭐⭐⭐
- **Type**: Framework Elm-like
- **Look-and-feel**: ⚠️ Rendu custom
- **Accessibilité**: ❌ Non (issue ouverte depuis 4.5 ans)
- **IME**: ❌ Non
- **100% Rust**: ✅ Oui
- **Cross-platform**: ✅ Windows, Linux, macOS, Web

**Verdict**: Prometteur mais manque d'accessibilité et IME.

---

### Bibliothèques Qt

#### 9. **CXX-Qt** ⭐⭐
- **Type**: Bindings Qt (C++)
- **Look-and-feel**: ✅ **100% natif** (Qt utilise widgets natifs)
- **Accessibilité**: ✅ Oui (Qt a excellent support)
- **IME**: ✅ Oui
- **100% Rust**: ❌ Non (dépendance Qt C++)
- **Cross-platform**: ✅ Windows, Linux, macOS
- **Limitations**:
  - Linker hell (selon l'audit 2025)
  - Nécessite compte Qt
  - Dépendances lourdes
  - Setup complexe

**Verdict**: Natif mais setup trop complexe et dépendances lourdes.

---

## 🏆 Recommandations

### Pour look-and-feel **vraiment natif**

**Aucune solution cross-platform parfaite en 100% Rust.**

Les options:
1. **FLTK-rs** avec thèmes natifs (compromis acceptable)
   - Dépendance C++ mais légère
   - Thèmes natifs disponibles
   - Bon support IME et accessibilité

2. **Approche hybride**:
   - WinSafe pour Windows
   - GTK 4 pour Linux
   - Cacao pour macOS
   - ❌ Mais 3 codebases différentes !

---

### Pour look-and-feel **proche du natif** (recommandé)

**Slint** ⭐⭐⭐⭐⭐

**Pourquoi**:
- ✅ Styles natifs intégrés (Fluent pour Windows, Material pour Linux, Cupertino pour Mac)
- ✅ 100% Rust (backend Rust disponible)
- ✅ Accessibilité complète
- ✅ IME fonctionnel
- ✅ Cross-platform (Windows, Linux, macOS, Web, Embedded)
- ✅ Excellent tooling (LSP, live preview)
- ✅ Performance excellente
- ✅ Documentation complète
- ✅ Entreprise-backed (maintenance garantie)
- ✅ Communauté active

**Limitations**:
- ⚠️ Utilise un DSL (`.slint` files)
- ⚠️ Pas de widgets natifs OS (rendu custom)

**Mais**: Le rendu est suffisamment bon pour être indiscernable du natif dans la plupart des cas.

---

### Alternative: **Dioxus** (si look web acceptable)

Si vous préférez une syntaxe React-like et que le look-and-feel web ne vous dérange pas.

---

## 📋 Tableau comparatif final

| Bibliothèque | Natif | Cross-platform | Accessibilité | IME | 100% Rust | Maturité |
|--------------|-------|----------------|---------------|-----|-----------|----------|
| **Slint** | ⚠️ Proche | ✅ | ✅ | ✅ | ✅ | ⭐⭐⭐⭐⭐ |
| **FLTK-rs** | ⚠️ Thèmes | ✅ | ✅ | ✅ | ❌ | ⭐⭐⭐⭐ |
| **Dioxus** | ❌ Web | ✅ | ✅ | ✅ | ✅ | ⭐⭐⭐⭐ |
| **egui** | ❌ Custom | ✅ | ✅ | ⚠️ | ✅ | ⭐⭐⭐⭐ |
| **GTK 4** | ⚠️ Linux | ✅ | ❌ | ✅ | ❌ | ⭐⭐⭐ |
| **WinSafe** | ✅ Windows | ❌ | ✅ | ✅ | ✅ | ⭐⭐⭐ |
| **CXX-Qt** | ✅ Qt | ✅ | ✅ | ✅ | ❌ | ⭐⭐ |
| **Iced** | ❌ Custom | ✅ | ❌ | ❌ | ✅ | ⭐⭐⭐ |

---

## 🎯 Décision finale

**Pour le projet nonogram-solver, je recommande: Slint**

**Raisons**:
1. ✅ Look-and-feel proche du natif sur toutes les plateformes
2. ✅ 100% Rust (objectif du projet)
3. ✅ Accessibilité complète
4. ✅ Cross-platform sans compromis
5. ✅ Excellent tooling et documentation
6. ✅ Performance excellente (important pour affichage d'images)
7. ✅ Maintenance garantie (entreprise-backed)

**Compromis accepté**:
- ⚠️ Utilise un DSL (mais bien intégré avec Rust)
- ⚠️ Rendu custom (mais indiscernable du natif)

---

## 📚 Références

- [2025 Survey of Rust GUI Libraries](https://www.boringcactus.com/2025/04/13/2025-survey-of-rust-gui-libraries.html)
- [Are We GUI Yet?](https://areweguiyet.com/)
- [Slint Documentation](https://slint.dev/)
- [FLTK-rs Themes](https://github.com/fltk-rs/fltk-theme)
- [WinSafe Documentation](https://github.com/rodrigocfd/winsafe)
