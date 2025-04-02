
<div align="center">
    <img src="./src/assets/ferris.png" alt="Logo du projet" width="300">
</div>

# Générateur de Structure de Projet

## Description

Cette application GUI permet de générer automatiquement une structure de dossiers et de fichiers à partir d'un fichier README.md. Elle analyse le fichier à la recherche de blocs de texte décrivant une arborescence et crée la structure correspondante dans le dossier choisi.

## Fonctionnalités

- Interface graphique simple et intuitive
- Sélection du fichier README.md par navigateur de fichiers
- Sélection du dossier de sortie par navigateur de dossiers
- Affichage des messages de succès ou d'erreur
- Thème sombre ergonomique avec codes couleur

## Utilisation

1. Lancez l'application
2. Cliquez sur "📄 Sélectionner README.md" pour choisir votre fichier README
3. Cliquez sur "📂 Sélectionner Dossier de Sortie" pour définir le dossier cible
4. Cliquez sur "🚀 Générer Structure" pour créer la structure de projet

## Format du README.md

L'application détecte les blocs de code markdown contenant une structure de projet. Par exemple :

```plaintext
src/
    main.rs
    lib.rs
Cargo.toml
README.md
```

### Règles de détection

- Un élément est considéré comme un dossier s'il se termine par `/` ou s'il ne contient pas de `.` (point)
- L'indentation permet de déterminer la hiérarchie des éléments
- Les formats de bloc supportés incluent `plaintext`, `bash`, `sh`

## Exemples

### Exemple 1: Structure simple

**Dans le README.md:**
```plaintext
src/
    main.rs
    lib.rs
Cargo.toml
README.md
```

**Résultat généré:**
```
projet/
├── src/
│   ├── main.rs
│   ├── lib.rs
├── Cargo.toml
├── README.md
```

### Exemple 2: Structure complexe

**Dans le README.md:**
```bash
backend/
    models/
        user.rs
        post.rs
    routes/
        api.rs
frontend/
    components/
        App.tsx
        Header.tsx
    styles/
        main.css
```

**Résultat généré:**
```
mon_projet/
├── backend/
│   ├── models/
│   │   ├── user.rs
│   │   ├── post.rs
│   ├── routes/
│   │   ├── api.rs
├── frontend/
│   ├── components/
│   │   ├── App.tsx
│   │   ├── Header.tsx
│   ├── styles/
│   │   ├── main.css
```

## Limitations

- Les liens symboliques ne sont pas pris en compte
- Le format du bloc de texte doit être explicite
- Les noms de fichiers contenant des espaces peuvent nécessiter une attention particulière

## Prérequis

- Compatible avec Windows, macOS et Linux
- Nécessite les droits d'écriture dans le dossier de destination