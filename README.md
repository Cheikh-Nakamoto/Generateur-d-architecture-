# 🌳 Project Tree Generator

![Project Logo](./src/assets/ferris.png)

**Transformez vos schémas d'arborescence en vrais dossiers et fichiers en un clic !**

## 🎯 Fonctionnalité Principale

Convertit les structures visuelles comme ceci :
```plaintext
your_project/
├── backend/
│   ├── models/
│   │   ├── user.py
│   │   └── post.py
│   └── routes/
│       └── api.py
└── frontend/
    ├── public/
    │   └── index.html
    └── src/
        └── App.jsx
```
En une véritable structure de fichiers !

## 🚀 Comment Utiliser

1. **Copier** une arborescence dans votre README.md
2. **Lancer** l'application
3. **Sélectionner** votre fichier et dossier de sortie
4. **Générer** la structure

## 💡 Cas d'Usage Parfaits

- 🏗 Initialisation rapide de projets
- 📚 Création de structures à partir de tutoriels
- 🌀 Réplication d'architectures open-source
- 🎓 Enseignement (montrer des structures aux étudiants)

## 🔍 Fonctionnement Technique

Le programme analyse :
- Les symboles `├──`, `│`, `└──` pour la hiérarchie
- L'indentation (espaces/tabulations)
- Les extensions de fichiers pour déterminer le type

## 📦 Exemple Complet

**Entrée (dans README.md) :**
```markdown
mon_projet/
├── docs/
│   └── api.md
├── src/
│   ├── utils/
│   │   └── helpers.js
│   └── index.js
└── package.json
```

**Sortie Générée :**
```
📁 mon_projet/
├── 📁 docs/
│   └── 📄 api.md
├── 📁 src/
│   ├── 📁 utils/
│   │   └── 📄 helpers.js
│   └── 📄 index.js
└── 📄 package.json
```

## ⚙️ Configuration Avancée

Ajoutez un bloc YAML pour personnaliser :
```yaml
structure-config:
  create_empty_files: true
  default_permissions: 755
  ignore_patterns:
    - "*.tmp"
```

## 📌 À Noter

- Préserve les espaces dans les noms de fichiers
- Gère les caractères spéciaux (_, -, @, etc.)
- Avertit en cas de conflits de noms

## 📚 Inspiration

Né d'une frustration personnelle après avoir dû recréer manuellement une structure complexe pour la énième fois !

---

*"Parfois les outils les plus utiles naissent des frustrations les plus simples"*