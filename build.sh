#!/bin/bash

# Script de build et d'installation pour le Générateur de Structure de Projet
# Version améliorée avec meilleure gestion des erreurs et support multi-plateforme

set -euo pipefail

# Configuration
PROJECT_NAME="projet-structure-generator"
VERSION="1.0.0"
ICON_PATH="./src/assets/ferris.png"
DESKTOP_FILE="$PROJECT_NAME.desktop"

# Couleurs pour les messages
RED='\033[1;31m'
GREEN='\033[1;32m'
BLUE='\033[1;34m'
NC='\033[0m'

# Fonctions d'affichage
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1" >&2
    exit 1
}

# Vérifier les dépendances
check_dependencies() {
    if ! command -v cargo &> /dev/null; then
        print_error "Rust n'est pas installé. Veuillez installer Rust via https://rustup.rs/"
    fi

    if [[ "$OSTYPE" == "darwin"* ]] && ! command -v create-dmg &> /dev/null; then
        print_status "Installez create-dmg pour créer un bundle macOS: brew install create-dmg"
    fi
}

# Détection du système d'exploitation
detect_os() {
    case "$OSTYPE" in
        linux-gnu*)  OS="linux" ;;
        darwin*)     OS="macos" ;;
        msys*|cygwin*|win32*) OS="windows" ;;
        *)           print_error "Système non supporté: $OSTYPE" ;;
    esac

    print_status "Système détecté: $OS"
}

# Compilation du projet
compile_project() {
    print_status "Compilation en cours..."
    if ! cargo build --release; then
        print_error "Échec de la compilation"
    fi
    print_success "Compilation réussie"
}

# Installation pour Linux
install_linux() {
    local target_dir="${HOME}/.local/bin"
    local desktop_dir="${HOME}/.local/share/applications"
    local icon_dir="${HOME}/.local/share/icons/hicolor/128x128/apps"

    mkdir -p "$target_dir" "$desktop_dir" "$icon_dir"

    # Copie des fichiers
    cp "target/release/${PROJECT_NAME}" "$target_dir/"
    chmod +x "$target_dir/${PROJECT_NAME}"
    cp "$ICON_PATH" "$icon_dir/${PROJECT_NAME}.png"

    # Création du fichier .desktop
    cat > "$desktop_dir/$DESKTOP_FILE" <<EOF
[Desktop Entry]
Version=${VERSION}
Name=Générateur de Structure de Projet
Comment=Génère une structure de projet à partir d'un fichier README.md
Exec=${target_dir}/${PROJECT_NAME}
Icon=${PROJECT_NAME}
Terminal=false
Type=Application
Categories=Development;Utility;
EOF

    update-desktop-database "$desktop_dir" || true
}

# Installation pour macOS
install_macos() {
    local app_dir="/Applications/${PROJECT_NAME}.app"
    local contents_dir="${app_dir}/Contents"
    local macos_dir="${contents_dir}/MacOS"
    local resources_dir="${contents_dir}/Resources"

    # Création de la structure .app
    rm -rf "$app_dir"
    mkdir -p "$macos_dir" "$resources_dir"

    # Copie des fichiers
    cp "target/release/${PROJECT_NAME}" "$macos_dir/"
    cp "$ICON_PATH" "$resources_dir/"

    # Création du fichier Info.plist
    cat > "$contents_dir/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${PROJECT_NAME}</string>
    <key>CFBundleIconFile</key>
    <string>ferris.png</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.${PROJECT_NAME}</string>
    <key>CFBundleName</key>
    <string>Générateur de Structure de Projet</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
</dict>
</plist>
EOF

    # Fix des permissions
    chmod -R 755 "$app_dir"
}

# Installation pour Windows
install_windows() {
    local target_dir="${APPDATA}\\Programs\\${PROJECT_NAME}"
    local start_menu_dir="${APPDATA}\\Microsoft\\Windows\\Start Menu\\Programs"

    mkdir -p "$target_dir" "$start_menu_dir"

    # Copie des fichiers
    cp "target/release/${PROJECT_NAME}.exe" "$target_dir/"
    cp "$ICON_PATH" "$target_dir/"

    # Création du raccourci
    if command -v powershell &> /dev/null; then
        powershell -Command "\
            \$ws = New-Object -ComObject WScript.Shell; \
            \$sc = \$ws.CreateShortcut('${start_menu_dir}\\${PROJECT_NAME}.lnk'); \
            \$sc.TargetPath = '${target_dir}\\${PROJECT_NAME}.exe'; \
            \$sc.IconLocation = '${target_dir}\\ferris.png'; \
            \$sc.Save()"
    fi
}

# Lancement de l'application
launch_application() {
    read -rp "Voulez-vous lancer l'application maintenant ? (o/n) " answer
    if [[ "$answer" =~ ^[OoYy] ]]; then
        case "$OS" in
            linux)   "${HOME}/.local/bin/${PROJECT_NAME}" & ;;
            macos)   open "/Applications/${PROJECT_NAME}.app" ;;
            windows) start "${APPDATA}\\Programs\\${PROJECT_NAME}\\${PROJECT_NAME}.exe" ;;
        esac
    fi
}

# Main
check_dependencies
detect_os
compile_project

case "$OS" in
    linux)   install_linux ;;
    macos)   install_macos ;;
    windows) install_windows ;;
esac

print_success "Installation terminée avec succès !"
launch_application