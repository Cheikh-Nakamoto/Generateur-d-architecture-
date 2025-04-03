# 🌳 Guide d'installation - Générateur de Structure de Projet
![Project Logo](./src/assets/ferris.png)

Ce document explique comment installer le Générateur de Structure de Projet sur différents systèmes d'exploitation.

## Prérequis

Avant de commencer l'installation, assurez-vous d'avoir installé les logiciels suivants sur votre système :

- **Git** (pour cloner le dépôt)
- **Rust et Cargo** (pour compiler le programme)

### Installation des prérequis

#### Git

- **Windows** : Téléchargez et installez Git depuis [git-scm.com](https://git-scm.com/download/win)
- **macOS** : Installez via Homebrew avec `brew install git` ou téléchargez depuis [git-scm.com](https://git-scm.com/download/mac)
- **Linux** : Utilisez votre gestionnaire de paquets, par exemple `sudo apt install git` (Ubuntu/Debian) ou `sudo dnf install git` (Fedora)

#### Rust et Cargo

Sur tous les systèmes d'exploitation, le moyen le plus simple d'installer Rust et Cargo est d'utiliser rustup :

1. Visitez [rustup.rs](https://rustup.rs/)
2. Suivez les instructions pour votre système d'exploitation

## Installation en deux étapes

### 1. Cloner le dépôt

Commencez par récupérer le code source :

```bash
git clone https://github.com/Cheikh-Nakamoto/Generateur-d-architecture-.git
cd Generateur-d-architecture
```

### 2. Exécuter le script d'installation

Une fois dans le répertoire du projet, utilisez le script d'installation automatique.

#### Rendre le script exécutable (Linux/macOS uniquement)

```bash
chmod +x build.sh
```

#### Exécuter le script

- **Linux/macOS** : Dans un terminal, exécutez `./build.sh`
- **Windows** : Dans Git Bash ou WSL, exécutez `./build.sh`

#### Suivez les instructions à l'écran

Le script va :
- Compiler l'application
- L'installer avec les icônes et raccourcis appropriés
- Vous proposer de lancer l'application immédiatement

## Ce que fait le script d'installation

Le script automatise les actions suivantes :

1. Vérifie la présence de Rust
2. Détecte votre système d'exploitation
3. Compile l'application en mode release
4. Installe l'application au bon endroit selon votre système d'exploitation :
   - **Linux** : Crée un fichier .desktop et installe l'icône
   - **macOS** : Crée un bundle .app avec Info.plist
   - **Windows** : Installe l'application et crée un raccourci dans le menu Démarrer

## Installation manuelle alternative

Si vous préférez installer manuellement, voici les étapes à suivre après avoir cloné le dépôt :

### Compiler l'application

```bash
cargo build --release
```

### Installer l'application

#### Sur Linux

```bash
# Créer le dossier de destination
mkdir -p ~/.local/bin
# Copier l'exécutable
cp target/release/projet-structure-generator ~/.local/bin/
# Le rendre exécutable
chmod +x ~/.local/bin/projet-structure-generator
```

Pour l'intégration au bureau (optionnel) :
```bash
# Copier l'icône
mkdir -p ~/.local/share/icons/hicolor/128x128/apps/
cp src/assets/ferris.png ~/.local/share/icons/hicolor/128x128/apps/projet-structure-generator.png

# Créer un fichier .desktop
cat > ~/.local/share/applications/projet-structure-generator.desktop << EOF
[Desktop Entry]
Name=Générateur de Structure de Projet
Comment=Génère une structure de projet à partir d'un fichier README.md
Exec=~/.local/bin/projet-structure-generator
Icon=projet-structure-generator
Terminal=false
Type=Application
Categories=Development;Utility;
EOF

# Mettre à jour la base de données des applications
update-desktop-database ~/.local/share/applications/
```

#### Sur macOS

```bash
# Créer la structure du bundle .app
mkdir -p ~/Applications/projet-structure-generator.app/Contents/MacOS
mkdir -p ~/Applications/projet-structure-generator.app/Contents/Resources

# Copier l'exécutable et l'icône
cp target/release/projet-structure-generator ~/Applications/projet-structure-generator.app/Contents/MacOS/
cp src/assets/ferris.png ~/Applications/projet-structure-generator.app/Contents/Resources/

# Créer le fichier Info.plist
cat > ~/Applications/projet-structure-generator.app/Contents/Info.plist << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>projet-structure-generator</string>
    <key>CFBundleIconFile</key>
    <string>ferris.png</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.projet-structure-generator</string>
    <key>CFBundleName</key>
    <string>Générateur de Structure de Projet</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF
```

#### Sur Windows

```bash
# Créer le dossier de destination
mkdir -p "$APPDATA\Programs\projet-structure-generator"

# Copier l'exécutable et l'icône
cp target/release/projet-structure-generator.exe "$APPDATA\Programs\projet-structure-generator"
cp src/assets/ferris.png "$APPDATA\Programs\projet-structure-generator"
```

Pour créer un raccourci (dans PowerShell) :
```powershell
$WshShell = New-Object -comObject WScript.Shell
$Shortcut = $WshShell.CreateShortcut("$env:APPDATA\Microsoft\Windows\Start Menu\Programs\Générateur de Structure de Projet.lnk")
$Shortcut.TargetPath = "$env:APPDATA\Programs\projet-structure-generator\projet-structure-generator.exe"
$Shortcut.IconLocation = "$env:APPDATA\Programs\projet-structure-generator\ferris.png"
$Shortcut.Save()
```

## Lancement de l'application

Une fois l'installation terminée, vous pouvez lancer l'application :

- **Linux** : Depuis le menu des applications ou en exécutant `projet-structure-generator` dans un terminal
- **macOS** : En ouvrant l'application depuis le dossier Applications ou en utilisant Spotlight
- **Windows** : Depuis le menu Démarrer ou en cliquant sur le raccourci créé

## Résolution de problèmes

### Erreur "Command not found"

Si vous obtenez une erreur "Command not found" lors de l'exécution du script ou de l'application :

- Vérifiez que le chemin d'installation est bien dans votre PATH
- Sur Linux/macOS, assurez-vous que le fichier est exécutable (`chmod +x`)

### Problèmes de compilation

Si la compilation échoue :

- Assurez-vous que Rust est correctement installé avec `rustc --version`
- Vérifiez que les dépendances sont disponibles avec `cargo check`
- Consultez les erreurs spécifiques dans la sortie du terminal

### Problèmes d'installation

Si l'installation échoue :

- Vérifiez que vous disposez des droits d'écriture dans les dossiers d'installation
- Sur Linux/macOS, essayez d'utiliser `sudo` pour les opérations nécessitant des privilèges

## Support et questions

Pour toute question ou assistance supplémentaire :

- Ouvrez une issue sur le dépôt GitHub : [https://github.com/Cheikh-Nakamoto/Generateur-d-architecture-/issues](https://github.com/Cheikh-Nakamoto/Generateur-d-architecture-/issues)