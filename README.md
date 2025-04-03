
# 🌳 Project Tree Generator [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

![Project Logo](./src/assets/ferris.png)

**Transformez vos schémas d'arborescence en vrais dossiers et fichiers en un clic !**

## 📦 Binaires pré-compilés
Téléchargez la dernière version : [Releases](https://github.com/Cheikh-Nakamoto/Generateur-d-architecture-/releases/latest)

## 🎯 Fonctionnalités

- Conversion instantanée d'arborescences textuelles en structure réelle
- Prise en charge des formats Markdown et texte brut
- Gestion des hiérarchies complexes
- Prévisualisation avant génération
- Configuration avancée via YAML

## 🚀 Installation Rapide

### Via binaires pré-compilés
1. Téléchargez la version pour votre OS depuis [les Releases](https://github.com/Cheikh-Nakamoto/Generateur-d-architecture-/releases/latest)
2. Décompressez l'archive
3. Exécutez le binaire

### Via Cargo
```bash
cargo install project-tree-generator
```

## 🖥 Comment Utiliser

1. **Copiez** une arborescence depuis votre README.md
2. **Lancez** l'application
3. **Collez** la structure ou sélectionnez un fichier
4. **Choisissez** le dossier de destination
5. **Génerez** la structure

Exemple :
```plaintext
mon_projet/
├── src/
│   └── main.rs
└── Cargo.toml
```

## 📚 Cas d'Usage

- 🏗 Initialisation rapide de projets
- 📚 Reproduction de structures à partir de tutoriels
- 🌀 Clonage d'architectures open-source
- 🎓 Enseignement (démonstrations pour étudiants)
- 📝 Documentation vivante

## 🔍 Fonctionnement Technique

Le programme analyse :
- Les symboles `├──`, `│`, `└──` pour la hiérarchie
- L'indentation (espaces/tabulations)
- Les extensions de fichiers pour le type de contenu
- Les blocs de configuration YAML

## 📌 Fonctionnalités Spéciales

- Préserve les espaces dans les noms
- Gère les caractères spéciaux (_, -, @, etc.)
- Avertissements pour les conflits de noms
- Génération de fichiers templates
- Mode verbose pour le débogage

## 🌱 Contribuer

Les contributions sont les bienvenues ! 
1. Forkez le projet
2. Créez une branche (`git checkout -b feature/AmazingFeature`)
3. Committez vos changements (`git commit -m 'Add some AmazingFeature'`)
4. Pushez (`git push origin feature/AmazingFeature`)
5. Ouvrez une Pull Request

## 📜 Licence

MIT License - [Lire la licence complète](LICENSE)

```
Copyright 2023 VotreNom

Permission est accordée, gratuitement, à toute personne obtenant une copie
de ce logiciel et des fichiers de documentation associés (le "Logiciel"), 
de traiter dans le Logiciel sans restriction, y compris sans limitation 
les droits d'utiliser, copier, modifier, fusionner, publier, distribuer, 
sous-licencier et/ou vendre des copies du Logiciel, et d'autoriser les 
personnes à qui le Logiciel est fourni à le faire, sous réserve des 
conditions suivantes :

L'avis de copyright ci-dessus et cet avis d'autorisation doivent être 
inclus dans toutes copies ou parties substantielles du Logiciel.

LE LOGICIEL EST FOURNI "TEL QUEL", SANS GARANTIE D'AUCUNE SORTE, 
EXPRESSE OU IMPLICITE, Y COMPRIS MAIS SANS S'Y LIMITER LES GARANTIES 
DE QUALITÉ MARCHANDE, D'ADÉQUATION À UN USAGE PARTICULIER ET DE NON 
CONTREFAÇON. EN AUCUN CAS LES AUTEURS OU TITULAIRES DU COPYRIGHT NE 
POURRONT ÊTRE TENUS POUR RESPONSABLES DE TOUT DOMMAGE, RÉCLAMATION OU 
AUTRE RESPONSABILITÉ, QUE CE SOIT DANS LE CADRE D'UN CONTRAT, D'UN DÉLIT 
OU AUTRE, DÉCOULANT DE, EN LIEN AVEC OU EN RAPPORT AVEC LE LOGICIEL OU 
SON UTILISATION, OU D'AUTRES OPÉRATIONS EFFECTUÉES AVEC LE LOGICIEL.
```

## 💌 Contact

Pour toute question : [votre@email.com](mailto:votre@email.com)

---

*"Parfois les outils les plus utiles naissent des frustrations les plus simples"*
