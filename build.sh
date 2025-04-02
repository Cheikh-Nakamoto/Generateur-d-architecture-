#!/bin/bash

# Script de build et d'installation pour le Générateur de Structure de Projet
# Ce script détecte le système d'exploitation et construit l'application en conséquence

set -e # Sortir en cas d'erreur

PROJECT_NAME="projet-structure-generator"
VERSION="1.0.0"
ICON_PATH="./src/assets/ferris.png"
DESKTOP_FILE="$PROJECT_NAME.desktop"

# Fonction pour afficher des messages stylisés
print_status() {
    echo -e "\e[1;34m[INFO]\e[0m $1"
}

print_success() {
    echo -e "\e[1;32m[SUCCESS]\e[0m $1"
}

print_error() {
    echo -e "\e[1;31m[ERROR]\e[0m $1"
    exit 1
}

# Vérifier si rust est installé
if ! command -v cargo &> /dev/null; then
    print_error "Rust n'est pas installé. Veuillez installer Rust avant de continuer: https://rustup.rs/"
fi

# Détecter le système d'exploitation
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    OS="linux"
    TARGET_DIR="$HOME/.local/bin"
    DESKTOP_DIR="$HOME/.local/share/applications"
    ICON_DIR="$HOME/.local/share/icons/hicolor/128x128/apps"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    OS="macos"
    TARGET_DIR="/Applications"
    # macOS utilise des .app bundles, nous les traiterons différemment
elif [[ "$OSTYPE" == "msys"* || "$OSTYPE" == "cygwin"* || "$OSTYPE" == "win32" ]]; then
    OS="windows"
    TARGET_DIR="$APPDATA/Programs/$PROJECT_NAME"
    DESKTOP_DIR="$APPDATA/Microsoft/Windows/Start Menu/Programs"
else
    print_error "Système d'exploitation non supporté: $OSTYPE"
fi

print_status "Système d'exploitation détecté: $OS"
print_status "Début de la compilation..."

# Compiler le programme en mode release
cargo build --release || print_error "Échec de la compilation"

print_success "Compilation terminée avec succès!"

# Créer les répertoires d'installation si nécessaire
create_directories() {
    if [[ "$OS" == "linux" ]]; then
        mkdir -p "$TARGET_DIR" "$DESKTOP_DIR" "$ICON_DIR"
    elif [[ "$OS" == "windows" ]]; then
        mkdir -p "$TARGET_DIR" "$DESKTOP_DIR"
    fi
}

# Installer l'application
install_application() {
    if [[ "$OS" == "linux" ]]; then
        # Copier l'exécutable
        cp "target/release/$PROJECT_NAME" "$TARGET_DIR/"
        chmod +x "$TARGET_DIR/$PROJECT_NAME"
        
        # Installer l'icône
        cp "$ICON_PATH" "$ICON_DIR/$PROJECT_NAME.png"
        
        # Créer un fichier desktop
        cat > "$DESKTOP_DIR/$DESKTOP_FILE" << EOF
[Desktop Entry]
Name=Générateur de Structure de Projet
Comment=Génère une structure de projet à partir d'un fichier README.md
Exec=$TARGET_DIR/$PROJECT_NAME
Icon=$PROJECT_NAME
Terminal=false
Type=Application
Categories=Development;Utility;
EOF
        
        print_status "Mise à jour de la base de données desktop..."
        update-desktop-database "$DESKTOP_DIR" 2>/dev/null || true

    elif [[ "$OS" == "macos" ]]; then
        # Créer un bundle macOS .app
        print_status "Création du bundle macOS..."
        
        # Utiliser cargo-bundle si disponible
        if command -v cargo-bundle &> /dev/null; then
            cargo bundle --release || print_error "Échec de la création du bundle"
            cp -r "target/release/bundle/osx/$PROJECT_NAME.app" "$TARGET_DIR/"
        else
            print_status "cargo-bundle n'est pas installé. Installation basique..."
            mkdir -p "$TARGET_DIR/$PROJECT_NAME.app/Contents/MacOS"
            mkdir -p "$TARGET_DIR/$PROJECT_NAME.app/Contents/Resources"
            
            # Copier l'exécutable
            cp "target/release/$PROJECT_NAME" "$TARGET_DIR/$PROJECT_NAME.app/Contents/MacOS/"
            
            # Copier l'icône
            cp "$ICON_PATH" "$TARGET_DIR/$PROJECT_NAME.app/Contents/Resources/"
            
            # Créer le fichier Info.plist
            cat > "$TARGET_DIR/$PROJECT_NAME.app/Contents/Info.plist" << EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>$PROJECT_NAME</string>
    <key>CFBundleIconFile</key>
    <string>ferris.png</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.$PROJECT_NAME</string>
    <key>CFBundleName</key>
    <string>Générateur de Structure de Projet</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>$VERSION</string>
</dict>
</plist>
EOF
        fi

    elif [[ "$OS" == "windows" ]]; then
        # Installer sur Windows
        print_status "Installation sur Windows..."
        
        # Copier l'exécutable
        cp "target/release/$PROJECT_NAME.exe" "$TARGET_DIR/"
        
        # Copier l'icône
        cp "$ICON_PATH" "$TARGET_DIR/"
        
        # Créer un raccourci
        if command -v powershell &> /dev/null; then
            powershell -Command "
                \$WshShell = New-Object -comObject WScript.Shell
                \$Shortcut = \$WshShell.CreateShortcut('$DESKTOP_DIR\\Générateur de Structure de Projet.lnk')
                \$Shortcut.TargetPath = '$TARGET_DIR\\$PROJECT_NAME.exe'
                \$Shortcut.IconLocation = '$TARGET_DIR\\ferris.png'
                \$Shortcut.Save()
            "
        else
            print_status "PowerShell n'est pas disponible. Le raccourci n'a pas été créé."
        fi
    fi
}

# Exécuter l'installation
create_directories
install_application

print_success "Installation terminée!"
print_status "Vous pouvez maintenant lancer l'application depuis votre menu d'applications ou en exécutant '$PROJECT_NAME'."

# Proposer de lancer l'application
read -p "Voulez-vous lancer l'application maintenant? (o/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Oo]$ ]]; then
    if [[ "$OS" == "linux" ]]; then
        "$TARGET_DIR/$PROJECT_NAME" &
    elif [[ "$OS" == "macos" ]]; then
        open "$TARGET_DIR/$PROJECT_NAME.app"
    elif [[ "$OS" == "windows" ]]; then
        start "$TARGET_DIR/$PROJECT_NAME.exe"
    fi
fi