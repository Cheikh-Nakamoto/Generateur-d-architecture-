# Documentation de la fonction `create_project_structure`

## Description

La fonction `create_project_structure` permet de générer une structure de dossiers et de fichiers à partir du contenu d'un fichier `README.md`. Elle analyse le fichier à la recherche de blocs de texte décrivant une arborescence et crée la structure correspondante sur le disque.

## Utilisation

```sh
./programme <chemin_readme> <dossier_sortie>
```

### Arguments

- `readme_path` : Chemin vers le fichier `README.md` à analyser.
- `output_dir` : Dossier où la structure de projet sera créée.

### Valeur de retour

Retourne `Ok(())` si la structure est correctement générée, sinon une erreur (`Err`) en cas de problème d'accès aux fichiers ou de lecture.

## Détails des étapes

1. **Lecture du fichier `README.md`**
   - Charge le contenu du fichier spécifié dans une chaîne de caractères.
2. **Extraction de la structure des dossiers et fichiers**
   - Utilise des expressions régulières pour détecter les blocs contenant une arborescence de projet.
   - Filtre et normalise les chemins détectés.
3. **Création de l'arborescence**
   - Vérifie si chaque élément est un dossier ou un fichier.
   - Crée les dossiers nécessaires.
   - Crée les fichiers dans les bons emplacements.

## Cas d'utilisation

### 1. Génération automatique d'un projet à partir d'un `README.md`

**Exemple de contenu `README.md`** :

````
```plaintext
src/
    main.rs
    lib.rs
Cargo.toml
README.md
```
````

**Commande d'exécution** :

```sh
./programme README.md projet
```

**Résultat attendu** :

```
projet/
├── src/
│   ├── main.rs
│   ├── lib.rs
├── Cargo.toml
├── README.md
```

### 2. Traitement d'une arborescence plus complexe

**Exemple de contenu `README.md`** :

````
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
````

**Commande d'exécution** :

```sh
./programme README.md mon_projet
```

**Résultat attendu** :

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

## Fonctionnalités spécifiques

- **Détection intelligente des fichiers et dossiers** :
  - Un élément est considéré comme un dossier s'il se termine par `/` ou s'il ne contient pas de `.` (point).
- **Gestion des indentations** :
  - Permet de reconstruire l'arborescence même avec une indentation variable.
- **Suppression des doublons** :
  - Trie et supprime les éléments en double pour éviter des erreurs.

## Limitations et améliorations possibles

- Actuellement, les liens symboliques ne sont pas pris en compte.
- Le format du bloc de texte (ex: `plaintext`, `bash`, `sh`) doit être explicite.
- Peut être amélioré pour mieux gérer les fichiers avec des espaces dans leurs noms.

## Conclusion

Cette fonction est idéale pour automatiser la création de structures de projets à partir de spécifications contenues dans un `README.md`. Elle s'intègre parfaitement dans des flux de travail DevOps ou pour générer rapidement des projets selon des standards préétablis.

